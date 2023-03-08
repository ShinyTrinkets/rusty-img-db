use blake2::{Blake2b512, Blake2s256};
use clap::ValueEnum;
use digest::Digest;
use sha2::{Sha224, Sha256, Sha384, Sha512};
use std::fmt;

/// Crypto hash
pub fn hash_c(h: HashC, data: &[u8]) -> String {
    match h {
        HashC::Sha224 => hash_hex::<Sha224>(data),
        HashC::Sha256 => hash_hex::<Sha256>(data),
        HashC::Sha384 => hash_hex::<Sha384>(data),
        HashC::Sha512 => hash_hex::<Sha512>(data),
        HashC::Blake256 => hash_hex::<Blake2s256>(data),
        HashC::Blake512 => hash_hex::<Blake2b512>(data),
    }
}

fn hash_hex<T: Digest>(data: &[u8]) -> String {
    let mut hasher = T::new();
    hasher.update(data);
    let hash = hasher.finalize();
    hash.iter().map(|x| format!("{:x}", x)).collect()
}

/// Allowed crypto hashes
#[derive(Clone, PartialEq, Eq, ValueEnum, Debug)]
pub enum HashC {
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
