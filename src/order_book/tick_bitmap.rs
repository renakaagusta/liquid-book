use alloy_primitives::U256;

use super::bit_math::{least_significant_bit, most_significant_bit};
pub struct TickBitmap;

pub type TickBitmapStorage = stylus_sdk::storage::StorageMap<i16, stylus_sdk::storage::StorageU256>;

impl TickBitmap {
    pub fn position(tick: i32) -> (i16, u8) {
        let word_pos = (tick >> 8) as i16;
        let bit_pos = (tick % 256) as u8;
        (word_pos, bit_pos)
    }

    pub fn flip_tick(bitmap_storage: &mut TickBitmapStorage, tick: i32, tick_spacing: i32) {
        assert_eq!(tick % tick_spacing, 0);

        let (word_pos, bit_pos) = Self::position(tick / tick_spacing);
        let mask = U256::from(1) << bit_pos;
        let bitmap = bitmap_storage.get(word_pos) ^ mask;
        bitmap_storage.setter(word_pos).set(bitmap);
    }

    pub fn next_initialized_tick_within_one_word(
        bitmap_storage: &TickBitmapStorage,
        tick: i32,
        tick_spacing: i32,
        lte: bool,
    ) -> (i32, bool) {
        let mut compressed = tick / tick_spacing;

        if tick < 0 && tick % tick_spacing != 0 {
            compressed -= 1;
        }

        if lte {
            let (word_pos, bit_pos) = Self::position(compressed);
            let mask = (U256::from(1) << bit_pos) - U256::from(1) + (U256::from(1) << bit_pos);
            let masked = bitmap_storage.get(word_pos) & mask;

            let initialized = masked != U256::from(0);
            let next = if initialized {
                (compressed - (bit_pos as i32 - most_significant_bit(masked).unwrap() as i32))
                    * tick_spacing
            } else {
                (compressed - bit_pos as i32) * tick_spacing
            };

            (next, initialized)
        } else {
            let (word_pos, bit_pos) = Self::position(compressed + 1);
            let mask = !((U256::from(1) << bit_pos) - U256::from(1));
            let masked = bitmap_storage.get(word_pos) & mask;

            let initialized = masked != U256::from(0);
            let next = if initialized {
                (compressed + 1 + (least_significant_bit(masked).unwrap() as i32 - bit_pos as i32))
                    * tick_spacing
            } else {
                (compressed + 1 + (u8::MAX as i32 - bit_pos as i32)) * tick_spacing
            };

            (next, initialized)
        }
    }
}
