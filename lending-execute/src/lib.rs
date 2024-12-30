#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::Address;
use stylus_sdk::{alloy_primitives::U256, console, evm, prelude::*};

sol_interface! {
    interface IExecuteLending {
        function execute() external;
        function withdraw() external;
    }
}

sol_storage! {
    #[entrypoint]
    pub struct LendingExecute {
        address lending_address;
    }
}

#[public]
impl LendingExecute {
    pub fn initialize(&mut self, lending_address: Address) {
        self.lending_address.set(lending_address);
    }

    pub fn stake(&mut self) {
        let lending_address = self.lending_address.get();
        let execute_lending = IExecuteLending::new(lending_address);

        execute_lending.execute(self);

        console!("EXECUTE_LENDING :: execute lending");
    }

    pub fn withdraw(&mut self) {
        let lending_address = self.lending_address.get();
        let execute_lending = IExecuteLending::new(lending_address);

        execute_lending.withdraw(self);

        console!("EXECUTE_LENDING :: withdraw lending");
    }
}
