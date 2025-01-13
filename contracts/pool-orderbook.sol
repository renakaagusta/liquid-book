/**
 * This file was automatically generated by Stylus and represents a Rust program.
 * For more information, please see [The Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs).
 */

// SPDX-License-Identifier: MIT-OR-APACHE-2.0
pragma solidity ^0.8.23;

interface IPoolLiquidBook {
    function initialize(address asset_token, address stable_token, address pool_manager, address engine_manager, address bitmap_manager, address balance_manager, int128 current_tick, uint256 lot_size) external;

    function getTokenAddress(bool is_buy) external returns (address);

    function getEngineAddress() external view returns (address);

    function getLotSize() external view returns (uint256);

    function calculatePrice(uint256 volume, int128 tick) external returns (uint256);

    function transferLocked(int128 transfer_tick, uint256 transfer_volume, address buyer, address seller) external;
}
