// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

contract Lending {
    // Mock USD token
    IERC20 public mockUSD;

    // Mapping to store user balances
    mapping(address => uint256) public balances;
    // Mapping to store user deposit timestamps
    mapping(address => uint256) public depositTimestamps;
    // Annual interest rate in percentage (e.g., 5 for 5%)
    uint256 public annualInterestRate = 5;

    // Event for deposit
    event Deposit(address indexed user, uint256 amount);
    // Event for withdrawal
    event Withdraw(address indexed user, uint256 amount);
    // Event for yield withdrawal
    event YieldWithdraw(address indexed user, uint256 amount);

    // Constructor to set the Mock USD token address
    constructor(IERC20 _mockUSD) {
        mockUSD = _mockUSD;
    }

    // Function to deposit Mock USD into the contract
    function deposit(uint256 amount, address sender) external {
        require(amount > 0, "Deposit amount must be greater than zero");

        // Transfer Mock USD tokens from the user to the contract
        require(
            mockUSD.transferFrom(sender, address(this), amount),
            "Transfer failed"
        );

        // If the user has a previous balance, we calculate the yield
        if (balances[sender] > 0) {
            uint256 yield = calculateYield(sender);
            balances[sender] += yield;
        }

        // Update the user's balance and deposit timestamp
        balances[sender] += amount;
        depositTimestamps[sender] = block.timestamp;

        emit Deposit(sender, amount);
    }

    // Function to withdraw Mock USD from the contract, including yield
    function withdraw(uint256 amount, address sender) external {
        require(amount > 0, "Withdraw amount must be greater than zero");

        // Calculate the yield and update the balance
        uint256 yield = calculateYield(sender);

        uint256 totalBalance = balances[sender] + yield;

        require(totalBalance >= amount, "Insufficient balance");

        // Update the user's balance and reset the deposit timestamp
        balances[sender] = totalBalance - amount;
        depositTimestamps[sender] = block.timestamp;

        // Transfer the requested amount to the user
        require(mockUSD.transfer(sender, amount), "Transfer failed");

        emit Withdraw(sender, amount);
    }

    // Function to calculate the yield for a user
    function calculateYield(address user) internal view returns (uint256) {
        uint256 timeElapsed = block.timestamp - depositTimestamps[user];
        uint256 principal = balances[user];
        uint256 yield = (principal * annualInterestRate * timeElapsed) /
            (365 days * 100);
        return yield;
    }

    // Function to get the user's balance along with the yield
    function getBalanceWithYield(address user) external view returns (uint256) {
        uint256 yield = calculateYield(user);
        return balances[user] + yield;
    }
}
