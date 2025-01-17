#![cfg_attr(not(feature = "export-abi"), no_main)]
extern crate alloc;

use alloy_sol_types::sol;
use stylus_sdk::{
    alloy_primitives::{Address, U256},
    call::Call,
    console, contract, evm, msg,
    prelude::*,
};

sol_interface! {
    interface IERC20 {
        function totalSupply() external view returns (uint256);
        function balanceOf(address account) external view returns (uint256);
        function transfer(address recipient, uint256 amount)
            external
            returns (bool);
        function allowance(address owner, address spender)
            external
            view
            returns (uint256);
        function approve(address spender, uint256 amount) external returns (bool);
        function transferFrom(address sender, address recipient, uint256 amount)
            external
            returns (bool);
        function mint(uint256 value) external;
        function mintTo(address to, uint256 value) external;
        function name() external pure returns (string memory);
        function symbol() external pure returns (string memory);
        function decimals() external pure returns (uint8);
    }
}

sol! {
    // #[allow(missing_docs)]
    event Deposit(address indexed user, address indexed token, uint256 amount);
    // #[allow(missing_docs)]
    event Withdrawal(address indexed user, address indexed token, uint256 amount);
    // #[allow(missing_docs)]
    event BalanceUpdated(address indexed user, address indexed token, uint256 amount);
    // #[allow(missing_docs)]
    event OperatorSet(address indexed user, address indexed operator, bool approved);

    #[derive(Debug)]
    error AlreadyInitialized();
    #[derive(Debug)]
    error InsufficientBalance(address user, address token, uint256 want, uint256 have);
    #[derive(Debug)]
    error TransferError(address user, address token, uint256 amount);
    #[derive(Debug)]
    error ZeroAmount();
    #[derive(Debug)]
    error UnauthorizedOperator(address user, address operator);
}

#[derive(SolidityError, Debug)]
pub enum BalanceManagerError {
    AlreadyInitialized(AlreadyInitialized),
    InsufficientBalance(InsufficientBalance),
    ZeroAmount(ZeroAmount),
    TransferError(TransferError),
    UnauthorizedOperator(UnauthorizedOperator),
}

sol_storage! {
    #[entrypoint]
    pub struct BalanceManager {
        bool initialized;

        address owner;

        // Mapping to check if an operator is approved by an owner: owner -> operator -> isOperator
        mapping(address => mapping(address => bool)) is_operator;
        // Mapping to store user balances: user -> token_address -> amount
        mapping(address => mapping(address => uint256)) balances;
        // Mapping to store the allowance of a operator for a specific token by an owner: owner -> operator / responsible -> token -> amount
        mapping(address => mapping(address => mapping(address => uint256))) locked_balances;
    }
}

/// Declare that `Counter` is a contract with the following external methods.
#[public]
impl BalanceManager {
    pub fn initialize(&mut self, owner: Address) -> Result<(), Vec<u8>> {
        let initialized = self.initialized.get();
        if initialized {
            return Ok(());
        }
        self.initialized.set(true);
        self.owner.set(owner);
        Ok(())
    }

    pub fn get_balance(&self, user: Address, token: Address) -> U256 {
        self.balances.get(user).get(token)
    }

    pub fn get_locked_balance(&self, user: Address, operator: Address, token: Address) -> U256 {
        self.locked_balances.get(user).get(operator).get(token)
    }

    pub fn deposit(&mut self, token: Address, amount: U256) -> Result<(), BalanceManagerError> {
        if amount == U256::from(0) {
            return Err(BalanceManagerError::ZeroAmount(ZeroAmount {}));
        }
        let sender = msg::sender();
        let this = contract::address();
        let external_contract = IERC20::new(token);
        let config = Call::new_in(self);
        if let Err(_) = external_contract.transfer_from(config, sender, this, amount) {
            // console!("Deposit failed due to transfer error.");
            return Err(BalanceManagerError::TransferError(TransferError {
                user: sender,
                token,
                amount,
            }));
        } 
        // console!("Deposit successful.");

        let mut user_balances = self.balances.setter(sender);
        let current_token_balance = user_balances.get(token);
        user_balances
            .setter(token)
            .set(current_token_balance + amount);
        evm::log(Deposit {
            user: sender,
            token,
            amount,
        });

        Ok(())
    }

    pub fn withdraw(&mut self, token: Address, amount: U256) -> Result<(), BalanceManagerError> {
        let sender = msg::sender();
        let mut user_balances = self.balances.setter(sender);
        let current_token_balance = user_balances.get(token);
        if current_token_balance < amount {
            // console!("Insufficient balance for withdrawal.");
            return Err(BalanceManagerError::InsufficientBalance(
                InsufficientBalance {
                    user: sender,
                    token,
                    want: amount,
                    have: current_token_balance,
                },
            ));
        }
        user_balances
            .setter(token)
            .set(current_token_balance - amount);
        let external_contract = IERC20::new(token);
        let config = Call::new_in(self);

        if let Err(_) = external_contract.transfer(config, sender, amount) {
            // console!("Withdrawal failed due to transfer error.");
            return Err(BalanceManagerError::TransferError(TransferError {
                user: sender,
                token,
                amount,
            }));
        }
        // console!("Withdrawal successful.");
        evm::log(Withdrawal {
            user: sender,
            token,
            amount,
        });
        Ok(())
    }

