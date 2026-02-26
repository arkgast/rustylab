pub struct Block<Header, Extrinsic> {
    pub header: Header,
    pub extrinsics: Vec<Extrinsic>,
}

pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call,
}

pub type DispatchResult<T> = Result<(), T>;

pub trait Dispatch {
    type Caller;
    type Call;

    fn dispatch<T>(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult<T>;
}
