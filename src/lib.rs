// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{keccak256, Address, U128, U256},
    console,
    hostio::{storage_cache_bytes32, storage_flush_cache, storage_load_bytes32},
    prelude::*,
};

sol_storage! {
    #[entrypoint]
    pub struct OrderBook {
        uint128 current_tick;
        uint128 tick_spacing;
        uint256 buy_volume;
        uint256 sell_volume;
        mapping(uint128 => Order[]) tick_orders;
        mapping(uint128 => Tick) ticks;
    }

    pub struct Order {
        address user;
        uint128 quantity;
    }

    pub struct Tick {
        uint128 start_index;
        uint128 length;
        uint128 volume;
        bool is_buy;
    }
}

#[public]
impl OrderBook {
    pub fn add_order_to_orderbook(&mut self, quantity: U256, tick: U256, is_buy: bool) {
        let mut orders = if is_buy {
            self.buy_volume.set(tick * quantity);
            self.tick_orders.setter(U128::from(tick))
        } else {
            self.sell_volume.set(tick * quantity);
            self.tick_orders.setter(U128::from(tick))
        };

        let mut order = orders.grow();
        order.quantity.set(U128::from(quantity));
    }

    pub fn read_order(&self, tick: U256, order_index: U256) -> Result<(Address, U256), Vec<u8>> {
        let encoded_order_key = self.encode_order_key(tick, order_index).unwrap();
        let hashed_encoded_order_key = keccak256(encoded_order_key);

        let mut buffer_order_data: [u8; 32] = [0u8; 32];  

        unsafe {
            storage_load_bytes32(hashed_encoded_order_key.as_ptr(), buffer_order_data.as_mut_ptr());
        }

        let encoded_order_data = buffer_order_data.to_vec();
        let decoded_order_data = self.decode_order_data(encoded_order_data);

        Ok(decoded_order_data.unwrap())
    }

    pub fn write_order(
        &mut self,
        tick: U256,
        order_index: U256,
        user: Address,
        quantity: U256,
    ) {
        let encoded_order_key = self.encode_order_key(tick, order_index).unwrap();
        let encoded_order_data = self.encode_order_data(user, quantity).unwrap();

        let hashed_encoded_order_key = keccak256(encoded_order_key);

        unsafe {
            storage_cache_bytes32(hashed_encoded_order_key.as_ptr(), encoded_order_data.as_ptr());
            storage_flush_cache(false);
        }
    }

    pub fn encode_order_key(&self, tick: U256, order_index: U256) -> Result<Vec<u8>, Vec<u8>> {
        let mut encoded = Vec::new();
        encoded.extend_from_slice(&tick.to_be_bytes::<32>());
        encoded.extend_from_slice(b"-");
        encoded.extend_from_slice(&order_index.to_be_bytes::<32>());
        Ok(encoded)
    }

    pub fn encode_order_data(&self, user: Address, quantity: U256) -> Result<[u8; 32], Vec<u8>> {
        let mut encoded = [0u8; 32];
        encoded[..20].copy_from_slice(&<[u8; 20]>::from(user));
        encoded[20..32].copy_from_slice(&quantity.to_be_bytes::<32>()[20..32]);
        Ok(encoded)
    }

    pub fn decode_order_data(&self, encoded: Vec<u8>) -> Result<(Address, U256), Vec<u8>> {
        let mut user_bytes = [0u8; 20];
        user_bytes.copy_from_slice(&encoded[..20]);
        let user = Address::from(user_bytes);

        let mut quantity_bytes = [0u8; 32];
        quantity_bytes[20..32].copy_from_slice(&encoded[20..32]);
        let quantity = U256::from_be_bytes::<32>(quantity_bytes);
    
        Ok((user, quantity))
    }

    // pub fn match_market_order(
    //     &mut self,
    //     incoming_order_quantity: U256,
    //     incoming_order_tick: U256,
    //     incoming_order_is_buy: bool,
    // ) {
    //     let mut remaining_incoming_order_quantity = incoming_order_quantity;
    //     if incoming_order_is_buy {
    //         let possible_ticks = self.top_n_best_sell_ticks();

