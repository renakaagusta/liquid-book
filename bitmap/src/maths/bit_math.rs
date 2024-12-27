use std::ops::ShrAssign;

use ruint_macro::uint;

pub trait U256Extension: Sized {
    /// Returns if the integer is zero.
    fn is_zero(&self) -> bool;
    /// Returns 0 as a [U256].
    fn zero() -> Self;
    /// Returns 1 as a [U256].
    fn one() -> Self;

    /// Converts an 0x prefixed hex string to a [U256]. Only allowed in tests.
    #[cfg(test)]
    fn from_hex_str(value: &str) -> Self;
    /// Converts a decimal string to a [U256]. Only allowed in tests.
    #[cfg(test)]
    fn from_dec_str(value: &str) -> Option<Self>;
}

impl U256Extension for U256 {
    fn is_zero(&self) -> bool {
        self == &Self::zero()
    }

    fn zero() -> Self {
        Self::ZERO
    }

    fn one() -> Self {
        // little endian
        Self::from_limbs([1, 0, 0, 0])
    }

    #[cfg(test)]
    fn from_hex_str(value: &str) -> Self {
        debug_assert!(value.starts_with("0x"));
        value.parse().unwrap()
    }

    #[cfg(test)]
    fn from_dec_str(value: &str) -> Option<Self> {
        debug_assert!(!value.starts_with("0x"));
        value.parse().ok()
    }
}

pub type U256 = stylus_sdk::alloy_primitives::U256;

/// Returns the index of the most significant bit of the number passed.
pub fn most_significant_bit(mut x: U256) -> Result<u8, &'static str> {
    let mut r = 0;

    if x == U256::ZERO {
        return Err("ZeroValue");
    }

    if x >= uint!(0x100000000000000000000000000000000_U256) {
        x.shr_assign(128);
        r += 128;
    }

    if x >= uint!(0x10000000000000000_U256) {
        x.shr_assign(64);
        r += 64;
    }

    if x >= uint!(0x100000000_U256) {
        x.shr_assign(32);
        r += 32;
    }

    if x >= uint!(0x10000_U256) {
        x.shr_assign(16);
        r += 16;
    }

    if x >= uint!(0x100_U256) {
        x.shr_assign(8);
        r += 8;
    }

    if x >= uint!(0x10_U256) {
        x.shr_assign(4);
        r += 4;
    }
    if x >= uint!(0x4_U256) {
        x.shr_assign(2);
        r += 2;
    }

    if x >= uint!(0x2_U256) {
        r += 1;
    }

    Ok(r)
}

/// Returns the index of the least significant bit of the number passed.
pub fn least_significant_bit(mut x: U256) -> Result<u8, &'static str> {
    if x.is_zero() {
        return Err("ZeroValue");
    }

    let mut r = 255;

    if x & U256::from(u128::MAX) > U256::zero() {
        r -= 128;
    } else {
        x >>= 128;
    }

    if x & U256::from(u64::MAX) > U256::zero() {
        r -= 64;
    } else {
        x >>= 64;
    }

    if x & U256::from(u32::MAX) > U256::zero() {
        r -= 32;
    } else {
        x >>= 32;
    }

    if x & U256::from(u16::MAX) > U256::zero() {
        r -= 16;
    } else {
        x >>= 16;
    }

    if x & U256::from(u8::MAX) > U256::zero() {
        r -= 8;
    } else {
        x >>= 8;
    }

    if x & uint!(0xf_U256) > U256::zero() {
        r -= 4;
    } else {
        x >>= 4;
    }

    if x & uint!(0x3_U256) > U256::zero() {
        r -= 2;
    } else {
        x >>= 2;
    }

    if x & uint!(0x1_U256) > U256::zero() {
        r -= 1;
    }

    Ok(r)
}
