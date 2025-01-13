// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main, no_std)]
extern crate alloc;

use crate::alloc::string::ToString;
use alloy_sol_macro::sol;
use stylus_sdk::{
    alloy_primitives::{Address, I128, U128, U256},
    console, evm,
    prelude::{entrypoint, public, sol_interface, sol_storage},
};

sol! {
    event SetTickData(int128 indexed tick, bool indexed is_buy, uint256 indexed volume, bool is_existing_order);
}

sol_storage! {
    #[entrypoint]
    pub struct TickManager {
        address engine_address;
        address order_manager_address;
        address bitmap_manager_address;
        mapping(int128 => Tick) ticks;
    }

    pub struct Order {
        address user;
        uint128 volume;
    }

    pub struct Tick {
        uint128 start_index;
        uint128 length;
        uint128 volume;
        bool is_buy;
    }
}

sol_interface! {
    interface IBitmapStorage {
        function position(int32 tick) external returns (int16, uint8);
        function flip(int32 tick) external returns (int16, uint8);
        function nextTick(int32 tick, bool lte) external view returns (int32, bool);
    }
}

#[public]
impl TickManager {
    pub fn initialize(
        &mut self,
        engine_address: Address,
        bitmap_manager_address: Address,
        order_manager_address: Address,
    ) {
        self.engine_address.set(engine_address);
        self.bitmap_manager_address.set(bitmap_manager_address);
        self.order_manager_address.set(order_manager_address);
    }

    pub fn set_tick_data(
        &mut self,
        tick: i128,
        volume: U256,
        is_buy: bool,
        is_existing_order: bool,
    ) {
        let tick_data = self.ticks.get(tick.to_string().parse::<I128>().unwrap());
        let mut updated_start_index = tick_data.start_index.get();
        let mut updated_length = tick_data.length.get();
        let mut updated_volume = tick_data.volume.get();
        let mut updated_is_buy = tick_data.is_buy.get();
        let initial_volume = tick_data.volume.get();

        if is_existing_order {
            if volume == U256::ZERO || updated_volume == U128::ZERO {
                updated_start_index += U128::from(1) % U128::from(256);
                updated_is_buy = !tick_data.is_buy.get();

                self.ticks
                    .setter(tick.to_string().parse::<I128>().unwrap())
                    .start_index
                    .set(updated_start_index);
                self.ticks
                    .setter(tick.to_string().parse::<I128>().unwrap())
                    .is_buy
                    .set(updated_is_buy);

                if volume == U256::ZERO {
                    updated_length -= U128::from(1) % U128::from(256);

                    self.ticks
                        .setter(tick.to_string().parse::<I128>().unwrap())
                        .length
                        .set(updated_length);
                }
            }

            updated_volume = U128::from(volume);
        } else {
            if tick_data.volume.get() == U128::ZERO
                || (tick_data.is_buy.get() != is_buy && U128::from(volume) > tick_data.volume.get())
            {
                updated_volume = U128::from(volume) - tick_data.volume.get();
                updated_is_buy = !tick_data.is_buy.get();

                self.ticks
                    .setter(tick.to_string().parse::<I128>().unwrap())
                    .is_buy
                    .set(updated_is_buy);
            } else if tick_data.is_buy.get() != is_buy {
                updated_volume = tick_data.volume.get() - U128::from(volume);
            } else if tick_data.is_buy.get() == is_buy {
                updated_volume = tick_data.volume.get() + U128::from(volume);
            } else {
                updated_volume = U128::from(0);
            }

            updated_length += U128::from(1) % U128::from(256);

            self.ticks
                .setter(tick.to_string().parse::<I128>().unwrap())
                .length
                .set(updated_length);
        }

        self.ticks
            .setter(tick.to_string().parse::<I128>().unwrap())
            .volume
            .set(updated_volume);

        if initial_volume == U128::ZERO && updated_volume != U128::ZERO
            || initial_volume != U128::ZERO && updated_volume == U128::ZERO
        {
            let converted_tick: i32 = tick.try_into().unwrap();
            let bitmap_manager = IBitmapStorage::new(self.bitmap_manager_address.get());

            let _ = bitmap_manager.flip(self, converted_tick);
        }

        evm::log(SetTickData {
            tick: tick,
            is_buy: updated_is_buy,
            volume: U256::from(updated_volume),
            is_existing_order: is_existing_order,
        });
    }

    pub fn get_tick_data(&self, tick: i128) -> (U256, U256, U256, bool) {
        let tick_data = self.ticks.get(tick.to_string().parse::<I128>().unwrap());
        // console!("TICK :: get tick data :: tick: {}, volume: {}, is buy: {}", tick, U256::from(tick_data.volume.get()), tick_data.is_buy.get());
        (
            U256::from(tick_data.start_index.get()),
            U256::from(tick_data.length.get()),
            U256::from(tick_data.volume.get()),
            tick_data.is_buy.get(),
        )
    }
}