    //         if possible_ticks.is_empty() {
    //             for tick in possible_ticks {
    //                 let mut sell_quantity = U128::from(0);
    //                 let initial_remaining_incoming_order_quantity =
    //                     U128::from(remaining_incoming_order_quantity);
    //                 let mut orders: Vec<(U256, U256, U256)> = Vec::new();

    //                 {
    //                     let tick_orders = self.tick_orders.get(U128::from(tick));
    //                     if !tick_orders.is_empty() {
    //                         for index in 0..tick_orders.len() {
    //                             let sell_order = tick_orders.get(index).unwrap();
    //                             sell_quantity += sell_order.quantity.get();
    //                             orders.push((
    //                                 U256::from(index),
    //                                 tick,
    //                                 U256::from(sell_order.quantity.get()),
    //                             ));
    //                         }
    //                     }
    //                 }

    //                 if !orders.is_empty() {
    //                     remaining_incoming_order_quantity = self.execute_match(
    //                         orders,
    //                         remaining_incoming_order_quantity,
    //                         incoming_order_is_buy,
    //                     );
    //                 }

    //                 let mut tick_data = self.ticks.setter(U128::from(tick));
    //                 if initial_remaining_incoming_order_quantity > sell_quantity {
    //                     tick_data.is_buy.set(true);
    //                     tick_data.volume.set(U128::from(0));
    //                 } else if initial_remaining_incoming_order_quantity < sell_quantity {
    //                     tick_data
    //                         .volume
    //                         .set(sell_quantity - initial_remaining_incoming_order_quantity);
    //                 } else {
    //                     tick_data.volume.set(U128::from(0));
    //                 }

    //                 if remaining_incoming_order_quantity == U256::ZERO {
    //                     break;
    //                 }
    //             }

    //             if remaining_incoming_order_quantity != U256::ZERO {
    //                 self.add_order_to_orderbook(
    //                     U256::from(remaining_incoming_order_quantity),
    //                     incoming_order_tick,
    //                     incoming_order_is_buy,
    //                 );
    //             }
    //         } else {
    //             self.add_order_to_orderbook(
    //                 remaining_incoming_order_quantity,
    //                 incoming_order_tick,
    //                 incoming_order_is_buy,
    //             );
    //         }
    //     } else {
    //         let possible_ticks = self.top_n_best_buy_ticks();

    //         if possible_ticks.is_empty() {
    //             for tick in possible_ticks {
    //                 let mut buy_quantity = U128::from(0);
    //                 let initial_remaining_incoming_order_quantity =
    //                     U128::from(remaining_incoming_order_quantity);
    //                 let mut orders: Vec<(U256, U256, U256)> = Vec::new();

    //                 {
    //                     let tick_orders = self.tick_orders.get(U128::from(tick));
    //                     if !tick_orders.is_empty() {
    //                         for index in 0..tick_orders.len() {
    //                             let buy_order = tick_orders.get(index).unwrap();
    //                             buy_quantity += buy_order.quantity.get();
    //                             orders.push((
    //                                 U256::from(index),
    //                                 tick,
    //                                 U256::from(buy_order.quantity.get()),
    //                             ));
    //                         }
    //                     }
    //                 }

    //                 if !orders.is_empty() {
    //                     remaining_incoming_order_quantity = self.execute_match(
    //                         orders,
    //                         remaining_incoming_order_quantity,
    //                         incoming_order_is_buy,
    //                     );
    //                 }

    //                 let mut tick_data = self.ticks.setter(U128::from(tick));
    //                 if initial_remaining_incoming_order_quantity > buy_quantity {
    //                     tick_data.is_buy.set(false);
    //                     tick_data.volume.set(U128::from(0));
    //                 } else if initial_remaining_incoming_order_quantity < buy_quantity {
    //                     tick_data
    //                         .volume
    //                         .set(buy_quantity - initial_remaining_incoming_order_quantity);
    //                 } else {
    //                     tick_data.volume.set(U128::from(0));
    //                 }

