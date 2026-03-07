use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ProofOfExistenceError {
    ClaimAlreadyExists,
    ClaimNotFound,
    NotOwner,
}

impl Display for ProofOfExistenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProofOfExistenceError::NotOwner => write!(f, "not owner"),
            ProofOfExistenceError::ClaimNotFound => write!(f, "claim not found"),
            ProofOfExistenceError::ClaimAlreadyExists => write!(f, "claim already exists"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum SystemError {
    BlockNumberOverflow,
    NonceOverflow,
}

impl Display for SystemError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SystemError::NonceOverflow => write!(f, "nonce overflow"),
            SystemError::BlockNumberOverflow => write!(f, "block number overflow"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TransferError {
    NotEnoughBalance,
    BalanceOverflow,
    CannotTransferToSelf,
    ZeroTransfer,
}

impl Display for TransferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferError::NotEnoughBalance => write!(f, "not enough balance"),
            TransferError::BalanceOverflow => write!(f, "balance overflow"),
            TransferError::CannotTransferToSelf => write!(f, "cannot transfer to self"),
            TransferError::ZeroTransfer => write!(f, "zero transfer is not allowed"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeError {
    ProofOfExistence(ProofOfExistenceError),
    System(SystemError),
    Transfer(TransferError),
}

impl From<ProofOfExistenceError> for RuntimeError {
    fn from(value: ProofOfExistenceError) -> Self {
        RuntimeError::ProofOfExistence(value)
    }
}

impl From<SystemError> for RuntimeError {
    fn from(value: SystemError) -> Self {
        RuntimeError::System(value)
    }
}

impl From<TransferError> for RuntimeError {
    fn from(value: TransferError) -> Self {
        RuntimeError::Transfer(value)
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::System(err) => write!(f, "System error: {}", err),
            RuntimeError::Transfer(err) => write!(f, "Transfer error: {}", err),
            RuntimeError::ProofOfExistence(err) => write!(f, "PoE error: {}", err),
        }
    }
}
