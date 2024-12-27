#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

// use alloc::vec::Vec;
use alloy_primitives::Address;
use alloy_sol_macro::sol;
use stylus_sdk::{
    alloy_primitives::U256, 
    prelude::*, 
    evm,
    console
};

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

sol! {
    event PlaceOrder(address indexed user, uint256 indexed tick, bool indexed is_buy, uint256 volume);
}

sol_storage! {
    #[entrypoint]
    pub struct LiquidBookEngine {
        address bitmap_manager_address;
        address tick_manager_address;
        address order_manager_address;
        address matcher_manager_address;
    }
}

sol_interface! {
    interface ITickManager {
        function getTickData(uint256 tick) external view returns (uint256, uint256, uint256, bool);
        function setTickData(uint256 tick, (uint256, uint256, uint256, bool) tick_data) external;
        function getCurrentTick() external view returns (uint256);
    }

    interface IOrderManager {
        function insertOrder(uint256 tick, uint256 volume, address user, bool is_buy) external;
        function readOrder(uint256 tick, uint256 order_index) external view returns (address, uint256);
        function writeOrder(uint256 tick, uint256 order_index, address user, uint256 volume) external;
    }

    interface IBitmapStorage {
        function topNBestTicks(bool is_buy) external view returns (uint256[] memory);
    }

    interface IMatcherManager {
        function execute((uint256,uint256,uint256)[] valid_orders, uint256 incoming_order_volume, uint256 tick_value, uint256 tick_volume) external returns (uint256);
    }
}

#[public]

impl LiquidBookEngine {
    pub fn initialize(&mut self, tick_manager_address: Address, order_manager_address: Address, bitmap_manager_address: Address, matcher_manager_address: Address) {
        self.tick_manager_address.set(tick_manager_address);
        self.order_manager_address.set(order_manager_address);
        self.bitmap_manager_address.set(bitmap_manager_address);
        self.matcher_manager_address.set(matcher_manager_address);
    }

    pub fn place_order(
        &mut self,
        incoming_order_tick: U256,
        incoming_order_volume: U256,
        incoming_order_user: Address,
        incoming_order_is_buy: bool,
        incoming_order_is_market: bool
    ) {
        let tick_manager = ITickManager::new(self.tick_manager_address.get());
        let order_manager = IOrderManager::new(self.order_manager_address.get());
        let bitmap_manager = IBitmapStorage::new(self.bitmap_manager_address.get());
        let matcher = IMatcherManager::new(self.matcher_manager_address.get());

        let mut remaining_incoming_order_volume: alloy_primitives::Uint<256, 4> =
            incoming_order_volume;
        let possible_ticks: Vec<U256> = bitmap_manager.top_n_best_ticks(&*self, incoming_order_is_buy).unwrap();

        let filtered_possible_ticks: Vec<U256> = if incoming_order_is_market {
            possible_ticks
        } else if incoming_order_is_buy {
            possible_ticks
                .iter()
                .filter(|tick| incoming_order_tick >= **tick)
                .cloned()
                .collect()
        } else {
            possible_ticks
                .iter()
                .filter(|tick| incoming_order_tick <= **tick)
                .cloned()
                .collect()
        };
        
        if !filtered_possible_ticks.is_empty() {
            let mut last_tick = U256::from(0);

            for tick in filtered_possible_ticks {
                let (start_index, _, volume, _) = tick_manager.get_tick_data(&*self, tick).unwrap();

                let mut orders: Vec<(U256, U256, U256)> = Vec::new();

                console!("ENGINE :: filtered possible ticks");
                if volume != U256::ZERO {
                    console!("ENGINE :: valid_orders: 1");
                    let mut index = start_index % U256::from(256);

                    loop {
                        let order = order_manager
                            .read_order(&*self, tick, U256::from(index))
                            .unwrap();
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
                    remaining_incoming_order_volume = matcher.execute(&mut *self, orders, remaining_incoming_order_volume, tick, volume).unwrap();
                }

                if remaining_incoming_order_volume == U256::ZERO {
                    break;
                }

                last_tick = tick;
            }

            if remaining_incoming_order_volume != U256::ZERO {
                let _ = order_manager.insert_order(
                    self,
                    last_tick,
                    U256::from(remaining_incoming_order_volume),
                    incoming_order_user,
                    incoming_order_is_buy,
                );
            }
        } else {     
            console!("ENGINE :: no filtered possible_ticks");
            let _ = order_manager.insert_order(
                self,
                incoming_order_tick,
                U256::from(remaining_incoming_order_volume),
                incoming_order_user,
                incoming_order_is_buy,
            );
        }

        evm::log(PlaceOrder {
            user: incoming_order_user,
            tick: incoming_order_tick,
            is_buy: incoming_order_is_buy,
            volume: incoming_order_volume,
        });
    }
}