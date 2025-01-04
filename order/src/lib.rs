// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main, no_std)]
extern crate alloc;

use alloc::vec::Vec;
use alloy_sol_macro::sol;
use stylus_sdk::{
    alloy_primitives::{keccak256, Address, U256},
    hostio::{storage_cache_bytes32, storage_flush_cache, storage_load_bytes32},
    prelude::*,
    evm
};

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

sol! {
    event InsertOrder(address indexed user, int128 indexed tick, uint256 indexed order_index, bool is_buy, uint256 volume);
    event UpdateOrder(int128 indexed tick, uint256 indexed order_index, uint256 volume);
}

sol_storage! {
    #[entrypoint]
    pub struct OrderManager {
        address engine_address;
        address tick_manager_address;
        address bitmap_manager_address;
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
    interface ITickManager {
        function setTickData(int128 tick, uint256 volume, bool is_buy, bool is_existing_order) external;
        function getTickData(int128 tick) external view returns (uint256, uint256, uint256, bool);
        function getCurrentTick() external view returns (uint256);
        function setCurrentTick(int128 tick) external returns (uint256);
    }
}

#[public]
impl OrderManager {
    pub fn initialize(
        &mut self,
        engine_address: Address,
        bitmap_manager_address: Address,
        tick_manager_address: Address,
    ) {
        self.engine_address.set(engine_address);
        self.bitmap_manager_address.set(bitmap_manager_address);
        self.tick_manager_address.set(tick_manager_address);
    }

    pub fn insert_order(&mut self, tick: i128, volume: U256, user: Address, is_buy: bool) -> U256 {
        let tick_manager_address = self.tick_manager_address.get();
        let tick_manager = ITickManager::new(tick_manager_address);

        let (start_index, length, tick_volume, tick_is_buy) =
            tick_manager.get_tick_data(&*self, tick).unwrap();

        let order_index = start_index + length % U256::from(256);

        self.write_order(tick, order_index, user, volume);
        tick_manager.set_tick_data(self, tick, volume, is_buy, false);
    
        evm::log(InsertOrder {
            user: user,
            tick: tick,
            order_index: order_index,
            is_buy: is_buy,
            volume: volume,
        });

        order_index
        // U256::from(0)
    }

    pub fn update_order(&mut self, tick: i128, volume: U256, order_index: U256) {
        let tick_manager = ITickManager::new(self.tick_manager_address.get());
        let order_data = self.read_order(tick, order_index).unwrap();

        if volume == U256::ZERO {
            self.delete_order(tick, order_index);
        } else {
            self.write_order(tick, U256::from(order_index), order_data.0, volume);
        }

        tick_manager.set_tick_data(self, tick, volume, false, true);

        evm::log(UpdateOrder {
            tick: tick,
            order_index: order_index,
            volume: volume,
        });
    }

    pub fn read_order(&self, tick: i128, order_index: U256) -> Result<(Address, U256), Vec<u8>> {
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

    pub fn write_order(&mut self, tick: i128, order_index: U256, user: Address, volume: U256) {
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

    pub fn delete_order(&mut self, tick: i128, order_index: U256) {
        let encoded_order_key = self.encode_order_key(tick, order_index).unwrap();
        let hashed_encoded_order_key = keccak256(encoded_order_key);

        unsafe {
            storage_cache_bytes32(hashed_encoded_order_key.as_ptr(), [0u8; 32].as_ptr());
            storage_flush_cache(false);
        }
    }

    pub fn encode_order_key(&self, tick: i128, order_index: U256) -> Result<Vec<u8>, Vec<u8>> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&tick.to_be_bytes());
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
}
