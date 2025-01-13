// interface IErc20 {
//     function name() external pure returns (string memory);

//     function symbol() external pure returns (string memory);

//     function decimals() external pure returns (uint8);

//     function totalSupply() external view returns (uint256);

//     function balanceOf(address owner) external view returns (uint256);

//     function transfer(address to, uint256 value) external returns (bool);

//     function transferFrom(address from, address to, uint256 value) external returns (bool);

//     function approve(address spender, uint256 value) external returns (bool);

//     function allowance(address owner, address spender) external view returns (uint256);

//     error InsufficientBalance(address, uint256, uint256);

//     error InsufficientAllowance(address, address, uint256, uint256);
// }

// interface IWethToken is IErc20 {
//     function mint(uint256 value) external;

//     function mintTo(address to, uint256 value) external;

//     function burn(uint256 value) external;

//     error InsufficientBalance(address, uint256, uint256);

//     error InsufficientAllowance(address, address, uint256, uint256);
// }

// Only run this as a WASM if the export-abi feature is not set.
#![cfg_attr(not(any(feature = "export-abi", test)), no_main)]
extern crate alloc;

// Modules and imports
mod erc20;

use crate::erc20::{Erc20, Erc20Error, Erc20Params};
use alloy_primitives::{Address, U256};
use stylus_sdk::{msg, prelude::*};

/// Immutable definitions
struct WethTokenParams;
impl Erc20Params for WethTokenParams {
    const NAME: &'static str = "Wrapped ETH";
    const SYMBOL: &'static str = "WETH";
    const DECIMALS: u8 = 18;
}

sol_storage! {
    #[entrypoint]
    struct WethToken {
        // Allows erc20 to access StylusToken's storage and make calls
        #[borrow]
        Erc20<WethTokenParams> erc20;
    }
}

#[public]
#[inherit(Erc20<WethTokenParams>)]
impl WethToken {
    /// Mints tokens
    pub fn mint(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.mint(msg::sender(), value)?;
        Ok(())
    }

    /// Mints tokens to another address
    pub fn mint_to(&mut self, to: Address, value: U256) -> Result<(), Erc20Error> {
        self.erc20.mint(to, value)?;
        Ok(())
    }

    /// Burns tokens
    pub fn burn(&mut self, value: U256) -> Result<(), Erc20Error> {
        self.erc20.burn(msg::sender(), value)?;
        Ok(())
    }
}
