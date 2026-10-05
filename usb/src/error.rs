#[derive(thiserror::Error, Debug)]
pub enum ConnectError {
    #[error("No GoXLR device was found")]
    DeviceNotFound,

    #[error("Device is not a GoXLR")]
    DeviceNotGoXLR,

    #[error("Unable to Claim Interface")]
    DeviceNotClaimed,
}

#[derive(thiserror::Error, Debug)]
pub enum CommandError {
    #[error("Malformed response from GoXLR")]
    MalformedResponse(#[from] std::io::Error),
}
