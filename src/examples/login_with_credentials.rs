//! Crate for interacting with the Salesforce API
//!
//! This crate includes the tools connecting to Salesforce and manipulating
//! Salesforce objects
//!
//! # Example
//!
//! The following example will connect to Salesforce and create an Account
//! object
//!
//!
//! ```rust,no_run
//!
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), dyn Error> {//!
//!
//! use rustforce::client::client::Client;
//!
//! let client = Client();
//! Ok(())
//! }
//! ```
