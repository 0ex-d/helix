#![allow(clippy::too_many_arguments)]

pub mod builder;
pub mod gossiper;
pub mod integration_tests;
pub mod proposer;
pub mod relay_data;
pub mod router;
pub mod service;
pub mod middleware;

#[cfg(test)]
pub mod test_utils;

mod grpc {
    include!(concat!(env!("OUT_DIR"), "/gossip.rs"));
}