    pub fn set_operator(
        &mut self,
        operator: Address,
        approved: bool,
    ) -> Result<bool, BalanceManagerError> {
        let sender = msg::sender();
        self.is_operator.setter(sender).insert(operator, approved);

        evm::log(OperatorSet {
            user: sender,
            operator,
            approved,
        });

        Ok(true)
    }

    pub fn lock(
        &mut self,
        user: Address,
        operator: Address,
        token: Address,
        amount: U256,
    ) -> Result<(), BalanceManagerError> {
        if !self.is_operator.get(user).get(operator) {
            return Err(BalanceManagerError::UnauthorizedOperator(
                UnauthorizedOperator { user, operator },
            ));
        }
        let mut user_balances = self.balances.setter(user);
        let current_token_balance = user_balances.get(token);
        if current_token_balance < amount {
            // console!("Insufficient balance for locking.");
            return Err(BalanceManagerError::InsufficientBalance(
                InsufficientBalance {
                    user,
                    token,
                    want: amount,
                    have: current_token_balance,
                },
            ));
        }
        // user_balances
        //     .setter(token)
        //     .set(current_token_balance - amount);
        // let mut user_locked_balances = self.locked_balances.setter(user);
        // let mut operator_balances = user_locked_balances.setter(operator);
        // let current_locked_token_balance = operator_balances.get(token);
        // operator_balances
        //     .setter(token)
        //     .set(current_locked_token_balance + amount);
        // // console!(
        // //     "Locking successful. {}",
        // //     current_locked_token_balance + amount
        // // );
        Ok(())
    }

    pub fn unlock(
        &mut self,
        user: Address,
        operator: Address,
        token: Address,
        amount: U256,
    ) -> Result<(), BalanceManagerError> {
        // let user = msg::sender();
        if !self.is_operator.get(user).get(operator) {
            return Err(BalanceManagerError::UnauthorizedOperator(
                UnauthorizedOperator { user, operator },
            ));
        }
        let mut user_locked_balances = self.locked_balances.setter(user);
        let mut operator_balances = user_locked_balances.setter(operator);
        let current_locked_token_balance = operator_balances.get(token);
        if current_locked_token_balance < amount {
            // console!("Insufficient locked balance for unlocking.");
            return Err(BalanceManagerError::InsufficientBalance(
                InsufficientBalance {
                    user,
                    token,
                    want: amount,
                    have: current_locked_token_balance,
                },
            ));
        }
        operator_balances
            .setter(token)
            .set(current_locked_token_balance - amount);
        let mut user_balances = self.balances.setter(user);
        let current_token_balance = user_balances.get(token);
        user_balances
            .setter(token)
            .set(current_token_balance + amount);
        // console!("Unlocking successful.");
        Ok(())
    }

    pub fn transfer_locked(
        &mut self,
        sender: Address,
        operator: Address,
        receiver: Address,
        token: Address,
        amount: U256,
    ) -> Result<(), BalanceManagerError> {
        // let operator = msg::sender();
        if !self.is_operator.get(sender).get(operator) {
            // console!("Operator is not allowed.");
            return Err(BalanceManagerError::TransferError(TransferError {
                user: sender,
                token,
                amount,
            }));
        }
        let mut sender_locked_balances = self.locked_balances.setter(sender);
        let mut sender_operator_balances = sender_locked_balances.setter(operator);
        let sender_current_locked_token_balance = sender_operator_balances.get(token);
        if sender_current_locked_token_balance < amount {
            // console!("Insufficient locked balance for transfer.");
            return Err(BalanceManagerError::InsufficientBalance(
                InsufficientBalance {
                    user: sender,
                    token,
                    want: amount,
                    have: sender_current_locked_token_balance,
                },
            ));
        }
        // sender_operator_balances
        //     .setter(token)
        //     .set(sender_current_locked_token_balance - amount);
        // console!(
        //     "Sender's locked balance changed from {} to {}.",
        //     sender_current_locked_token_balance,
        //     sender_current_locked_token_balance - amount,
        // );

        // let mut receiver_balances = self.balances.setter(receiver);
        // let receiver_current_token_balance = receiver_balances.get(token);
        // receiver_balances
        //     .setter(token)
        //     .set(receiver_current_token_balance + amount);
        // // console!(
        // //     "Receiver's balance changed from {} to {}.",
        // //     receiver_current_token_balance,
        // //     receiver_current_token_balance + amount
        // // );

        // console!("Transfer locked balance successful.");
        // evm::log(BalanceUpdated {
        //     user: sender,
        //     token,
        //     amount,
        // });
        Ok(())
    }
}
