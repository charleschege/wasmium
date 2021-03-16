use ed25519_dalek::{PublicKey, Signature};
use tai64::TAI64N;

use crate::WasmiumError;
/// (identifier (32bytes) | hash (32bytes) | signature (64bytes) | manifest_body_byte_length)
/// Signature - 64 bytes
#[derive(Debug, Clone, Copy)]
pub struct WasmiumManifestHeader {
    identifier: blake3::Hash,
    account: Option<PublicKey>,
    hash: blake3::Hash,
    signature: Option<Signature>,
}

impl Default for WasmiumManifestHeader {
    fn default() -> Self {
        let time_now = TAI64N::now();

        Self {
            identifier: blake3::hash(b""),
            account: Option::default(),
            hash: blake3::hash(b""),
            signature: Option::default(),
        }
    }
}

impl WasmiumManifestHeader {
    pub fn add_account(&mut self, value: PublicKey) -> &mut Self {
        self.account = Some(value);

        self
    }

    pub fn from_bytes(&mut self, signature: Signature) -> &mut Self {
        self.signature = Some(signature);

        self
    }

    pub fn to_bytes(&self) -> Result<[u8; 128], WasmiumError> {
        let mut header_bytes = [0_u8; 128];
        header_bytes[0..=31].copy_from_slice(self.identifier.as_bytes());
        header_bytes[32..=63].copy_from_slice(self.hash.as_bytes());

        match self.signature {
            None => return Err(WasmiumError::SignatureNotProvided),
            Some(signature) => header_bytes[64..=127].copy_from_slice(&signature.to_bytes()),
        }

        Ok(header_bytes)
    }
}
