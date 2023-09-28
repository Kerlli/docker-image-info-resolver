use std::io;
use sha2::{Sha256, Digest};

pub fn digest(data: &[u8]) -> String {
  let mut hasher = Sha256::new();
  hasher.update(data);
  base16ct::lower::encode_string(&hasher.finalize()[..])
}

pub fn digest_io(readable: &mut impl io::Read) -> String {
  let mut hasher = Sha256::new();
  io::copy(readable, &mut hasher).unwrap();
  base16ct::lower::encode_string(&hasher.finalize()[..])
}
