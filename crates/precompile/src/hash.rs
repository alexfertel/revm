//! Hash precompiles, it contains SHA-256 and RIPEMD-160 hash precompiles
//! More details in [`sha256_run`] and [`ripemd160_run`]
use super::calc_linear_cost_u32;
use crate::{PrecompileError, PrecompileOutput, PrecompileResult, PrecompileWithAddress};
use sha2::Digest;

/// SHA-256 precompile
pub const SHA256: PrecompileWithAddress =
    PrecompileWithAddress(crate::u64_to_address(2), sha256_run);

/// RIPEMD-160 precompile
pub const RIPEMD160: PrecompileWithAddress =
    PrecompileWithAddress(crate::u64_to_address(3), ripemd160_run);

/// Computes the SHA-256 hash of the input data
///
/// This function follows specifications defined in the following references:
/// - [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf)
/// - [Solidity Documentation on Mathematical and Cryptographic Functions](https://docs.soliditylang.org/en/develop/units-and-global-variables.html#mathematical-and-cryptographic-functions)
/// - [Address 0x02](https://etherscan.io/address/0000000000000000000000000000000000000002)
pub fn sha256_run(input: &[u8], gas_limit: u64) -> PrecompileResult {
    let cost = calc_linear_cost_u32(input.len(), 60, 12);
    if cost > gas_limit {
        Err(PrecompileError::OutOfGas)
    } else {
        let output = sha2::Sha256::digest(input);
        Ok(PrecompileOutput::new(cost, output.to_vec().into()))
    }
}

/// Computes the RIPEMD-160 hash of the input data
///
/// This function follows specifications defined in the following references:
/// - [Ethereum Yellow Paper](https://ethereum.github.io/yellowpaper/paper.pdf)
/// - [Solidity Documentation on Mathematical and Cryptographic Functions](https://docs.soliditylang.org/en/develop/units-and-global-variables.html#mathematical-and-cryptographic-functions)
/// - [Address 03](https://etherscan.io/address/0000000000000000000000000000000000000003)
pub fn ripemd160_run(input: &[u8], gas_limit: u64) -> PrecompileResult {
    let gas_used = calc_linear_cost_u32(input.len(), 600, 120);
    if gas_used > gas_limit {
        Err(PrecompileError::OutOfGas)
    } else {
        let mut hasher = ripemd::Ripemd160::new();
        hasher.update(input);

        let mut output = [0u8; 32];
        hasher.finalize_into((&mut output[12..]).into());
        Ok(PrecompileOutput::new(gas_used, output.to_vec().into()))
    }
}
