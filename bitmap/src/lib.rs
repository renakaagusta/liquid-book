// Allow cargo stylus export-abi to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_sol_macro::sol;
use stylus_sdk::storage::{StorageMap, StorageU256};
use stylus_sdk::{
    alloy_primitives::{U256},
    prelude::*,
    stylus_proc::entrypoint,
    evm,
    console,
};

pub mod maths;
use maths::tick_bitmap::TickBitmap;

#[storage]
#[entrypoint]
pub struct BitmapManager {
    storage: StorageMap<i16, StorageU256>,
    current_tick: StorageU256,
}

sol! {
    event SetCurrentTick(uint256 indexed tick);
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

    pub fn get_current_tick(&self) -> U256 {
        console!("BITMAP :: current tick: {}", self.current_tick.get());

        U256::from(self.current_tick.get())
    }

    pub fn set_current_tick(&mut self, tick: U256)  -> Result<U256, Vec<u8>> {
        self.current_tick.set(U256::from(tick));
        
        console!("BITMAP :: set current tick: {}", tick);

        evm::log(SetCurrentTick {
            tick: tick
        });

        Ok(tick)
    }

    pub fn top_n_best_ticks(&self, is_buy: bool) -> Vec<U256> {
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

            if counter == U256::from(5) {
                break;
            }
        }

        console!("BITMAP :: best ticks: {:?}", best_ticks);

        best_ticks
    }

    pub fn flip(&mut self, tick: i32) -> (i16, u8) {
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
