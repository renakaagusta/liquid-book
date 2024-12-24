// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256},
    console,
    prelude::{entrypoint, public, sol_interface, sol_storage},
};

sol_storage! {
    #[entrypoint]
    pub struct Matcher {
        address tick_manager_address;
        address order_manager_address;
    }
}

sol_interface! {
    interface ITickManager {
        function initialize(address engine_address, address bitmap_manager_address, address order_manager_address) external;
        function updateTick(uint256 tick, uint256 volume, bool is_buy, bool is_existing_order) external;
        function getTickData(uint256 tick) external view returns (uint256, uint256, uint256, bool);
        function setTickData(uint256 tick, (uint256, uint256, uint256, bool) tick_data) external;
        function getCurrentTick() external view returns (uint256);
        function setCurrentTick(uint256 tick) external;
    }

    interface IOrderManager {
        function initialize(address engine_address, address bitmap_manager_address, address tick_manager_address) external;
        function insertOrder(uint256 tick, uint256 volume, address user, bool is_buy) external;
        function updateOrder(uint256 tick, uint256 volume, uint256 order_index) external;
        function readOrder(uint256 tick, uint256 order_index) external view returns (address, uint256);
        function writeOrder(uint256 tick, uint256 order_index, address user, uint256 volume) external;
        function deleteOrder(uint256 tick, uint256 order_index) external;
        function encodeOrderKey(uint256 tick, uint256 order_index) external view returns (uint8[] memory);
        function encodeOrderData(address user, uint256 volume) external view returns (uint8[32] memory);
        function decodeOrderData(uint8[] memory encoded) external view returns (address, uint256);
    }
}

#[public]
impl Matcher {
    pub fn initialize(&mut self, tick_manager_address: Address, order_manager_address: Address) {
        self.tick_manager_address.set(tick_manager_address);
        self.order_manager_address.set(order_manager_address);
    }

    fn execute(
        &mut self,
        valid_orders: Vec<(U256, U256, U256)>,
        incoming_order_quantity: U256,
    ) -> U256 {
        let mut remaining_incoming_order_quantity = incoming_order_quantity;
        let tick_manager_address = self.tick_manager_address.get();
        let order_manager_address = self.order_manager_address.get();
        let tick_manager = ITickManager::new(tick_manager_address);
        let order_manager = IOrderManager::new(order_manager_address);

        for (order_index, order_tick, order_quantity) in valid_orders {
            let mut remaining_order_quantity = order_quantity;

            if remaining_order_quantity < remaining_incoming_order_quantity {
                remaining_incoming_order_quantity -= order_quantity;
                remaining_order_quantity = U256::ZERO;
            } else if remaining_order_quantity == remaining_incoming_order_quantity {
                remaining_order_quantity = U256::ZERO;
                remaining_incoming_order_quantity = U256::ZERO;
            } else {
                remaining_order_quantity -= remaining_incoming_order_quantity;
                remaining_incoming_order_quantity = U256::ZERO;
            }

            let _ = tick_manager.set_current_tick(&mut *self, order_tick);
            let _ = order_manager.update_order(
                &mut *self,
                order_tick,
                order_index,
                remaining_order_quantity,
            );

            if remaining_incoming_order_quantity == U256::ZERO {
                break;
            }
        }

        remaining_incoming_order_quantity
    }
}
