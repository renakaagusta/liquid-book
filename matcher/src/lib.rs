// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_sol_macro::sol;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    evm,
    prelude::{entrypoint, public, sol_interface, sol_storage},
    console,
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
        address pool_address;
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
        function convertFromTickToPrice(int128 tick) external view returns (uint256);
    }

    interface IOrderManager {
        function updateOrder(int128 tick, uint256 volume, uint256 order_index) external;
    }

    interface IPoolLiquidBook {
        function transferLocked(
            int128 transfer_tick,
            uint256 transfer_volume,
            address sender,
            address receiver
        ) external;
    }
}

#[public]
impl MatcherManager {
    pub fn initialize(
        &mut self,
        tick_manager_address: Address,
        bitmap_manager_address: Address,
        order_manager_address: Address,
        pool_address: Address,
    ) {
        self.tick_manager_address.set(tick_manager_address);
        self.bitmap_manager_address.set(bitmap_manager_address);
        self.order_manager_address.set(order_manager_address);
        self.pool_address.set(pool_address);
    }

    fn execute(
        &mut self,
        user: Address,
        is_buy: bool,
        is_market: bool,
        valid_orders: Vec<(i128, U256, U256, Address)>,
        incoming_order_volume: U256,
        tick_value: i128,
        tick_volume: U256,
    ) -> Result<U256, Vec<u8>> {
        console!("MATCHER :: match :: 1");

        let mut remaining_incoming_order_volume = incoming_order_volume;
        let mut remaining_tick_volume = tick_volume;
        let tick_manager = ITickManager::new(self.tick_manager_address.get());
        let bitmap_manager = IBitmapManager::new(self.bitmap_manager_address.get());
        let order_manager = IOrderManager::new(self.order_manager_address.get());
        let pool = IPoolLiquidBook::new(self.pool_address.get());

        for (order_tick, order_index, order_volume, order_user) in valid_orders {
            let mut remaining_order_volume = order_volume;
            let use_order_volume;
            let (buyer, seller) = if is_buy {
                (user, order_user)
            } else {
                (order_user, user)
            };

            if remaining_order_volume < remaining_incoming_order_volume {
                use_order_volume = order_volume;
                remaining_incoming_order_volume -= order_volume;
                remaining_order_volume = U256::ZERO;
            } else if remaining_order_volume == remaining_incoming_order_volume {
                use_order_volume = order_volume;
                remaining_order_volume = U256::ZERO;
                remaining_incoming_order_volume = U256::ZERO;
            } else {
                use_order_volume = remaining_incoming_order_volume;
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
                volume: order_volume - remaining_order_volume,
            });

            console!("MATCHER :: match :: 2");

            // console!(
            //     "Tick: {:?}, Buyer: {:?}, Seller: {:?}, Volume: {:?}",
            //     order_tick,
            //     buyer,
            //     seller,
            //     use_order_volume
            // );

            // TODO
            // if let Err(e) =
            //     pool.transfer_locked(&mut *self, order_tick, use_order_volume, buyer, seller)
            // {
            //     // console!("Error during transfer_locked: {:?}", e);
            //     return Err(e.into());
            // };

            let _ = bitmap_manager.set_current_tick(&mut *self, order_tick);
            let _ = order_manager.update_order(
                &mut *self,
                order_tick,
                remaining_order_volume,
                order_index,
            );
            if remaining_incoming_order_volume == U256::ZERO {
                break;
            }
        }

        let _ = tick_manager.set_tick_data(self, tick_value, remaining_tick_volume, is_buy, true);

        // console!("MATCHER :: MATCH ORDER :: tick :: remaining_incoming_order_volume: {}, tick: {}", tick_value, remaining_incoming_order_volume);

        Ok(remaining_incoming_order_volume)
    }
}
