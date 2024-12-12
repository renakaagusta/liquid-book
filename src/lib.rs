// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
use alloy_primitives::{Signed, I16};
use stylus_sdk::evm;
use stylus_sdk::storage::{StorageI16, StorageMap, StorageU256};
use stylus_sdk::{
    alloy_primitives::{U128, U256},
    console,
    prelude::*,
    stylus_proc::entrypoint,
    ArbResult,
};

pub mod order_book;
use order_book::tick_bitmap::{self, TickBitmap, TickBitmapStorage};

#[storage]
#[entrypoint]
pub struct OrderBook {
    bitmap_storage: StorageMap<i16, StorageU256>,
}

#[public]
impl OrderBook {
    fn flip(&mut self, tick: i32) {
        TickBitmap::flip_tick(&mut self.bitmap_storage, tick, 1);
    }

    fn next_tick(&mut self, tick: i32, lte: bool) {
        let (next, initialized) =
            TickBitmap::next_initialized_tick_within_one_word(&self.bitmap_storage, tick, 1, lte);
        console!("next: {}, initialized: {}", next, initialized);
    }

    fn get_bitmap(&mut self, index: i16) {
        let bitmap = self.bitmap_storage.get(index);
        console!("{:b}", bitmap);
    }

    fn test_bitmap(&mut self) {
        assert_eq!(TickBitmap::position(0), (0, 0));
        assert_eq!(TickBitmap::position(256), (1, 0));
        assert_eq!(TickBitmap::position(257), (1, 1));

        assert_eq!(TickBitmap::position(-1), (-1, 255));
        assert_eq!(TickBitmap::position(-256), (-1, 0));
        assert_eq!(TickBitmap::position(-257), (-2, 255));

        console!("test_position passed");

        let mut masked = U256::from(1);

        TickBitmap::flip_tick(&mut self.bitmap_storage, 0, 1);
        assert_eq!(self.bitmap_storage.get(0), masked);

        TickBitmap::flip_tick(&mut self.bitmap_storage, 253, 1);
        masked = masked | masked << 253;
        assert_eq!(self.bitmap_storage.get(0), masked);

        TickBitmap::flip_tick(&mut self.bitmap_storage, 254, 1);
        masked = masked | masked << 254;
        assert_eq!(self.bitmap_storage.get(0), masked);

        TickBitmap::flip_tick(&mut self.bitmap_storage, 255, 1);
        masked = masked | masked << 255;
        assert_eq!(self.bitmap_storage.get(0), masked);

        TickBitmap::flip_tick(&mut self.bitmap_storage, -1, 1);
        assert_eq!(self.bitmap_storage.get(-1), U256::from(1) << 255);

        console!("test flip passed");
    }
}
