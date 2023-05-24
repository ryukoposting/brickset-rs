//! This crate provides utilities for working with the [BrickSet API](https://brickset.com/article/52666/brickset-web-services).
//! This includes:
//! 
//! - Low-level tools for building API requests, and parsing API responses.
//! - High-level client wrapper for [reqwest](https://docs.rs/reqwest/)
//! 
//! # Sample
//! 
//! ```no_run
//! use brickset::{reqwest_api::ClientWrapper, request::GetSetsParameters};
//! use std::error::Error;
//! 
//! const API_KEY: &str = "<your API key>";
//! 
//! #[tokio::main]
//! async fn main() {
//!     let reqwest_client = reqwest::Client::default();
//!     let mut client = ClientWrapper::new(API_KEY, &reqwest_client);
//!     
//!     let params = GetSetsParameters::new()
//!         .query("fire truck")
//!         .theme("City");
//!     
//!     let sets = client.get_sets(params).await.expect("get_sets");
//!     
//!     println!("Found {} matching sets", sets.matches);
//!     for set in sets.sets.iter() {
//!         println!("{} {}", set.number, set.name.as_deref().unwrap_or("(Unknown)"));
//!     }
//! }
//! ```
//! 
//! # Features
//! 
//! - `log` (default): Generate log messages using the [log](https://docs.rs/log/) crate.
//! - `reqwest` (default): High-level wrapper for [reqwest](https://docs.rs/reqwest/). If
//!   you aren't using reqwest, you should disable this feature.

pub mod v3;

pub use v3::*;
