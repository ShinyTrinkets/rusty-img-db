use blake2::{Blake2b512, Blake2s256};
use crc32fast;
use digest::Digest;
use ripemd::{Ripemd128, Ripemd160};
use sha2::{Sha224, Sha256, Sha384, Sha512};
use std::fmt;
use std::hash::Hasher;
use strum_macros::EnumString;

/// Cryptographical hash
pub fn hash_c(h: &HashC, data: &[u8]) -> String {
    match h {
        HashC::Crc32 => crc_hex::<crc32fast::Hasher>(data),
        HashC::Blake256 => hash_hex::<Blake2s256>(data),
        HashC::Blake512 => hash_hex::<Blake2b512>(data),
        HashC::Ripemd128 => hash_hex::<Ripemd128>(data),
        HashC::Ripemd160 => hash_hex::<Ripemd160>(data),
        HashC::Sha224 => hash_hex::<Sha224>(data),
        HashC::Sha256 => hash_hex::<Sha256>(data),
        HashC::Sha384 => hash_hex::<Sha384>(data),
        HashC::Sha512 => hash_hex::<Sha512>(data),
    }
}

fn hash_hex<T: Digest>(data: &[u8]) -> String {
    let mut hasher = T::new();
    hasher.update(data);
    let hash = hasher.finalize();
    hash.iter().map(|x| format!("{:x}", x)).collect()
}

fn crc_hex<T: Hasher>(data: &[u8]) -> String {
    let nr = crc32fast::hash(data);
    format!("{:x}", nr)
}

/// Allowed crypto hashes
#[derive(Clone, PartialEq, Eq, Debug, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum HashC {
    Crc32,
    Blake256,
    Blake512,
    Ripemd128,
    Ripemd160,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
}

impl fmt::Display for HashC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
