#![allow(clippy::module_name_repetitions)]

use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum ParseMacError {
    #[error("Invalid number of octets in the MAC: {0} (Should be 6)")]
    InvalidLength(usize),
    #[error("One or more of the MAC's octets are invalid")]
    InvalidOctet(#[from] ParseIntError),
}
