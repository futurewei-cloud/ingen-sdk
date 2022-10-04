#![allow(dead_code)]
#![allow(unused_imports)]

mod bindings_icmp;
mod bindings_socket;

pub use bindings_icmp::*;
pub use bindings_socket::*;

// Before `*.world` is supported, the Error type will be generated for each host components, and cannot be resolved to a common one.
// The world proposal is currently tracked by https://github.com/WebAssembly/component-model/pull/83. After `world` is supported,
// we will be able to make this simpler.
wit_error_rs::impl_error!(icmp::Error);
wit_error_rs::impl_from!(anyhow::Error, icmp::Error::ErrorWithDescription);

wit_error_rs::impl_error!(socket::Error);
wit_error_rs::impl_from!(anyhow::Error, socket::Error::ErrorWithDescription);
