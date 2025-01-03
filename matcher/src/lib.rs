// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_sol_macro::sol;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    prelude::{entrypoint, public, sol_interface, sol_storage},
    // console
};

sol_storage! {
    #[entrypoint]
    pub struct MatcherManager {
        address order_manager_address;
        address bitmap_manager_address;
    }
}

sol_interface! {
    interface IBitmapManager {
        function setCurrentTick(int128 tick) external returns (uint256);
        function flip(int32 tick) external returns (int16, uint8);
    }

    interface IOrderManager {
        function updateOrder(int128 tick, uint256 volume, uint256 order_index) external;
    }
}

#[public]
impl MatcherManager {
    pub fn initialize(&mut self, bitmap_manager_address: Address, order_manager_address: Address) {
        self.bitmap_manager_address.set(bitmap_manager_address);
        self.order_manager_address.set(order_manager_address);
    }

    fn execute(
        &mut self,
        valid_orders: Vec<(i128, U256, U256)>,
        incoming_order_volume: U256,
        tick_value: i128,
        tick_volume: U256
    ) -> U256 {
        // console!("MATCHER :: remaining incoming order volume:");

        let mut remaining_incoming_order_volume = incoming_order_volume;
        let bitmap_manager = IBitmapManager::new(self.bitmap_manager_address.get());
        let order_manager = IOrderManager::new(self.order_manager_address.get());

        for (order_tick, order_index, order_volume) in valid_orders {
            let mut remaining_order_volume = order_volume;

            if remaining_order_volume < remaining_incoming_order_volume {
                remaining_incoming_order_volume -= order_volume;
                remaining_order_volume = U256::ZERO;
            } else if remaining_order_volume == remaining_incoming_order_volume {
                remaining_order_volume = U256::ZERO;
                remaining_incoming_order_volume = U256::ZERO;
            } else {
                remaining_order_volume -= remaining_incoming_order_volume;
                remaining_incoming_order_volume = U256::ZERO;
            }

            let result = bitmap_manager.set_current_tick(&mut *self, order_tick);
            let _ = order_manager.update_order(
                &mut *self,
                order_tick,
                order_index,
                remaining_order_volume,
            );

            if remaining_incoming_order_volume == U256::ZERO {
                break;
            }
        }

        if incoming_order_volume >= tick_volume {     
            let converted_tick: i32 = tick_value.try_into().unwrap();
            bitmap_manager.flip(&mut *self, converted_tick);
        }

        // console!("MATCHER :: remaining incoming order volume: {}", remaining_incoming_order_volume);

        remaining_incoming_order_volume
    }
}
