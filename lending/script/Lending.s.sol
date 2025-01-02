// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Lending} from "../src/Lending.sol";
import {MockUSDC} from "../src/mocks/MockUSDC.sol"; // Import the MockUSDC contract

contract LendingScript is Script {
    Lending public lending;
    MockUSDC public mockUSDC;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        // Deploy the mock USDC contract
        mockUSDC = new MockUSDC();

        // Deploy the Lending contract with the address of the mock USDC contract
        lending = new Lending(mockUSDC);

        vm.stopBroadcast();
    }
}
