// Allow cargo stylus export-abi to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use core::ops::ShrAssign;
use crate::alloc::string::ToString;
use alloy_sol_macro::sol;
use stylus_sdk::storage::{StorageMap, StorageU256, StorageI128};
use stylus_sdk::{
    alloy_primitives::{U256, I128},
    prelude::*,
    stylus_proc::entrypoint,
    evm,
    console
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
        console!("BITMAP :: current tick: {}", self.current_tick.get());

        self
            .current_tick
            .get()
            .to_string()
            .parse::<i128>()
            .unwrap_or(0)
    }

    pub fn set_current_tick(&mut self, tick: i128) -> i128 {
        self.current_tick.set(tick.to_string().parse::<I128>().unwrap());
        
        // console!("BITMAP :: set current tick: {}", tick);

        evm::log(SetCurrentTick {
            tick: tick
        });

        tick
    }

    pub fn log(self, value: i32) {
        console!("BITMAP :: log :: value: {}", value);
    }

    pub fn top_n_best_ticks(&self, is_buy: bool) -> Vec<i128> {
        let mut counter = U256::from(0);
        let mut best_ticks: Vec<i128> = Vec::new();
        let mut current_tick = self
            .current_tick
            .get()
            .to_string()
            .parse::<i32>()
            .unwrap_or(0);

        // console!("BITMAP :: current tick: {:?}", current_tick);

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

        console!("BITMAP :: best ticks: {:?} {:?}", is_buy, best_ticks);

        best_ticks
    }

    pub fn flip(&mut self, tick: i32) -> (i16, u8) {
        console!("BITMAP :: flip: {}", tick);

        TickBitmap::flip_tick(&mut self.storage, tick, 1);

        evm::log(FlipTick {
            tick: tick
        });

        self.position(tick)
    }

    fn get_bitmap(&mut self, index: i16) {
        let bitmap = self.storage.get(index);
        console!("BITMAP :: bitmap: {:b}", bitmap);
    }

    pub fn next_tick(&self, tick: i32, lte: bool) -> (i32, bool) {
        let (next, initialized) =
            TickBitmap::next_initialized_tick_within_one_word(&self.storage, tick, 1, lte);
        (next, initialized)
    }
}