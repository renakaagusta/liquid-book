use bitmap::bitmap_storage::function;
use bitmap::order_book::tick_bitmap::{self, TickBitmap, TickBitmapStorage};
use stylus_sdk::alloy_primitives::U256;
#[cfg(test)]
mod tests {
    use super::*;
    use bitmap::OrderBook;

    #[test]
    fn test_position() {
        assert_eq!(TickBitmap::position(0), (0, 0));
        assert_eq!(TickBitmap::position(256), (1, 0));
        assert_eq!(TickBitmap::position(257), (1, 1));

        assert_eq!(TickBitmap::position(-1), (-1, 255));
        assert_eq!(TickBitmap::position(-256), (-1, 0));
        assert_eq!(TickBitmap::position(-257), (-2, 255));

        function();
    }

    #[motsu::test]
    fn testOrder(order_book: OrderBook) {
        order_book.flip(0);
    }

    #[motsu::test]
    fn uri_ignores_token_id() {
        assert_eq!(1, 1);
    }

    #[motsu::test]
    fn interface_id() {
        assert_eq!(1, 1);
    }
}
