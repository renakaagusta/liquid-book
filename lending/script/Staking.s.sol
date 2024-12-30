// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Script, console} from "forge-std/Script.sol";
import {Staking} from "../src/Staking.sol";

contract StakingScript is Script {
    Staking public Staking;

    function setUp() public {}

    function run() public {
        vm.startBroadcast();

        Staking = new Staking();

        vm.stopBroadcast();
    }
}
