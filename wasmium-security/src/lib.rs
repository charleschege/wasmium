/// Signs a WasmiumHash hash of the module eg.
/// WasmiumHash(profile | account | issued | expiry | WasmiumLease | CapabilitiesLength | wasm module)

/// To access the database path, the challange is to sign the account hash with its public key
use ed25519_dalek::{Keypair, PublicKey, Signature, Signer};
use rand::rngs::OsRng;
use tai64::TAI64N;

mod global;
pub use global::*;
mod manifest_body;
pub use manifest_body::*;
mod manifest_header;
pub use manifest_header::*;

/*prevent hash from being //TODO
    - root
    - superuser
    - test
    - admin
    - user
*/

#[derive(Debug, Clone)]
pub struct WasmiumManifest<'wm> {
    header: WasmiumManifestHeader,
    body: WasmiumManifestBody<'wm>,
}
