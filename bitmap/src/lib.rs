// Allow cargo stylus export-abi to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use crate::alloc::string::ToString;
use alloy_sol_macro::sol;
use stylus_sdk::storage::{StorageI128, StorageMap, StorageU256};
use stylus_sdk::{
    alloy_primitives::{I128, U256},
    console, evm,
    prelude::*,
    stylus_proc::entrypoint,
};

pub mod maths;
use maths::tick_bitmap::TickBitmap;

#[storage]
#[entrypoint]
pub struct BitmapManager {
    storage: StorageMap<i16, StorageU256>,
    current_tick: StorageI128,
}

sol! {
    event SetCurrentTick(int128 indexed tick);
    event FlipTick(int32 indexed tick);
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

    pub fn get_current_tick(&self) -> i128 {
        self.current_tick
            .get()
            .to_string()
            .parse::<i128>()
            .unwrap_or(0)
    }

    pub fn set_current_tick(&mut self, tick: i128) -> i128 {
        self.current_tick
            .set(tick.to_string().parse::<I128>().unwrap());

        evm::log(SetCurrentTick { tick: tick });

        tick
    }

    // pub fn log(&self, value: i32) {
    //     console!("BITMAP :: log :: value: {}", value);
    // }

    pub fn top_n_best_ticks(&self, is_buy: bool) -> Vec<i128> {
        let mut counter = U256::from(0);
        let mut best_ticks: Vec<i128> = Vec::new();
        let mut current_tick = self
            .current_tick
            .get()
            .to_string()
            .parse::<i32>()
            .unwrap_or(0);

        loop {
            let (next_tick, initialized) = self.next_tick(current_tick, !is_buy);

            if initialized {
                best_ticks.push(i128::from(next_tick));
            }

            current_tick = if !is_buy { next_tick - 1 } else { next_tick };
            counter += U256::from(1);

            if counter == U256::from(5) {
                break;
            }
        }

        // console!("BITMAP :: best ticks: {:?} {:?}", is_buy, best_ticks);

        best_ticks
    }

    pub fn flip(&mut self, tick: i32) -> (i16, u8) {
        TickBitmap::flip_tick(&mut self.storage, tick, 1);

        evm::log(FlipTick { tick: tick });

        self.position(tick)
    }

    fn get_bitmap(&mut self, index: i16) -> U256 {
        self.storage.get(index)
        // console!("BITMAP :: bitmap: {:b}", bitmap);
    }

    pub fn next_tick(&self, tick: i32, lte: bool) -> (i32, bool) {
        let (next, initialized) =
            TickBitmap::next_initialized_tick_within_one_word(&self.storage, tick, 1, lte);
        (next, initialized)
    }

    pub fn convert_from_tick_to_price(tick: i128) -> U256 {
        let base: u128 = 1_000_100; // 1.0001 * 10^6
        let scale: u128 = 1_000_000; // 10^6

        let mut result: u128 = scale;
        for _ in 0..tick {
            result = (result * base) / scale;
        }
        U256::from(result / scale)
    }
}
