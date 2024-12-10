// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{U128, U256},
    console, hostio,
    prelude::*,
    stylus_proc::entrypoint,
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
        order.is_buy.set(is_buy);
    }

    pub fn orders(&self) -> Vec<(U256, bool)> {
        let mut orders: Vec<(U256, bool)> = Vec::new();
        let tick_orders_len: usize = self.tick_orders.get(U128::from(10)).len();
        for index in 0..tick_orders_len {
            let current_tick = self.current_tick.get();
            let tick_orders = self.tick_orders.get(U128::from(current_tick));
            let buy_order = tick_orders.get(index).unwrap();
            orders.push((U256::from(buy_order.quantity.get()), buy_order.is_buy.get()));
        }

        orders
    }

    pub fn current_tick(&self) -> U256 {
        console!("tick: {}", self.current_tick.get());
        U256::from(self.current_tick.get())
    }

    pub fn set_current_tick(&mut self, current_tick: U256) {
        console!("tick: {}", self.current_tick.get());
        self.current_tick.set(U128::from(current_tick));
    }

    pub fn best_buy_tick(&self) -> U256 {
        let mut counter = 0;
        let mut best_tick = U256::from(0);

        loop {
            counter += 1;

            let tick_orders_len: usize = self
                .tick_orders
                .get(self.current_tick.get() - (U128::from(counter) * self.tick_spacing.get()))
                .len();

            if tick_orders_len != 0 {
                best_tick = U256::from(
                    self.current_tick.get() - (U128::from(counter) * self.tick_spacing.get()),
                );
            }

            if counter >= 5 || best_tick != U256::from(0) {
                break;
            }
        }

        best_tick
    }

    pub fn best_sell_tick(&self) -> U256 {
        let mut counter = 0;
        let mut best_tick = U256::from(0);

        loop {
            counter += 1;

            let tick_orders_len: usize = self
                .tick_orders
                .get(self.current_tick.get() + (U128::from(counter) * self.tick_spacing.get()))
                .len();

            if tick_orders_len != 0 {
                best_tick = U256::from(
                    self.current_tick.get() - (U128::from(counter) * self.tick_spacing.get()),
                );
            }

            if counter >= 5 || best_tick != U256::from(0) {
                break;
            }
        }

        best_tick
    }

    pub fn market_tick(&self, is_buy: bool) -> U256 {
        if is_buy {
            self.best_buy_tick()
        } else {
            self.best_sell_tick()
        }
    }

    pub fn top_n_best_buy_ticks(&self) -> Vec<U256> {
        let mut counter = 0;
        let mut best_ticks: Vec<U256> = Vec::new();

        loop {
            counter += 1;

            let tick_orders_len: usize = self
                .tick_orders
                .get(self.current_tick.get() - (U128::from(counter) * self.tick_spacing.get()))
                .len();

            if tick_orders_len != 0 {
                best_ticks.push(U256::from(
                    self.current_tick.get() - (U128::from(counter) * self.tick_spacing.get()),
                ));
            }

            if counter >= 5 || best_ticks.len() >= 5 {
                break;
            }
        }

        best_ticks
    }

    pub fn top_n_best_sell_ticks(&self) -> Vec<U256> {
        let mut counter = 0;
        let mut best_ticks: Vec<U256> = Vec::new();

        loop {
            counter += 1;

            let tick_orders_len: usize = self
                .tick_orders
                .get(self.current_tick.get() + (U128::from(counter) * self.tick_spacing.get()))
                .len();

            if tick_orders_len != 0 {
                best_ticks.push(
                    U256::from(self.current_tick.get())
                        - (U256::from(counter) * U256::from(self.tick_spacing.get())),
                );
            }

            if counter >= 5 || best_ticks.len() >= 5 {
                break;
            }
        }

        best_ticks
    }

    pub fn buy_volume(&self) -> U256 {
        self.buy_volume.get()
    }

    pub fn sell_volume(&self) -> U256 {
        self.sell_volume.get()
    }

    pub fn match_market_order(
        &mut self,
        incoming_order_quantity: U256,
        incoming_order_tick: U256,
        incoming_order_is_buy: bool,
    ) {
        let mut remaining_incoming_order_quantity = incoming_order_quantity;
        if incoming_order_is_buy {
            let possible_ticks = self.top_n_best_sell_ticks();

            if possible_ticks.is_empty() {
                for tick in possible_ticks {
                    let mut sell_quantity = U128::from(0);
                    let initial_remaining_incoming_order_quantity =
                        U128::from(remaining_incoming_order_quantity);
                    let mut orders: Vec<(U256, U256, U256)> = Vec::new();

                    {
                        let tick_orders = self.tick_orders.get(U128::from(tick));
                        if !tick_orders.is_empty() {
                            for index in 0..tick_orders.len() {
                                let sell_order = tick_orders.get(index).unwrap();
                                sell_quantity += sell_order.quantity.get();
                                orders.push((
                                    U256::from(index),
                                    tick,
                                    U256::from(sell_order.quantity.get()),
                                ));
                            }
                        }
                    } 

                    if !orders.is_empty() {
                        remaining_incoming_order_quantity = self.execute_match(
                            orders,
                            remaining_incoming_order_quantity,
                            incoming_order_is_buy,
                        );
                    }

                    let mut tick_data = self.ticks.setter(U128::from(tick));
                    if initial_remaining_incoming_order_quantity > sell_quantity {
                        tick_data.is_buy.set(true);
                        tick_data.volume.set(U128::from(0));
                    } else if initial_remaining_incoming_order_quantity < sell_quantity {
                        tick_data
                            .volume
                            .set(sell_quantity - initial_remaining_incoming_order_quantity);
                    } else {
                        tick_data.volume.set(U128::from(0));
                    }

                    if remaining_incoming_order_quantity == U256::ZERO {
                        break;
                    }
                }

                if remaining_incoming_order_quantity != U256::ZERO {
                    self.add_order_to_orderbook(
                        U256::from(remaining_incoming_order_quantity),
                        incoming_order_tick,
                        incoming_order_is_buy,
                    );
                }
            } else {
                self.add_order_to_orderbook(
                    remaining_incoming_order_quantity,
                    incoming_order_tick,
                    incoming_order_is_buy,
                );
            }
        } else {
            let possible_ticks = self.top_n_best_buy_ticks();

            if possible_ticks.is_empty() {
                for tick in possible_ticks {
                    let mut buy_quantity = U128::from(0);
                    let initial_remaining_incoming_order_quantity =
                        U128::from(remaining_incoming_order_quantity);
                    let mut orders: Vec<(U256, U256, U256)> = Vec::new();

                    {
                        let tick_orders = self.tick_orders.get(U128::from(tick));
                        if !tick_orders.is_empty() {
                            for index in 0..tick_orders.len() {
                                let buy_order = tick_orders.get(index).unwrap();
                                buy_quantity += buy_order.quantity.get();
                                orders.push((
                                    U256::from(index),
                                    tick,
                                    U256::from(buy_order.quantity.get()),
                                ));
                            }
                        }
                    } 

                    if !orders.is_empty() {
                        remaining_incoming_order_quantity = self.execute_match(
                            orders,
                            remaining_incoming_order_quantity,
                            incoming_order_is_buy,
                        );
                    }

                    let mut tick_data = self.ticks.setter(U128::from(tick));
                    if initial_remaining_incoming_order_quantity > buy_quantity {
                        tick_data.is_buy.set(false);
                        tick_data.volume.set(U128::from(0));
                    } else if initial_remaining_incoming_order_quantity < buy_quantity {
                        tick_data
                            .volume
                            .set(buy_quantity - initial_remaining_incoming_order_quantity);
                    } else {
                        tick_data.volume.set(U128::from(0));
                    }

                    if remaining_incoming_order_quantity == U256::ZERO {
                        break;
                    }
                }

                if remaining_incoming_order_quantity != U256::ZERO {
                    self.add_order_to_orderbook(
                        U256::from(remaining_incoming_order_quantity),
                        incoming_order_tick,
                        incoming_order_is_buy,
                    );
                }
            } else {
                self.add_order_to_orderbook(
                    remaining_incoming_order_quantity,
                    incoming_order_tick,
                    incoming_order_is_buy,
                );
            }
        }
    }

    pub fn limit_market_order(
        &mut self,
        incoming_order_quantity: U256,
        incoming_order_tick: U256,
        incoming_order_is_buy: bool,
    ) {
        let mut remaining_incoming_order_quantity = incoming_order_quantity;

        if incoming_order_is_buy {
            let possible_ticks = self.top_n_best_sell_ticks();

            if !possible_ticks.is_empty() {
                let orders_to_process: Vec<(U256, Vec<(U256, U256, U256)>)> = possible_ticks
                    .into_iter()
                    .filter(|&tick| incoming_order_tick >= tick)
                    .filter(|&tick| !self.tick_orders.get(U128::from(tick)).is_empty())
                    .map(|tick| {
                        let orders: Vec<(U256, U256, U256)> = (0..self
                            .tick_orders
                            .get(U128::from(tick))
                            .len())
                            .filter_map(|index| {
                                let tick_orders = self.tick_orders.get(U128::from(tick));
                                let order = tick_orders.get(index)?;
                                Some((U256::from(index), tick, U256::from(order.quantity.get())))
                            })
                            .collect();
                        (tick, orders)
                    })
                    .collect();

                for (_, orders) in orders_to_process {
                    remaining_incoming_order_quantity = self.execute_match(
                        orders,
                        remaining_incoming_order_quantity,
                        incoming_order_is_buy,
                    );

                    if remaining_incoming_order_quantity == U256::ZERO {
                        break;
                    }
                }
            }
        } else {
            let possible_ticks = self.top_n_best_sell_ticks();

            if !possible_ticks.is_empty() {
                let orders_to_process: Vec<(U256, Vec<(U256, U256, U256)>)> = possible_ticks
                    .into_iter()
                    .filter(|&tick| incoming_order_tick >= tick)
                    .filter(|&tick| !self.tick_orders.get(U128::from(tick)).is_empty())
                    .map(|tick| {
                        let orders: Vec<(U256, U256, U256)> = (0..self
                            .tick_orders
                            .get(U128::from(tick))
                            .len())
                            .filter_map(|index| {
                                let tick_orders = self.tick_orders.get(U128::from(tick));
                                let order = tick_orders.get(index)?;
                                Some((U256::from(index), tick, U256::from(order.quantity.get())))
                            })
                            .collect();
                        (tick, orders)
                    })
                    .collect();

                for (_, orders) in orders_to_process {
                    remaining_incoming_order_quantity = self.execute_match(
                        orders,
                        remaining_incoming_order_quantity,
                        incoming_order_is_buy,
                    );

                    if remaining_incoming_order_quantity == U256::ZERO {
                        break;
                    }
                }
            }
        }

        if remaining_incoming_order_quantity != U256::ZERO {
            self.add_order_to_orderbook(
                remaining_incoming_order_quantity,
                incoming_order_tick,
                incoming_order_is_buy,
            );
        }
    }

    fn execute_match(
        &mut self,
        valid_orders: Vec<(U256, U256, U256)>,
        incoming_order_quantity: U256,
        incoming_order_is_buy: bool,
    ) -> U256 {
        let mut remaining_incoming_order_quantity = incoming_order_quantity;

        for (order_index, order_tick, order_quantity) in valid_orders {
            let mut remaining_order_quantity = order_quantity;

            // Partially Matched
            if remaining_order_quantity < remaining_incoming_order_quantity {
                remaining_incoming_order_quantity -= order_quantity;
                remaining_order_quantity = U256::ZERO;
            }
            // Perfectly Matched
            else if remaining_order_quantity == remaining_incoming_order_quantity {
                remaining_incoming_order_quantity = U256::ZERO;
                remaining_order_quantity = U256::ZERO;
                break;
            }
            // Fully Matched
            else {
                remaining_incoming_order_quantity = U256::ZERO;
                remaining_order_quantity -= remaining_incoming_order_quantity;
                break;
            }

            self.update_order(
                order_index,
                remaining_order_quantity,
                order_tick,
                incoming_order_is_buy,
            );
        }

        remaining_incoming_order_quantity
    }

    fn update_order(&mut self, index: U256, quantity: U256, tick: U256, is_buy: bool) {
        let mut tick_data = if is_buy {
            self.buy_volume.set(tick * quantity);
            self.tick_orders.setter(U128::from(tick))
        } else {
            self.sell_volume.set(tick * quantity);
            self.tick_orders.setter(U128::from(tick))
        };

        let mut order = tick_data.setter(index).unwrap();

        order.quantity.set(U128::from(quantity));
    }
}
