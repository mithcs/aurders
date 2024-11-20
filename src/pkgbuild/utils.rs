use crate::pkgbuild::user_input::get_checksum_type_input;

use std::fs;

use sha2::{Sha224, Sha256, Sha384, Sha512, Digest};

pub static mut SOURCES: Vec<String> = Vec::new();
pub static mut SOURCES_COUNT: u8 = 0;
pub static mut CHECKSUM_TYPE: String = String::new();

/// Returns the sources count
#[allow(dead_code)]
pub(in super) fn get_sources_count() -> u8 {
    unsafe {
        return SOURCES_COUNT;
    }
}

/// Returns the checksum type
pub(in super) fn get_checksum_type() -> String {
    get_checksum_type_input();
    unsafe {
        return CHECKSUM_TYPE.clone();
    }
}

/// Returns the checksum of bytes of given source
pub fn get_checksum(source: String) -> String {
    let bytes: Vec<u8>;
    if source.contains("http") {
        let response = minreq::get(source)
            .send().unwrap();

        bytes = response.as_bytes().to_vec();
    } else {
        bytes = fs::read(source).unwrap();
    }

    let hash: String;

    hash = unsafe {
        match CHECKSUM_TYPE.as_str() {
            "SHA256" => get_sha256_hash(&bytes),
            "SHA512" => get_sha512_hash(&bytes),
            "SHA384" => get_sha384_hash(&bytes),
            "SHA224" => get_sha224_hash(&bytes),
            _ => get_sha256_hash(&bytes),
        }
    };

    return hash;
}

/// Returns Sha256 hash of given bytes
fn get_sha256_hash(bytes: &[u8]) -> String {
    Sha256::digest(bytes);
    return format!("{:x}", Sha256::digest(bytes));
}

/// Returns Sha512 hash of given bytes
fn get_sha512_hash(bytes: &[u8]) -> String {
    Sha512::digest(bytes);
    return format!("{:x}", Sha512::digest(bytes));
}

/// Returns Sha384 hash of given bytes
fn get_sha384_hash(bytes: &[u8]) -> String {
    Sha384::digest(bytes);
    return format!("{:x}", Sha384::digest(bytes));
}

/// Returns Sha224 hash of given bytes
fn get_sha224_hash(bytes: &[u8]) -> String {
    Sha224::digest(bytes);
    return format!("{:x}", Sha224::digest(bytes));
}
