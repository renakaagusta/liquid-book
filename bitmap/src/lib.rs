// Allow cargo stylus export-abi to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
use alloy_primitives::{Address, U16};
use stylus_sdk::storage::{StorageAddress, StorageMap, StorageU16, StorageU256};
use stylus_sdk::{
    alloy_primitives::{U128, U256},
    console,
    prelude::*,
    stylus_proc::entrypoint,
};

pub mod maths;
use maths::tick_bitmap::TickBitmap;

#[storage]
#[entrypoint]
pub struct BitmapManager {
    storage: StorageMap<i16, StorageU256>,
    current_tick: StorageU256,
}

sol_interface! {
    interface ITickManager {
        function getCurrentTick() external view returns (uint256);
    }
}

#[public]
impl BitmapManager {
    pub fn position(&mut self, tick: i32) -> (i16, u8) {
        let (word_pos, bit_pos) = TickBitmap::position(tick);
        (word_pos, bit_pos)
    }

    pub fn get_current_tick(&self) -> U256 {
        U256::from(self.current_tick.get())
    }

    pub fn set_current_tick(&mut self, tick: U256) {
        self.current_tick.set(U256::from(tick));
    }

    pub fn top_n_best_ticks(&self, is_buy: bool) -> Result<Vec<U256>, Vec<u8>> {
        let mut counter = U256::from(0);
        let mut best_ticks: Vec<U256> = Vec::new();
        let mut current_tick = self
            .current_tick
            .get()
            .to_string()
            .parse::<i32>()
            .unwrap_or(0);

        loop {
            let (next_tick, initialized) = self.next_tick(current_tick, !is_buy);

            if initialized {
                best_ticks.push(U256::from(next_tick));
            }

            current_tick = if !is_buy { next_tick - 1 } else { next_tick };
            counter += U256::from(1);
            console!("current_tick: {}", current_tick);
            console!("counter: {}", counter);

            if counter == U256::from(5) {
                break;
            }
        }

        Ok(best_ticks)
    }

    pub fn flip(&mut self, tick: i32) -> (i16, u8) {
        TickBitmap::flip_tick(&mut self.storage, tick, 1);
        return self.position(tick);
    }

    fn get_bitmap(&mut self, index: i16) {
        let bitmap = self.storage.get(index);
        console!("{:b}", bitmap);
    }

    pub fn next_tick(&self, tick: i32, lte: bool) -> (i32, bool) {
        let (next, initialized) =
            TickBitmap::next_initialized_tick_within_one_word(&self.storage, tick, 1, lte);
        (next, initialized)
    }

    //TODO: remove test function
    pub fn test_bitmap(&mut self) {
        assert_eq!(TickBitmap::position(0), (0, 0));
        assert_eq!(TickBitmap::position(256), (1, 0));
        assert_eq!(TickBitmap::position(257), (1, 1));

        assert_eq!(TickBitmap::position(-1), (-1, 255));
        assert_eq!(TickBitmap::position(-256), (-1, 0));
        assert_eq!(TickBitmap::position(-257), (-2, 255));

        console!("test_position passed");

        let mut masked = U256::from(1);

        TickBitmap::flip_tick(&mut self.storage, 0, 1);
        assert_eq!(self.storage.get(0), masked);

        TickBitmap::flip_tick(&mut self.storage, 253, 1);
        masked = masked | masked << 253;
        assert_eq!(self.storage.get(0), masked);

        TickBitmap::flip_tick(&mut self.storage, 254, 1);
        masked = masked | masked << 254;
        assert_eq!(self.storage.get(0), masked);

        TickBitmap::flip_tick(&mut self.storage, 255, 1);
        masked = masked | masked << 255;
        assert_eq!(self.storage.get(0), masked);

        TickBitmap::flip_tick(&mut self.storage, -1, 1);
        assert_eq!(self.storage.get(-1), U256::from(1) << 255);

        console!("test flip passed");
    }
}
