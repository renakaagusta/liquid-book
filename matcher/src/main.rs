#![cfg_attr(not(feature = "export-abi"), no_main, no_std)]

#[cfg(feature = "export-abi")]
fn main() {
    matcher::print_abi("MIT-OR-APACHE-2.0", "pragma solidity ^0.8.23;");
}
