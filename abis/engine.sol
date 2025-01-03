// SPDX-License-Identifier: MIT-OR-APACHE-2.0
pragma solidity ^0.8.23;

contract LiquidBookEngine {
    event PlaceOrder(address indexed user, int128 indexed tick, uint256 order_index, bool is_buy, bool is_market, uint256 volume, uint256 remaining_volume);

    function initialize(address tick_manager_address, address order_manager_address, address bitmap_manager_address, address matcher_manager_address) external;

    function placeOrder(int128 incoming_order_tick, uint256 incoming_order_volume, address incoming_order_user, bool incoming_order_is_buy, bool incoming_order_is_market) external returns (uint256, int128, uint256);
}