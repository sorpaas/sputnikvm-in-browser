extern crate sputnikvm;
extern crate rlp;
extern crate hexutil;
extern crate bigint;
extern crate block_core;

use sputnikvm::{VM, HeaderParams, Context, SeqContextVM, MainnetEmbeddedPatch, RequireError, AccountCommitment, ValidTransaction, VMStatus, SeqTransactionVM};
use block_core::{TransactionAction};
use bigint::{Gas, H256, U256, Address};
use rlp::Rlp;
use hexutil::*;

fn main() { }

// Functions that you wish to access from Javascript
// must be marked as no_mangle

#[no_mangle]
pub fn run_example() -> i32 {
    return 0;
}
