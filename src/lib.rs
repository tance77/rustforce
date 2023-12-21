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
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {//!
//!
//! use rustforce::client::bulk_api::BulkAPi;
//! use rustforce::client::client::{Client};
//!
//! let mut client = Client::new();
//!
//! client.set_client_id("your_client_id");
//! client.set_client_secret("your_client_secret");
//!
//! client.login_with_credential("your_username".to_string(), "your_password".to_string()).await?;
//!
//!
//!
//! Ok(())
//! }
//! ```
pub mod access_token;
pub mod client;
pub mod errors;
pub mod responses;
pub mod utils;
pub mod xml;
