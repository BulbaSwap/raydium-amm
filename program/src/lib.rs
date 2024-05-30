// #![deny(missing_docs)]

//! An Uniswap-like program for the Solana blockchain.
#[macro_use]
pub mod log;

mod entrypoint;
pub mod error;
pub mod instruction;
pub mod invokers;
pub mod math;
pub mod processor;
pub mod state;

// Export current solana-sdk types for downstream users who may also be building with a different solana-sdk version
pub use solana_program;

#[cfg(feature = "devnet")]
solana_program::declare_id!("baskUiPSS8Tv3fvCdYKmBfwgwK3Vbkr8KDCCET3KjSv");

#[cfg(not(any(feature = "localnet", feature = "devnet")))]
solana_program::declare_id!("dexrBFKXSS5Mge5FN8DRRMfsqigjgj2P9ifm38orQRx");
