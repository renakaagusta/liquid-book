#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_primitives::Address;
use stylus_sdk::{alloy_primitives::U256, console, msg, prelude::*};

sol_storage! {
    #[entrypoint]
    pub struct LiquidBookEngine {
        address bitmap_manager_address;
        uint256 next_order_id;
        mapping(uint256 => Order) orders;
        mapping(uint256 => uint256[]) tick_orders;
        mapping(uint256 => Tick) ticks;
    }

    pub struct Order {
        address trader;
        uint256 amount;
        uint256 tick;
        bool is_buy;
        bool is_active;
    }

    pub struct Tick {
        uint256 buy_index;
        uint256 sell_index;
        uint256 volume;
    }
}

sol_interface! {
    interface IBitmapStorage {
        function flip(int32 tick) external returns (int16, uint8);
        function topNBestTicks(bool is_buy) external view returns (uint256[5] memory);
        function setCurrentTick(uint256 tick) external;
    }
}

#[public]

impl LiquidBookEngine {
    pub fn initialize(&mut self, bitmap_manager_address: Address) {
        self.bitmap_manager_address.set(bitmap_manager_address);
    }

    fn get_order(&self, id: U256) -> (Address, U256, U256, bool, bool) {
        let order = self.orders.get(id);
        (
            order.trader.get(),
            order.amount.get(),
            order.tick.get(),
            order.is_buy.get(),
            order.is_active.get(),
        )
    }

    fn get_order_tick(&self, tick: U256) -> (U256, U256, U256) {
        (
            self.ticks.get(tick).buy_index.get(),
            self.ticks.get(tick).sell_index.get(),
            self.ticks.get(tick).volume.get(),
        )
    }

    fn place_limit_order(&mut self, tick: U256, amount: U256, is_buy: bool) -> U256 {
        let id = self.next_order_id.get();
        self.orders.setter(id).trader.set(msg::sender());
        self.orders.setter(id).amount.set(amount);
        self.orders.setter(id).tick.set(tick);
        self.orders.setter(id).is_buy.set(is_buy);
        self.orders.setter(id).is_active.set(true);
        self.next_order_id.set(id + U256::from(1));
        self.tick_orders.setter(tick).push(id);
        self.orders.get(id);
        self.execute_match(tick);

        id
    }

    fn cancel_order(&mut self, id: U256) {
        let order_tick;
        let order_amount;
        let order_tick_volume;

        {
            let order = self.orders.get(id);
            order_tick = order.tick.get();
            order_amount = order.amount.get();
            order_tick_volume = self.ticks.get(order_tick).volume.get();
        }

        self.orders.setter(id).is_active.set(false);
        self.ticks
            .setter(order_tick)
            .volume
            .set(order_tick_volume - order_amount);
    }
}

impl LiquidBookEngine {
    fn execute_match(&mut self, tick: U256) {
        let orders = self.tick_orders.get(tick);
        let initial_volume = self.ticks.get(tick).volume.get();
        let mut updates: Vec<(_, _, bool)> = Vec::new();
        let start_index = self.ticks.get(tick);
        let mut sell_start_index = start_index.sell_index.get().to::<usize>();
        let mut buy_start_index = start_index.buy_index.get().to::<usize>();
        let mut has_match_order = false;

        for sell_index in sell_start_index..orders.len() {
            let sell_order_id = orders.get(U256::from(sell_index)).unwrap();
            let sell_order = self.orders.get(sell_order_id);

            if sell_order.is_buy.get() == true || !sell_order.is_active.get() {
                continue;
            }

            let mut sell_remaining_amount = sell_order.amount.get();

            for buy_index in buy_start_index..orders.len() {
                let buy_order_id = orders.get(U256::from(buy_index)).unwrap();
                let buy_order = self.orders.get(buy_order_id);

                if !buy_order.is_buy.get() || !buy_order.is_active.get() {
                    continue;
                }

                let matched_amount = if sell_remaining_amount > buy_order.amount.get() {
                    buy_order.amount.get()
                } else {
                    sell_remaining_amount
                };

                has_match_order = true;

                let updated_amount = buy_order.amount.get() - matched_amount;

                if updated_amount == U256::ZERO {
                    buy_start_index = buy_index;
                }

                updates.push((buy_order_id, updated_amount, updated_amount > U256::ZERO));
                sell_remaining_amount -= matched_amount;

                if sell_remaining_amount == U256::ZERO {
                    sell_start_index = sell_index;
                    break;
                }
            }

            updates.push((
                sell_order_id,
                sell_remaining_amount,
                sell_remaining_amount > U256::ZERO,
            ));
        }

        self.ticks
            .setter(tick)
            .sell_index
            .set(U256::from(sell_start_index));
        self.ticks
            .setter(tick)
            .buy_index
            .set(U256::from(buy_start_index));

        for (order_id, amount, is_active) in updates {
            self.orders.setter(order_id).amount.set(amount);
            self.orders.setter(order_id).is_active.set(is_active);
        }

        let mut latest_volume = U256::ZERO;

        for sell_index in sell_start_index..orders.len() {
            let order_id = orders.get(U256::from(sell_index)).unwrap();
            let sell_order = self.orders.get(order_id);

            if sell_order.is_buy.get() || !sell_order.is_active.get() {
                continue;
            }

            latest_volume += sell_order.amount.get();
        }

        for buy_index in buy_start_index..orders.len() {
            let order_id = orders.get(U256::from(buy_index)).unwrap();
            let buy_order = self.orders.get(order_id);

            if !buy_order.is_buy.get() || !buy_order.is_active.get() {
                continue;
            }

            latest_volume += buy_order.amount.get();
        }

        self.ticks.setter(tick).volume.set(latest_volume);
        let bitmap_manager = IBitmapStorage::new(self.bitmap_manager_address.get());

        if has_match_order {
            let _ = bitmap_manager.set_current_tick(&mut *self, U256::from(tick));
        }

        if initial_volume == U256::ZERO && latest_volume != U256::ZERO
            || initial_volume == U256::ZERO && latest_volume == U256::ZERO
            || initial_volume != U256::ZERO && latest_volume == U256::ZERO
        {
            let converted_tick: i32 = tick.try_into().unwrap();
            let _ = bitmap_manager.flip(self, converted_tick);
        }
    }

