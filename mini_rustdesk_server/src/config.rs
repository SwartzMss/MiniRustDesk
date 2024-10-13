use std::{
    collections::HashMap,
    fs,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    path::{Path, PathBuf},
    sync::{Arc, Mutex, RwLock},
    time::SystemTime,
};

pub const RENDEZVOUS_PORT: i32 = 21116;
pub const COMPRESS_LEVEL: i32 = 3;


pub fn get_any_listen_addr(is_ipv4: bool) -> SocketAddr {
    if is_ipv4 {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), 0)
    } else {
        SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), 0)
    }
}