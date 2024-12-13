// Allow `cargo stylus export-abi` to generate a main function.
#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use stylus_sdk::{
    alloy_primitives::{Address, U128, U256},
    console,
    prelude::*,
    storage,
};

sol_storage! {
    #[entrypoint]
    pub struct LiquidBookEngine {
        address storage_address;
    }

    pub struct Order {
        address user;
        uint128 volume;
    }

    pub struct Tick {
        uint128 start_index;
        uint128 length;
        uint128 volume;
        bool is_buy;
    }
}

sol_interface! {
    interface ILiquidBookStorage {
        function insertOrder(uint256 tick, uint256 volume, address user, bool is_buy) external;
        function updateOrder(uint256 tick, uint256 volume, uint256 order_index) external;
        function updateTick(uint256 tick, uint256 volume, bool is_buy, bool is_existing_order) external;
        function readOrder(uint256 tick, uint256 order_index) external view returns (address, uint256);
        function writeOrder(uint256 tick, uint256 order_index, address user, uint256 volume) external;
        function deleteOrder(uint256 tick, uint256 order_index) external;
        function encodeOrderKey(uint256 tick, uint256 order_index) external view returns (uint8[] memory);
        function encodeOrderData(address user, uint256 volume) external view returns (uint8[32] memory);
        function decodeOrderData(uint8[] memory encoded) external view returns (address, uint256);
        function getTickData(uint256 tick) external view returns (uint256, uint256, uint256, bool);
        function setTickData(uint256 tick, (uint256, uint256, uint256, bool) tick_data) external;
        function getCurrentTick() external view returns (uint256);
        function setCurrentTick(uint256 tick) external;
    }
}

#[public]
impl LiquidBookEngine {
    pub fn set_storage(&mut self, storage_address: Address) {
        self.storage_address.set(storage_address);
    }

    pub fn top_n_best_ticks(&self, is_buy: bool) -> Result<Vec<U256>, Vec<u8>> {
        let storage = ILiquidBookStorage::new(self.storage_address.get());
        let mut counter = U256::from(0);
        let mut best_ticks: Vec<U256> = Vec::new();
        let current_tick = storage.get_current_tick(self)?;

        loop {
            let tick_data = if is_buy {
                storage.get_tick_data(self, current_tick - counter)
            } else {
                storage.get_tick_data(self, current_tick + counter)
            };

            let (_, _, volume, is_buy) = tick_data.unwrap();

            if is_buy && volume > U256::ZERO {
                best_ticks.push(U256::from(U128::from(current_tick) - U128::from(counter)));
            }

            if counter >= U256::from(5) || best_ticks.len() >= 5 {
                break;
            }

            counter += U256::from(1);
        }

        Ok(best_ticks)
    }

    fn execute_match(
        &mut self,
        valid_orders: Vec<(U256, U256, U256)>,
        incoming_order_quantity: U256,
    ) -> U256 {
        let mut remaining_incoming_order_quantity = incoming_order_quantity;
        let storage_address = self.storage_address.get();
        let storage = ILiquidBookStorage::new(storage_address);

        for (order_index, order_tick, order_quantity) in valid_orders {
            let mut remaining_order_quantity = order_quantity;

            if remaining_order_quantity < remaining_incoming_order_quantity {
                remaining_incoming_order_quantity -= order_quantity;
                remaining_order_quantity = U256::ZERO;
            } else if remaining_order_quantity == remaining_incoming_order_quantity {
                remaining_order_quantity = U256::ZERO;
                remaining_incoming_order_quantity = U256::ZERO;
            } else {
                remaining_order_quantity -= remaining_incoming_order_quantity;
                remaining_incoming_order_quantity = U256::ZERO;
            }

            let _ = storage.set_current_tick(&mut *self, order_tick);
            let _ = storage.update_order(
                &mut *self,
                order_tick,
                order_index,
                remaining_order_quantity,
            );

            if remaining_incoming_order_quantity == U256::ZERO {
                break;
            }
        }

        remaining_incoming_order_quantity
    }

    pub fn match_order(
        &mut self,
        incoming_order_tick: U256,
        incoming_order_volume: U256,
        incoming_order_user: Address,
        incoming_order_is_buy: bool,
        incoming_order_is_market: bool,
    ) {
        let storage_address = self.storage_address.get();
        let storage = ILiquidBookStorage::new(storage_address);
        let mut remaining_incoming_order_volume = incoming_order_volume;
        let possible_ticks = self.top_n_best_ticks(incoming_order_is_buy).unwrap();

        let filtered_possible_ticks: Vec<U256> = if incoming_order_is_market {
            possible_ticks
        } else if incoming_order_is_buy {
            possible_ticks
                .iter()
                .filter(|tick| incoming_order_tick > **tick)
                .cloned()  
                .collect()
        } else {
            possible_ticks
                .iter()
                .filter(|tick| incoming_order_tick < **tick)
                .cloned() 
                .collect()
        };

        if filtered_possible_ticks.is_empty() {
            let mut last_tick = U256::from(0);

            for tick in filtered_possible_ticks {
                let tick_data = storage.get_tick_data(&*self, tick).unwrap();
                let (start_index, _, volume, _) = tick_data;

                let mut orders: Vec<(U256, U256, U256)> = Vec::new();

                if volume != U256::ZERO {
                    let mut index = start_index % U256::from(256);

                    loop {
                        let order = storage.read_order(&*self, tick, U256::from(index)).unwrap();
                        let (_, order_volume) = order;

                        if order_volume != U256::ZERO {
                            orders.push((tick, U256::from(index), order_volume));
                            index = (index + U256::from(1)) % U256::from(256);
                        } else {
                            break;
                        }
                    }
                }

                if !orders.is_empty() {
                    remaining_incoming_order_volume =
                        self.execute_match(orders, remaining_incoming_order_volume);
                }

                if remaining_incoming_order_volume == U256::ZERO {
                    break;
                }

                last_tick = tick;
            }

            if remaining_incoming_order_volume != U256::ZERO {
                // TODO
                // let _ = storage.set_current_tick(self, last_tick);
                let _ = storage.insert_order(
                    self,
                    last_tick,
                    U256::from(remaining_incoming_order_volume),
                    incoming_order_user,
                    incoming_order_is_buy,
                );
            }
        } else {
            let current_tick = storage.get_current_tick(&*self).unwrap();
            let _ = storage.insert_order(
                self,
                current_tick,
                U256::from(remaining_incoming_order_volume),
                incoming_order_user,
                incoming_order_is_buy,
            );
        }
    }
}