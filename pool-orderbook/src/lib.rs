#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

// use alloy_sol_types::sol;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    // console,
    prelude::*,
};

sol_interface! {
    interface IERC20 {
        function decimals() external pure returns (uint8);
    }

    interface IPoolManager {
        function transferLockedFromPool(
            uint256 volume,
            address token_address,
            address sender,
            address receiver
        ) external;
    }

    interface IBitmapStorage {
        function setCurrentTick(int128 tick) external returns (uint256);
        function getCurrentTick() external returns (int128);
        function convertFromTickToPrice(int128 tick) external view returns (uint256);
    }

    interface IBalanceManager {
        function transferLocked(address from, address operator, address to, address token, uint256 amount) external;
        function lock(address user, address operator, address token, uint256 amount) external;
    }
}

sol_storage! {
    #[entrypoint]
    pub struct PoolLiquidBook {
        address asset_token;
        address stable_token;
        address pool_manager;
        address engine_manager;
        address bitmap_manager;
        address balance_manager;
        uint256 lot_size;
    }
}

#[public]
impl PoolLiquidBook {
    pub fn initialize(
        &mut self,
        asset_token: Address,
        stable_token: Address,
        pool_manager: Address,
        engine_manager: Address,
        bitmap_manager: Address,
        balance_manager: Address,
        current_tick: i128,
        lot_size: U256,
    ) {
        self.asset_token.set(asset_token);
        self.stable_token.set(stable_token);
        self.pool_manager.set(pool_manager);
        self.engine_manager.set(engine_manager);
        self.bitmap_manager.set(bitmap_manager);
        self.balance_manager.set(balance_manager);
        self.lot_size.set(lot_size);
        let bitmap_manager = IBitmapStorage::new(bitmap_manager);
        let _ = bitmap_manager.set_current_tick(&mut *self, current_tick);
    }

    pub fn get_token_address(&mut self, is_buy: bool) -> Address {
        if is_buy {
            self.stable_token.get()
        } else {
            self.asset_token.get()
        }
    }
    pub fn get_engine_address(&self) -> Address {
        self.engine_manager.get()
    }

    pub fn get_lot_size(&self) -> U256 {
        self.lot_size.get()
    }

    pub fn calculate_price(&mut self, volume: U256, tick: i128) -> U256 {
        let bitmap_manager = IBitmapStorage::new(self.bitmap_manager.get());
        let price_per_volume = bitmap_manager
            .convert_from_tick_to_price(&mut *self, tick)
            .unwrap();
        let token = IERC20::new(self.asset_token.get());
        let token_decimals = U256::from(token.decimals(&mut *self).unwrap());
        volume * price_per_volume / U256::from(10u64).pow(token_decimals)
    }

    pub fn transfer_locked(
        &mut self,
        transfer_tick: i128,
        transfer_volume: U256,
        buyer: Address,
        seller: Address,
    ) -> Result<(), Vec<u8>> {
        let pool_manager = self.pool_manager.get();
        let _balance_manager = IBalanceManager::new(self.balance_manager.get());

        let price = self.calculate_price(transfer_volume, transfer_tick);
        let stable_token = self.stable_token.get();
        let asset_token = self.asset_token.get();

        let transfer_operations = [
            (price, stable_token, buyer, seller),
            (transfer_volume, asset_token, seller, buyer),
        ];

        for (volume, token, from, to) in transfer_operations.iter() {
            if let Err(e) = _balance_manager.transfer_locked(
                &mut *self,
                *from,
                pool_manager,
                *to,
                *token,
                *volume,
            ) {
                return Err(e.into());
            }
        }
        Ok(())
    }
}
