// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{keccak256, Address, U128, U256},
    hostio::{storage_cache_bytes32, storage_flush_cache, storage_load_bytes32},
    prelude::{sol_storage, public, entrypoint},
};

sol_storage! {
    #[entrypoint]
    pub struct TickManager {
        address engine_address;
        address order_manager_address;
        address bitmap_manager_address;
        uint128 current_tick;
        mapping(uint128 => Tick) ticks;
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

    pub fn update_tick(&mut self, tick: U256, volume: U256, is_buy: bool, is_existing_order: bool) {
        let tick_data = self.ticks.get(U128::from(tick));
        let mut updated_start_index = tick_data.start_index.get();
        let mut updated_length = tick_data.length.get();
        let mut updated_volume = tick_data.volume.get();
        let mut updated_is_buy = tick_data.is_buy.get();

        if is_existing_order {
            updated_volume -= U128::from(volume);

            if volume == U256::ZERO {
                updated_start_index += U128::from(1) % U128::from(256);

                self.ticks
                    .setter(U128::from(tick))
                    .start_index
                    .set(updated_start_index);
            }
        } else {
            if tick_data.is_buy.get() != is_buy && U128::from(volume) > tick_data.volume.get() {
                updated_volume = U128::from(volume) - tick_data.volume.get();
                updated_is_buy = !tick_data.is_buy.get();

                self.ticks
                    .setter(U128::from(tick))
                    .is_buy
                    .set(updated_is_buy);
            } else if tick_data.is_buy.get() != is_buy {
                updated_volume = tick_data.volume.get() - U128::from(volume);
            } else {
                updated_volume = U128::from(0);
            }

            updated_length += U128::from(1) % U128::from(256);

            self.ticks
                .setter(U128::from(tick))
                .length
                .set(updated_length);
        }

        self.ticks
            .setter(U128::from(tick))
            .volume
            .set(updated_volume);
    }

    pub fn get_tick_data(&self, tick: U256) -> (U256, U256, U256, bool) {
        let tick_data = self.ticks
            .get(U128::from(tick));
        (U256::from(tick_data.start_index.get()), U256::from(tick_data.length.get()), U256::from(tick_data.volume.get()), tick_data.is_buy.get())
    }

    pub fn set_tick_data(&mut self, tick: U256, tick_data: (U256, U256, U256, bool))  {
        let (start_index, length, volume, is_buy) = tick_data;
        self.ticks.setter(U128::from(tick)).start_index.set(U128::from(start_index));
        self.ticks.setter(U128::from(tick)).length.set(U128::from(length));
        self.ticks.setter(U128::from(tick)).volume.set(U128::from(volume));
        self.ticks.setter(U128::from(tick)).is_buy.set(is_buy);
    }

    pub fn get_current_tick(&self) -> U256 {
        U256::from(self.current_tick.get())
    }

    pub fn set_current_tick(&mut self, tick: U256)  {
        self.current_tick.set(U128::from(tick));
    }
}
