//The most primitive representation of blockchain block
pub struct Block<Header, Extrinsic> {
    // Contains metadata about the block
    pub header: Header,
    // Represents the transactions to be executed in the block
    pub extrinsics: Vec<Extrinsic>,
}

// Extremely simple header, only contains the block number
// On real blockchain, it would contain:
// - parent block hash
// - state root hash
// - extrinsics root hash
// - difficulty
// - timestamp
// - nonce
// - etc.
pub struct Header<BlockNumber> {
    pub block_number: BlockNumber,
}

// Shows who is calling the function and what function is being called
pub struct Extrinsic<Caller, Call> {
    pub caller: Caller,
    pub call: Call,
}

// The result of the function call
// If everything is ok, it returns Ok(())
// If there is an error, it returns Err(&'static str) - static error message
pub type DispatchResult = Result<(), &'static str>;

// A trait that defines the dispatch function
// Allows us to call functions on the blockchain
// Allows us to dispatch an incoming extrinsic to the appropriate state transition function call
pub trait Dispatch {
    // The type used to identify the caller of the function
    type Caller;
    // The state transition function call the caller trying to access
    type Call;

    // This function takes the 'caller' and the 'call' they want to make, and returns the result of the function call
    fn dispatch(&mut self, caller: Self::Caller, call: Self::Call) -> DispatchResult;
}