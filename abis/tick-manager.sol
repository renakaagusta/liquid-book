/**
 * This file was automatically generated by Stylus and represents a Rust program.
 * For more information, please see [The Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs).
 */

// SPDX-License-Identifier: MIT-OR-APACHE-2.0
pragma solidity ^0.8.23;

contract TickManager {
    event InsertOrder(address indexed user, uint256 indexed tick, bool indexed is_buy, uint256 volume);
    
    event UpdateOrder(uint256 indexed tick, uint256 indexed order_index, uint256 volume);

    function initialize(address engine_address, address bitmap_manager_address, address order_manager_address) external;

    function setTickData(uint256 tick, uint256 volume, bool is_buy, bool is_existing_order) external;

    function getTickData(uint256 tick) external view returns (uint256, uint256, uint256, bool);
}