#[derive(Debug, Clone, Copy)]
pub enum WasmiumError {
    SignatureNotProvided,
    ErrorParsingWasmiumHash,
    ErrorParsingWasmiumProfile,
    ErrorParsingWasmiumPermissions,
    ErrorParsingWasmiumProtocol,
    ErrorParsingTAI64NSlice,
    TAI64NLengthInvalid,
    TAI64NNanosecondsInvalid,
}

pub type Blake3ByteHash = [u8; 32];
