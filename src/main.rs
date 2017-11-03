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
use std::rc::Rc;
use std::str::FromStr;

fn main() { }

// Functions that you wish to access from Javascript
// must be marked as no_mangle

#[no_mangle]
pub fn run_example() -> i32 {
    let header = HeaderParams {
        beneficiary: Address::default(),
        timestamp: 0x01,
        number: U256::zero(),
        difficulty: U256::zero(),
        gas_limit: Gas::max_value(),
    };

    let context = Context {
        address: Address::default(),
        caller: Address::default(),
        code: Rc::new(read_hex("0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff01600055").unwrap()),
        data: Rc::new(Vec::new()),
        gas_limit: Gas::from_str("0x0186a0").unwrap(),
        gas_price: Gas::zero(),
        origin: Address::default(),
        value: U256::zero(),
        apprent_value: U256::zero(),
        is_system: false,
    };

    let mut vm = SeqContextVM::<MainnetEmbeddedPatch>::new(context, header.clone());
    vm.commit_account(AccountCommitment::Nonexist(Address::default()));
    vm.fire().unwrap();

    match vm.status() {
        VMStatus::ExitedOk => return 0,
        _ => return 1,
    }
}
