use crate::{pwhash::PWHash, result::Result};
use std::{fmt, io};

#[derive(Clone)]
pub enum Format {
    Basic(Basic),
}

impl Format {
    pub fn derive_key(&mut self, password: &[u8], key: &mut [u8]) -> Result {
        match self {
            Format::Basic(derive) => derive.derive_key(password, key),
        }
    }

    pub fn mut_pwhash(&mut self) -> &mut PWHash {
        match self {
            Format::Basic(derive) => derive.mut_pwhash(),
        }
    }

    pub fn pwhash(&self) -> &PWHash {
        match self {
            Format::Basic(derive) => derive.pwhash(),
        }
    }

    pub fn read(&mut self, reader: &mut dyn io::Read) -> Result {
        match self {
            Format::Basic(derive) => derive.read(reader),
        }
    }

    pub fn write(&self, writer: &mut dyn io::Write) -> Result {
        match self {
            Format::Basic(derive) => derive.write(writer),
        }
    }

    pub fn basic(pwhash: PWHash) -> Self {
        Format::Basic(Basic { pwhash })
    }

}

#[derive(Clone)]
pub struct Basic {
    pub pwhash: PWHash,
}

impl Basic {
    pub fn derive_key(&mut self, password: &[u8], key: &mut [u8]) -> Result {
        self.pwhash.pwhash(password, key)
    }

    pub fn mut_pwhash(&mut self) -> &mut PWHash {
        &mut self.pwhash
    }

    pub fn pwhash(&self) -> &PWHash {
        &self.pwhash
    }

    pub fn read(&mut self, _reader: &mut dyn io::Read) -> Result {
        Ok(())
    }

    pub fn write(&self, _writer: &mut dyn io::Write) -> Result {
        Ok(())
    }
}

#[derive(Clone)]
pub struct KeyShare(pub(crate) [u8; 33]);

impl Default for KeyShare {
    fn default() -> Self {
        KeyShare([0; 33])
    }
}

impl fmt::Debug for KeyShare {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("KeyShare").field(&&self.0[..]).finish()
    }
}

impl KeyShare {
    pub fn to_vec(&self) -> Vec<u8> {
        self.0.to_vec()
    }

    pub fn from_slice(slice: &[u8]) -> KeyShare {
        let mut share = [0u8; 33];
        share.copy_from_slice(slice);
        KeyShare(share)
    }
}
