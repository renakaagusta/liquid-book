#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::Address;
use stylus_sdk::{alloy_primitives::U256, console, msg, prelude::*};

sol_interface! {
    interface IExecuteLending {
        function deposit(uint256 amount, address sender) external;
        function withdraw(uint256 amount, address sender) external;
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

        console!(
            "EXECUTE_LENDING :: initialize lending address: {:?}",
            lending_address
        );
    }

    pub fn deposit(&mut self, amount: U256) {
        let lending_address = self.lending_address.get();
        let execute_lending = IExecuteLending::new(lending_address);
        let _ = execute_lending.deposit(self, amount, msg::sender());

        console!("EXECUTE_LENDING :: execute lending");
    }

    pub fn withdraw(&mut self, amount: U256) {
        let lending_address = self.lending_address.get();
        let execute_lending = IExecuteLending::new(lending_address);
        let _ = execute_lending.withdraw(self, amount, msg::sender());

        console!("EXECUTE_LENDING :: withdraw lending");
    }
}
