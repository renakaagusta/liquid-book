// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {Lending} from "../src/Lending.sol";
import {MockUSDC} from "../src/mocks/MockUSDC.sol";

contract LendingTest is Test {
    Lending lending;
    MockUSDC mockUSDC;
    address user = address(0x123);
    address rewarder = address(0x456);

    function setUp() public {
        mockUSDC = new MockUSDC();
        lending = new Lending(mockUSDC);

        mockUSDC.transfer(user, 1000);
    }

    function testDeposit() public {
        vm.startPrank(user);
        mockUSDC.approve(address(lending), 1000);
        lending.deposit(1000, user);

        uint256 balance = lending.getBalanceWithYield(user);
        assertEq(balance, 1000, "Balance should be 1000");
        vm.stopPrank();
    }

    function testWithdraw() public {
        vm.startPrank(user);
        mockUSDC.approve(address(lending), 1000);
        lending.deposit(1000, user);
        vm.stopPrank();

        //Send reward to the lending contract
        deal(address(mockUSDC), rewarder, 1000);
        mockUSDC.transfer(address(lending), 1000);
        vm.warp(block.timestamp + 365 days);

        uint256 balance = lending.getBalanceWithYield(user);
        assertEq(balance, 1050, "Balance should be 1050");

        //Withdraw
        vm.startPrank(user);
        lending.withdraw(1050, user);
        uint256 userBalance = mockUSDC.balanceOf(user);
        assertEq(
            userBalance,
            1050,
            "User should have 1050 USDC after withdrawal"
        );

        vm.stopPrank();
    }
}
