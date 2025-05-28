#![deny(missing_docs)]

//! Codetypo-LSP: Language Server Protocol implementation for Codetypo spell checker.
//!
//! This crate provides the backend for the Codetypo LSP server, enabling spell checking in editors via LSP.

mod codetypo;
/// LSP server implementation and API for Codetypo.
pub mod lsp;
mod state;
mod windows;
