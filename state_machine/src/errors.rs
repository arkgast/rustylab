use std::fmt::Display;

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
pub enum SystemError {
    BlockNumberOverflow,
    NonceOverflow,
}
