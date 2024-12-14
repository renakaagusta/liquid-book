// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;
use stylus_sdk::storage::{StorageMap, StorageU256};
use stylus_sdk::{alloy_primitives::U256, console, prelude::*, stylus_proc::entrypoint};

pub mod maths;
use maths::tick_bitmap::TickBitmap;

#[storage]
#[entrypoint]
pub struct BitmapStorage {
    storage: StorageMap<i16, StorageU256>,
}

#[public]
impl BitmapStorage {
    pub fn position(&mut self, tick: i32) -> (i16, u8) {
        let (word_pos, bit_pos) = TickBitmap::position(tick);
        (word_pos, bit_pos)
    }

    pub fn flip(&mut self, tick: i32) -> (i16, u8) {
        TickBitmap::flip_tick(&mut self.storage, tick, 1);
        return self.position(tick);
    }

    fn get_bitmap(&mut self, index: i16) {
        let bitmap = self.storage.get(index);
        console!("{:b}", bitmap);
    }

    pub fn next_tick(&mut self, tick: i32, lte: bool) -> (i32, bool) {
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
