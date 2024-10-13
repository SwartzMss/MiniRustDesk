use crate::common::*;
use crate::database;
use bytes::Bytes;
use crate::rendezvous::*;
use tokio::sync::{Mutex, RwLock};
use crate::ResultType;
use serde_derive::{Deserialize, Serialize};
use std::{collections::HashMap, collections::HashSet, net::SocketAddr, sync::Arc, time::Instant};

type UserStatusMap = HashMap<Vec<u8>, Arc<(Option<Vec<u8>>, bool)>>;
type IpChangesMap = HashMap<String, (Instant, HashMap<String, i32>)>;
lazy_static::lazy_static! {
    pub(crate) static ref USER_STATUS: RwLock<UserStatusMap> = Default::default();
    pub(crate) static ref IP_CHANGES: Mutex<IpChangesMap> = Default::default();
}
pub static IP_CHANGE_DUR: u64 = 180;
pub static IP_CHANGE_DUR_X2: u64 = IP_CHANGE_DUR * 2;
pub static DAY_SECONDS: u64 = 3600 * 24;
pub static IP_BLOCK_DUR: u64 = 60;

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub(crate) struct PeerInfo {
    #[serde(default)]
    pub(crate) ip: String,
}

pub(crate) struct Peer {
    pub(crate) socket_addr: SocketAddr,
    pub(crate) last_reg_time: Instant,
    pub(crate) guid: Vec<u8>,
    pub(crate) uuid: Bytes,
    pub(crate) pk: Bytes,
    pub(crate) info: PeerInfo,
    pub(crate) reg_pk: (u32, Instant), // how often register_pk
}

impl Default for Peer {
    fn default() -> Self {
        Self {
            socket_addr: "0.0.0.0:0".parse().unwrap(),
            last_reg_time: get_expired_time(),
            guid: Vec::new(),
            uuid: Bytes::new(),
            pk: Bytes::new(),
            info: Default::default(),
            reg_pk: (0, get_expired_time()),
        }
    }
}

pub(crate) type LockPeer = Arc<RwLock<Peer>>;

#[derive(Clone)]
pub(crate) struct PeerMap {
    pub(crate) db: database::Database,
}

impl PeerMap {
    pub(crate) async fn new() -> ResultType<Self> {
        let exe_path_result = std::env::current_exe();
        let db_path = match exe_path_result {
            Ok(exe_path) => exe_path.with_file_name("db_v2.sqlite3"),
            Err(e) => {
                log::error!("Failed to get current executable path: {}", e);
                std::path::PathBuf::from("db_v2.sqlite3")
            }
        };

        let db_path_str = db_path.to_str().unwrap_or("db_v2.sqlite3");
        log::info!("DB Path: {}", db_path_str);

        let pm = Self {
            db: database::Database::new(db_path_str).await?,
        };
        Ok(pm)
    }

    pub(crate) async fn update_or_insert_peer(
        &self,
        id: String,
        uuid: Bytes,
        pk: Bytes,
        ip: String,
    ) -> register_pk_response::Result {
        let peer = self.db.get_peer_by_id(&id).await;
        match peer {
            Ok(Some(existing_peer)) => {
                log::info!("Peer exists, updating...");
                let info_str = serde_json::to_string(&existing_peer.info).unwrap_or_default();
                if let Err(err) = self.db.update_pk_by_guid(&existing_peer.guid, &id, &pk, &info_str).await {
                    log::error!("db.update_pk failed: {}", err);
                    register_pk_response::Result::SERVER_ERROR
                } else {
                    log::info!("Peer updated successfully.");
                    register_pk_response::Result::OK
                }
            },
            Ok(None) => {
                log::info!("Peer does not exist, inserting...");
                let info_str = serde_json::to_string(&PeerInfo { ip }).unwrap_or_default();
                match self.db.insert_peer(&id, &uuid, &pk, &info_str).await {
                    Ok(guid) => {
                        log::info!("Peer inserted successfully with GUID: {:?}", guid);
                        register_pk_response::Result::OK
                    },
                    Err(err) => {
                        log::error!("db.insert_peer failed: {}", err);
                        register_pk_response::Result::SERVER_ERROR
                    }
                }
            },
            Err(err) => {
                log::error!("Failed to get peer by ID: {}", err);
                register_pk_response::Result::SERVER_ERROR
            }
        }
    }

    pub(crate) async fn get_peer_by_id(&self, id: &str) -> Option<LockPeer> {
         if let Ok(Some(v)) = self.db.get_peer_by_id(id).await {
            let peer = Peer {
                guid: v.guid,
                uuid: v.uuid.into(),
                pk: v.pk.into(),
                info: serde_json::from_str::<PeerInfo>(&v.info).unwrap_or_default(),
                ..Default::default()
            };
            let peer = Arc::new(RwLock::new(peer));
            return Some(peer);
        }
        None
    }

    pub(crate) async fn get_peer_by_guid(&self, guid:&[u8]) -> Option<LockPeer> {
        if let Ok(Some(v)) = self.db.get_peer_by_guid(guid).await {
           let peer = Peer {
               guid: v.guid,
               uuid: v.uuid.into(),
               pk: v.pk.into(),
               info: serde_json::from_str::<PeerInfo>(&v.info).unwrap_or_default(),
               ..Default::default()
           };
           let peer = Arc::new(RwLock::new(peer));
           return Some(peer);
       }
       None
   }
}
