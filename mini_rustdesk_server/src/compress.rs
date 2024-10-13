use std::cell::RefCell;
use zstd::block::{Compressor, Decompressor};

thread_local! {
    static COMPRESSOR: RefCell<Compressor> = RefCell::new(Compressor::new());
    static DECOMPRESSOR: RefCell<Decompressor> = RefCell::new(Decompressor::new());
}


pub fn compress(data: &[u8], level: i32) -> Vec<u8> {
    let mut out = Vec::new();
    COMPRESSOR.with(|c| {
        if let Ok(mut c) = c.try_borrow_mut() {
            match c.compress(data, level) {
                Ok(res) => out = res,
                Err(err) => {
                    log::info!("Failed to compress: {}", err);
                }
            }
        }
    });
    out
}

pub fn decompress(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    DECOMPRESSOR.with(|d| {
        if let Ok(mut d) = d.try_borrow_mut() {
            const MAX: usize = 1024 * 1024 * 64;
            const MIN: usize = 1024 * 1024;
            let mut n = 30 * data.len();
            n = n.clamp(MIN, MAX);
            match d.decompress(data, n) {
                Ok(res) => out = res,
                Err(err) => {
                    log::info!("Failed to decompress: {}", err);
                }
            }
        }
    });
    out
}
