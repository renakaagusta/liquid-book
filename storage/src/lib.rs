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
    pub struct LiquidBookStorage {
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
impl LiquidBookStorage {
    pub fn insert_order(&mut self, tick: U256, volume: U256, user: Address, is_buy: bool) {
        let tick_data = self.ticks.get(U128::from(tick));
        let order_index = tick_data.start_index.get() + tick_data.length.get() % U128::from(256);
        self.write_order(tick, U256::from(order_index), user, volume);
        self.update_tick(tick, volume, is_buy, false);
    }

    pub fn update_order(&mut self, tick: U256, volume: U256, order_index: U256) {
        let order_data = self.read_order(tick, order_index).unwrap();

        if volume == U256::ZERO {
            self.delete_order(tick, order_index);
        } else {
            self.write_order(tick, U256::from(order_index), order_data.0, volume);
        }

        self.update_tick(tick, volume, false, true);
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

    pub fn read_order(&self, tick: U256, order_index: U256) -> Result<(Address, U256), Vec<u8>> {
        let encoded_order_key = self.encode_order_key(tick, order_index).unwrap();
        let hashed_encoded_order_key = keccak256(encoded_order_key);

        let mut buffer_order_data: [u8; 32] = [0u8; 32];

        unsafe {
            storage_load_bytes32(
                hashed_encoded_order_key.as_ptr(),
                buffer_order_data.as_mut_ptr(),
            );
        }

        let encoded_order_data = buffer_order_data.to_vec();
        let decoded_order_data = self.decode_order_data(encoded_order_data);

        Ok(decoded_order_data.unwrap())
    }

    pub fn write_order(&mut self, tick: U256, order_index: U256, user: Address, volume: U256) {
        let encoded_order_key = self.encode_order_key(tick, order_index).unwrap();
        let encoded_order_data = self.encode_order_data(user, volume).unwrap();

        let hashed_encoded_order_key = keccak256(encoded_order_key);

        unsafe {
            storage_cache_bytes32(
                hashed_encoded_order_key.as_ptr(),
                encoded_order_data.as_ptr(),
            );
            storage_flush_cache(false);
        }
    }

    pub fn delete_order(&mut self, tick: U256, order_index: U256) {
        let encoded_order_key = self.encode_order_key(tick, order_index).unwrap();
        let hashed_encoded_order_key = keccak256(encoded_order_key);

        unsafe {
            storage_cache_bytes32(hashed_encoded_order_key.as_ptr(), [0u8; 32].as_ptr());
            storage_flush_cache(false);
        }
    }

    pub fn encode_order_key(&self, tick: U256, order_index: U256) -> Result<Vec<u8>, Vec<u8>> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&tick.to_be_bytes::<32>());
        encoded.extend_from_slice(b"-");
        encoded.extend_from_slice(&order_index.to_be_bytes::<32>());
        Ok(encoded)
    }

    pub fn encode_order_data(&self, user: Address, volume: U256) -> Result<[u8; 32], Vec<u8>> {
        let mut encoded = [0u8; 32];
        encoded[..20].copy_from_slice(&<[u8; 20]>::from(user));
        encoded[20..32].copy_from_slice(&volume.to_be_bytes::<32>()[20..32]);
        Ok(encoded)
    }

    pub fn decode_order_data(&self, encoded: Vec<u8>) -> Result<(Address, U256), Vec<u8>> {
        let mut user_bytes = [0u8; 20];
        user_bytes.copy_from_slice(&encoded[..20]);
        let user = Address::from(user_bytes);

        let mut volume_bytes = [0u8; 32];
        volume_bytes[20..32].copy_from_slice(&encoded[20..32]);
        let volume = U256::from_be_bytes::<32>(volume_bytes);

        Ok((user, volume))
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
