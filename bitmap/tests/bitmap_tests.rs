use liquid_book::maths::tick_bitmap::{TickBitmap, TickBitmapStorage};
use stylus_sdk::alloy_primitives::U256;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position() {
        assert_eq!(TickBitmap::position(0), (0, 0));
        assert_eq!(TickBitmap::position(256), (1, 0));
        assert_eq!(TickBitmap::position(257), (1, 1));

        assert_eq!(TickBitmap::position(-1), (-1, 255));
        assert_eq!(TickBitmap::position(-256), (-1, 0));
        assert_eq!(TickBitmap::position(-257), (-2, 255));
    }
}
