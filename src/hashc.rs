use blake2::{Blake2b512, Blake2s256};
use clap::ValueEnum;
use crc32fast;
use digest::Digest;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use std::fmt;
use std::hash::Hasher;

/// Cryptographical hash
pub fn hash_c(h: &HashC, data: &[u8]) -> String {
    match h {
        HashC::Sha224 => hash_hex::<Sha224>(data),
        HashC::Sha256 => hash_hex::<Sha256>(data),
        HashC::Sha384 => hash_hex::<Sha384>(data),
        HashC::Sha512 => hash_hex::<Sha512>(data),
        HashC::Blake256 => hash_hex::<Blake2s256>(data),
        HashC::Blake512 => hash_hex::<Blake2b512>(data),
        HashC::Crc32 => crc_hex::<crc32fast::Hasher>(data),
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
#[derive(Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum HashC {
    Crc32,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Blake256,
    Blake512,
}

impl fmt::Display for HashC {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