    //                 if remaining_incoming_order_quantity == U256::ZERO {
    //                     break;
    //                 }
    //             }

    //             if remaining_incoming_order_quantity != U256::ZERO {
    //                 self.add_order_to_orderbook(
    //                     U256::from(remaining_incoming_order_quantity),
    //                     incoming_order_tick,
    //                     incoming_order_is_buy,
    //                 );
    //             }
    //         } else {
    //             self.add_order_to_orderbook(
    //                 remaining_incoming_order_quantity,
    //                 incoming_order_tick,
    //                 incoming_order_is_buy,
    //             );
    //         }
    //     }
    // }

    // pub fn limit_market_order(
    //     &mut self,
    //     incoming_order_quantity: U256,
    //     incoming_order_tick: U256,
    //     incoming_order_is_buy: bool,
    // ) {
    //     let mut remaining_incoming_order_quantity = incoming_order_quantity;

    //     if incoming_order_is_buy {
    //         let possible_ticks = self.top_n_best_sell_ticks();

    //         if !possible_ticks.is_empty() {
    //             let orders_to_process: Vec<(U256, Vec<(U256, U256, U256)>)> = possible_ticks
    //                 .into_iter()
    //                 .filter(|&tick| incoming_order_tick >= tick)
    //                 .filter(|&tick| !self.tick_orders.get(U128::from(tick)).is_empty())
    //                 .map(|tick| {
    //                     let orders: Vec<(U256, U256, U256)> = (0..self
    //                         .tick_orders
    //                         .get(U128::from(tick))
    //                         .len())
    //                         .filter_map(|index| {
    //                             let tick_orders = self.tick_orders.get(U128::from(tick));
    //                             let order = tick_orders.get(index)?;
    //                             Some((U256::from(index), tick, U256::from(order.quantity.get())))
    //                         })
    //                         .collect();
    //                     (tick, orders)
    //                 })
    //                 .collect();

    //             for (_, orders) in orders_to_process {
    //                 remaining_incoming_order_quantity = self.execute_match(
    //                     orders,
    //                     remaining_incoming_order_quantity,
    //                     incoming_order_is_buy,
    //                 );

    //                 if remaining_incoming_order_quantity == U256::ZERO {
    //                     break;
    //                 }
    //             }
    //         }
    //     } else {
    //         let possible_ticks = self.top_n_best_sell_ticks();

    //         if !possible_ticks.is_empty() {
    //             let orders_to_process: Vec<(U256, Vec<(U256, U256, U256)>)> = possible_ticks
    //                 .into_iter()
    //                 .filter(|&tick| incoming_order_tick >= tick)
    //                 .filter(|&tick| !self.tick_orders.get(U128::from(tick)).is_empty())
    //                 .map(|tick| {
    //                     let orders: Vec<(U256, U256, U256)> = (0..self
    //                         .tick_orders
    //                         .get(U128::from(tick))
    //                         .len())
    //                         .filter_map(|index| {
    //                             let tick_orders = self.tick_orders.get(U128::from(tick));
    //                             let order = tick_orders.get(index)?;
    //                             Some((U256::from(index), tick, U256::from(order.quantity.get())))
    //                         })
    //                         .collect();
    //                     (tick, orders)
    //                 })
    //                 .collect();

    //             for (_, orders) in orders_to_process {
    //                 remaining_incoming_order_quantity = self.execute_match(
    //                     orders,
    //                     remaining_incoming_order_quantity,
    //                     incoming_order_is_buy,
    //                 );

    //                 if remaining_incoming_order_quantity == U256::ZERO {
    //                     break;
    //                 }
    //             }
    //         }
    //     }

    //     if remaining_incoming_order_quantity != U256::ZERO {
    //         self.add_order_to_orderbook(
    //             remaining_incoming_order_quantity,
    //             incoming_order_tick,
    //             incoming_order_is_buy,
    //         );
    //     }
    // }
}
