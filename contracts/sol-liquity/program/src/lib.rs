#![deny(missing_docs)]

//! A lending program for the Solana blockchain.

pub mod entrypoint;

// Export current sdk types for downstream users building with a different sdk version
pub use solana_program;

solana_program::declare_id!("6TvznH3B2e3p2mbhufNBpgSrLx6UkgvxtVQvopEZ2kuH");
