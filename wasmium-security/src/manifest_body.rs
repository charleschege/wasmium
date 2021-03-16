use crate::Blake3ByteHash;
use camino::Utf8Path;
use ed25519_dalek::{PublicKey, Signature};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tai64::TAI64N;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WasmiumManifestBody<'wm> {
    profile: WasmiumProfile,
    account: Option<PublicKey>,
    issue: TAI64N,
    lease: WasmiumLease,
    #[serde(borrow)]
    cap: Vec<WasmiumCapability<'wm>>,
    wasm: Vec<u8>,
}

impl<'wm> Default for WasmiumManifestBody<'wm> {
    fn default() -> Self {
        let time_now = TAI64N::now();

        Self {
            profile: WasmiumProfile::Development,
            account: Option::default(),
            issue: time_now,
            lease: WasmiumLease::Expiry(time_now),
            cap: Vec::default(),
            wasm: Vec::default(),
        }
    }
}

impl<'wm> WasmiumManifestBody<'wm> {
    pub fn change_profile(&mut self, value: WasmiumProfile) -> &mut Self {
        self.profile = value;

        self
    }

    pub fn add_account(&mut self, value: PublicKey) -> &mut Self {
        self.account = Some(value);

        self
    }

    pub fn change_lease(&mut self, lease: WasmiumLease) -> &mut Self {
        self.lease = lease;

        self
    }

    pub fn add_capability(&mut self, capability: WasmiumCapability<'wm>) -> &mut Self {
        self.cap.push(capability);

        self
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WasmiumProfile {
    Development,
    Staging,
    Production,
}

impl Default for WasmiumProfile {
    fn default() -> Self {
        Self::Development
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WasmiumCapability<'wm> {
    NetworkAccess {
        socket: SocketAddr,
        protocol: WasmiumProtocol,
    },
    FileSystem {
        #[serde(borrow)]
        file_path: &'wm Utf8Path,
        permissions: WasmiumPermissions,
    },
    Module(Blake3ByteHash),
    Store {
        #[serde(borrow)]
        store_path: &'wm Utf8Path,
        permissions: WasmiumPermissions,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct WasmiumPermissions {
    open: bool,
    read: bool,
    write: bool,
    execute: bool,
    append: bool,
}

impl Default for WasmiumPermissions {
    fn default() -> Self {
        WasmiumPermissions {
            open: bool::default(),
            read: bool::default(),
            write: bool::default(),
            execute: bool::default(),
            append: bool::default(),
        }
    }
}
impl WasmiumPermissions {
    pub fn open(&mut self, value: bool) -> &mut Self {
        self.open = value;

        self
    }

    pub fn read(&mut self, value: bool) -> &mut Self {
        self.read = value;

        self
    }

    pub fn write(&mut self, value: bool) -> &mut Self {
        self.write = value;

        self
    }

    pub fn execute(&mut self, value: bool) -> &mut Self {
        self.execute = value;

        self
    }

    pub fn append(&mut self, value: bool) -> &mut Self {
        self.append = value;

        self
    }

    pub fn can_open(&self) -> bool {
        self.open
    }

    pub fn can_read(&self) -> bool {
        self.read
    }

    pub fn can_write(&self) -> bool {
        self.write
    }

    pub fn can_execute(&self) -> bool {
        self.execute
    }

    pub fn can_append(&self) -> bool {
        self.append
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasmiumProtocol {
    NoAccess,
    UDP,
    TCP,
    HTTP,
    QUIC,
    Websocket,
    ModuleSentEvents,
}

impl Default for WasmiumProtocol {
    fn default() -> Self {
        WasmiumProtocol::NoAccess
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WasmiumLease {
    ExpiresNever,
    Expiry(TAI64N),
}

impl Default for WasmiumLease {
    fn default() -> Self {
        WasmiumLease::Expiry(TAI64N::now())
    }
}
