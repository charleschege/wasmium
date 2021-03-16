#### Wasmium-Security (pre-alpha)

This is a part of the `Wasmium` crate that handles security functions such as signing and verifying of web assembly modules. It uses the Edwards-Curve Digital Signature algorithm with Curve25519 and Blake3 as the hashing algorithm.

The contents as stored as a `.wasmium` file in bytes using `bincode` default settings.

`TAI64N` is used to provide nanosecond accuracy on time issued and lease of the module while avoiding having to deal with nanoseconds. All paths are `UTF8` encoded.

The structure of the manifest file that describes the module is:

`manifest header | manifest body`

##### Manifest Header

The manifest header outlines the identity of a module as a `blake3::Hash` format, the hash of the body bytes and the `Ed25519` signature of the file `signed by the owner of the wasm module`

`identifier (32bytes) | account Ed25519 Public Key | blake3 Hash (32bytes) | Ed25519 Signature (64 bytes)   `

##### Manifest Body

`profile | issue | lease | capabilities | wasm module`

The manifest body contains

-  `profile` of the module which indicates if the module is being run in `Development`, `Staging` or `Production` mode
- `issue` the time containing a `TAI64N` timestamp showing the time the module was signed
- `lease` shows when the module will expire
- `cap` shows what the module can access or not
- `wasm` contains the bytes to be executed as a wasm module

#### Warning

THIS  SOFTWARE IS PRE-ALPHA AND THE APIs MIGHT CHANGE DRASTICALLY BEFORE VERSION `1.0.0`