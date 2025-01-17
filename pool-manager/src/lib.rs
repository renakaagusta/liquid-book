#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_sol_types::sol;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    contract, 
    // evm, 
    msg,
    prelude::*,
};

sol_interface! {
    interface IPoolLiquidBook {
        function initialize(
            address asset_token,
            address stable_token,
            address pool_manager,
            address engine_manager,
            address bitmap_manager,
            address balance_manager,
            int128 current_tick,
            uint256 lot_size,
        ) external;

        function getEngineAddress() external view returns (address);
        function getTokenAddress(bool is_buy) external view returns (address);
        function calculatePrice(uint256 volume, int128 tick) external view returns (uint256);
        function getLotSize() external view returns (uint256);
    }

    interface IBalanceManager {
        function transferLocked(address from, address operator, address to, address token, uint256 amount) external;
        function lock(address user, address operator, address token, uint256 amount) external;
    }

    interface ILiquidBookEngine {
        function placeOrder(
            uint256 incoming_order_volume,
            int128 incoming_order_tick,
            address incoming_order_user,
            bool incoming_order_is_buy,
            bool incoming_order_is_market
        ) external returns (uint256, int128, uint256);
    }
}

sol! {
    event PoolInitialized(address indexed pool_address, address indexed asset_token, address indexed stable_token, int128 current_tick, uint256 lot_size);
}

sol_storage! {
    #[entrypoint]
    pub struct PoolManager {
        address balance_manager;
        address[] pool_addresses;
        mapping(address => bool) is_pools;
    }
}

#[public]
impl PoolManager {
    pub fn initialize(&mut self, balance_manager: Address) {
        self.balance_manager.set(balance_manager);
    }

    pub fn add_pool(
        &mut self,
        pool_address: Address,
        asset_token: Address,
        stable_token: Address,
        engine_manager: Address,
        bitmap_manager: Address,
        current_tick: i128,
        lot_size: U256,
    ) {
        if self.is_pools.get(pool_address) {
            return;
        }

        let pool_liquid_book = IPoolLiquidBook::new(pool_address);
        let balance_manager = self.balance_manager.get();
        let _ = pool_liquid_book.initialize(
            &mut *self,
            asset_token,
            stable_token,
            contract::address(),
            engine_manager,
            bitmap_manager,
            balance_manager,
            current_tick,
            lot_size,
        );

        self.pool_addresses.push(pool_address);
        self.is_pools.insert(pool_address, true);

        // evm::log(PoolInitialized {
        //     pool_address,
        //     asset_token,
        //     stable_token,
        //     current_tick,
        //     lot_size,
        // });
    }

    pub fn place_order(
        &mut self,
        pool_address: Address,
        incoming_order_volume: U256,
        incoming_order_tick: i128,
        incoming_order_is_buy: bool,
        incoming_order_is_market: bool,
    ) -> Result<(), Vec<u8>> {
        let incoming_order_user = msg::sender();
        let pool_liquid_book = IPoolLiquidBook::new(pool_address);

        // TODO
        // let volume_locked = match incoming_order_is_buy {
        //     true => pool_liquid_book
        //         .calculate_price(&mut *self, incoming_order_volume, incoming_order_tick)
        //         .unwrap(),
        //     false => incoming_order_volume,
        // };

        // let token_address = pool_liquid_book
        //     .get_token_address(&*self, incoming_order_is_buy)
        //     .unwrap();
        // let _balance_manager = IBalanceManager::new(self.balance_manager.get());
        
        // if let Err(e) = _balance_manager.lock(
        //     &mut *self,
        //     incoming_order_user,
        //     contract::address(),
        //     token_address,
        //     volume_locked,
        // ) {
        //     return Err(e.into());
        // }

        // let lot_size = pool_liquid_book.get_lot_size(&mut *self).unwrap();

        // if incoming_order_volume < lot_size || incoming_order_volume % lot_size != U256::ZERO {
        //     return Err(b"Invalid order volume.".to_vec());
        // }

        let engine_manager = ILiquidBookEngine::new(pool_liquid_book.get_engine_address(&mut *self).unwrap());
        if let Err(e) = engine_manager.place_order(
            &mut *self,
            incoming_order_volume,
            incoming_order_tick,
            incoming_order_user,
            incoming_order_is_buy,
            incoming_order_is_market,
        ) {
            return Err(e.into());
        }

        Ok(())
    }
}
