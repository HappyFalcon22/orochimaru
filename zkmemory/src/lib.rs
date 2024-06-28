//! This crate provides a simple RAM machine for use in the zkVM
#![recursion_limit = "256"]
#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    unused,
    warnings,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    missing_docs,
    unused_imports
)]
#![forbid(unsafe_code)]

/// Base trait for generic type
pub mod base;
/// A commitment module that commit to the memory trace through the execution trace
/// Currently supports: KZG, Merkle Tree, Verkle Tree.
pub mod commitment;
/// Define all configuration of `StateMachine`
pub mod config;
/// Constraints for checking the lexicographic ordering
pub mod constraints;
/// Define all errors of `StateMachine`
pub mod error;
/// Definition of abstract machine (instruction, trace and context)
pub mod machine;
