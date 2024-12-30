// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";
import {StakingRewards, IERC20} from "../src/Staking.sol";

contract StakingRewardsTest is Test {
    StakingRewards public stakingRewards;
    IERC20 public stakingToken;
    IERC20 public rewardsToken;
    address public owner;
    address public user;

    function setUp() public {
        owner = address(this);
        user = address(0x123);
        stakingToken = IERC20(address(new MockERC20()));
        rewardsToken = IERC20(address(new MockERC20()));
        stakingRewards = new StakingRewards(
            address(stakingToken),
            address(rewardsToken)
        );
    }

    function test_Stake() public {
        uint256 amount = 1000;
        stakingToken.approve(address(stakingRewards), amount);
        stakingRewards.stake(amount);
        assertEq(stakingRewards.balanceOf(address(this)), amount);
        assertEq(stakingRewards.totalSupply(), amount);
    }

    function test_Withdraw() public {}
}

contract MockERC20 is IERC20 {
    mapping(address => uint256) public balances;
    mapping(address => mapping(address => uint256)) public allowances;
    uint256 public override totalSupply;

    function balanceOf(
        address account
    ) external view override returns (uint256) {
        return balances[account];
    }

    function transfer(
        address recipient,
        uint256 amount
    ) external override returns (bool) {
        balances[msg.sender] -= amount;
        balances[recipient] += amount;
        return true;
    }

    function allowance(
        address owner,
        address spender
    ) external view override returns (uint256) {
        return allowances[owner][spender];
    }

    function approve(
        address spender,
        uint256 amount
    ) external override returns (bool) {
        allowances[msg.sender][spender] = amount;
        return true;
    }

    function transferFrom(
        address sender,
        address recipient,
        uint256 amount
    ) external override returns (bool) {
        allowances[sender][msg.sender] -= amount;
        balances[sender] -= amount;
        balances[recipient] += amount;
        return true;
    }

    function mint(address account, uint256 amount) external {
        balances[account] += amount;
        totalSupply += amount;
    }
}
