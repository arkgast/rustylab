pub type AccountId = String;
pub type Balance = u128;
pub type BlockNumber = u32;
pub type Nonce = u32;
pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
pub type Header = crate::support::Header<BlockNumber>;
pub type Block = crate::support::Block<Header, Extrinsic>;
