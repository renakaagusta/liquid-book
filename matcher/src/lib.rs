// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U256},
    prelude::{entrypoint, public, sol_interface, sol_storage},
    evm,
    console
};

sol! {
    event MatchOrder(address indexed user, int128 indexed tick, uint256 order_index, bool is_buy, bool is_market, uint256 volume);
}

sol_storage! {
    #[entrypoint]
    pub struct MatcherManager {
        address tick_manager_address;
        address bitmap_manager_address;
        address order_manager_address;
    }
}

sol_interface! {
    interface ITickManager {
        function setTickData(int128 tick, uint256 volume, bool is_buy, bool is_existing_order) external;
        function getTickData(int128 tick) external view returns (uint256, uint256, uint256, bool);
        function getCurrentTick() external view returns (uint256);
        function setCurrentTick(int128 tick) external returns (uint256);
    }

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
    pub fn initialize(&mut self, tick_manager_address: Address, bitmap_manager_address: Address, order_manager_address: Address) {
        self.tick_manager_address.set(tick_manager_address);
        self.bitmap_manager_address.set(bitmap_manager_address);
        self.order_manager_address.set(order_manager_address);
    }

    fn execute(
        &mut self,
        user: Address,
        is_buy: bool,
        is_market: bool,
        valid_orders: Vec<(i128, U256, U256)>,
        incoming_order_volume: U256,
        tick_value: i128,
        tick_volume: U256
    ) -> U256 {
        let mut remaining_incoming_order_volume = incoming_order_volume;
        let mut remaining_tick_volume = tick_volume;
        let tick_manager = ITickManager::new(self.tick_manager_address.get());
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

            remaining_tick_volume -= order_volume - remaining_order_volume;

            evm::log(MatchOrder {
                user: user,
                tick: order_tick,
                is_buy: is_buy,
                order_index: order_index,
                is_market: is_market,
                volume: order_volume - remaining_order_volume
            });

            bitmap_manager.set_current_tick(&mut *self, order_tick);
            order_manager.update_order(
                &mut *self,
                order_tick,
                order_index,
                remaining_order_volume,
            );

            // console!("MATCHER :: MATCH ORDER :: order tick: {}, order index: {}, volume: {}", order_tick, order_index, order_volume);

            if remaining_incoming_order_volume == U256::ZERO {
                break;
            }
        }

        tick_manager.set_tick_data(self, tick_value, remaining_tick_volume, is_buy, true);

        // console!("MATCHER :: MATCH ORDER :: tick :: remaining_incoming_order_volume: {}, tick: {}", tick_value, remaining_incoming_order_volume);

        remaining_incoming_order_volume
    }
}