    pub fn match_market_order(&mut self) {
        //TODO
    }

    // pub fn match_market_order(
    //     &mut self,
    //     incoming_order_tick: U256,
    //     incoming_order_volume: U256,
    //     incoming_order_user: Address,
    //     incoming_order_is_buy: bool,
    //     incoming_order_is_market: bool,
    // ) {
    //     let tick_manager = ITickManager::new(self.tick_manager_address.get());
    //     let order_manager = IOrderManager::new(self.order_manager_address.get());
    //     let bitmap_manager = IBitmapStorage::new(self.bitmap_manager_address.get());

    //     let mut remaining_incoming_order_volume: alloy_primitives::Uint<256, 4> =
    //         incoming_order_volume;
    //     let possible_ticks = bitmap_manager
    //         .top_n_best_ticks(&*self, incoming_order_is_buy)
    //         .unwrap();

    //     let filtered_possible_ticks: Vec<U256> = if incoming_order_is_market {
    //         possible_ticks.clone()
    //     } else if incoming_order_is_buy {
    //         possible_ticks
    //             .iter()
    //             .filter(|tick| incoming_order_tick > **tick)
    //             .cloned()
    //             .collect()
    //     } else {
    //         possible_ticks
    //             .iter()
    //             .filter(|tick| incoming_order_tick < **tick)
    //             .cloned()
    //             .collect()
    //     };

    //     if !filtered_possible_ticks.is_empty() {
    //         let mut last_tick = U256::from(0);

    //         for tick in filtered_possible_ticks {
    //             let tick_data = tick_manager.get_tick_data(&*self, tick).unwrap();
    //             let (start_index, _, volume, _) = tick_data;

    //             let mut orders: Vec<(U256, U256, U256)> = Vec::new();

    //             if volume != U256::ZERO {
    //                 let mut index = start_index % U256::from(256);

    //                 loop {
    //                     let order = order_manager
    //                         .read_order(&*self, tick, U256::from(index))
    //                         .unwrap();
    //                     let (_, order_volume) = order;

    //                     if order_volume != U256::ZERO {
    //                         orders.push((tick, U256::from(index), order_volume));
    //                         index = (index + U256::from(1)) % U256::from(256);
    //                     } else {
    //                         break;
    //                     }
    //                 }
    //             }

    //             if !orders.is_empty() {
    //                 remaining_incoming_order_volume =
    //                     self.execute_match(orders, remaining_incoming_order_volume);
    //             }

    //             if remaining_incoming_order_volume == U256::ZERO {
    //                 break;
    //             }

    //             last_tick = tick;
    //         }

    //         if remaining_incoming_order_volume != U256::ZERO {
    //             // TODO
    //             // let _ = storage.set_current_tick(self, last_tick);
    //             let _ = order_manager.insert_order(
    //                 self,
    //                 last_tick,
    //                 U256::from(remaining_incoming_order_volume),
    //                 incoming_order_user,
    //                 incoming_order_is_buy,
    //             );
    //         }
    //     } else {
    //         let current_tick = tick_manager.get_current_tick(&*self).unwrap();
    //         let _ = order_manager.insert_order(
    //             self,
    //             current_tick,
    //             U256::from(remaining_incoming_order_volume),
    //             incoming_order_user,
    //             incoming_order_is_buy,
    //         );
    //     }
    // }
}
