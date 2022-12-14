#![allow(dead_code)]

mod bindings;
mod egp_env;

pub use bindings::*;
pub use egp_env::*;

// Before `*.world` is supported, the Error type will be generated for each host components, and cannot be resolved to a common one.
// The world proposal is currently tracked by https://github.com/WebAssembly/component-model/pull/83. After `world` is supported,
// we will be able to make this simpler.
wit_error_rs::impl_error!(bindings::icmp::Error);
wit_error_rs::impl_from!(anyhow::Error, icmp::Error::ErrorWithDescription);

wit_error_rs::impl_error!(bindings::socket::Error);
wit_error_rs::impl_from!(anyhow::Error, socket::Error::ErrorWithDescription);
