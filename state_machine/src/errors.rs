use std::fmt::Display;

#[derive(Debug, PartialEq, Eq)]
pub enum ProofOfExistenceError {
    ClaimAlreadyExists,
    ClaimNotFound,
    NotOwner,
}

#[derive(Debug, PartialEq, Eq)]
pub enum SystemError {
    BlockNumberOverflow,
    NonceOverflow,
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
        write!(f, "{:?}", self)
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
        write!(f, "{:?}", self)
    }
}
