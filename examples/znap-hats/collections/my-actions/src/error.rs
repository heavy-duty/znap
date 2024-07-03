use znap::prelude::*;

#[derive(ErrorCode)]
pub enum ActionError {
    #[error(msg = "Invalid account public key")]
    InvalidAccountPublicKey,
    #[error(msg = "Invalid transfer instruction")]
    InvalidTransferInstruction,
    #[error(msg = "Internal server error")]
    InternalServerError,
    #[error(msg = "Unknown server error")]
    UnknownServerError,
    #[error(msg = "Car not found")]
    CarNotFound,
    #[error(msg = "Invalid response body")]
    InvalidResponseBody,
}