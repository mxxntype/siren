use std::num::ParseIntError;

#[allow(clippy::module_name_repetitions)]
#[derive(thiserror::Error, Debug)]
pub enum ParseMacError {
    #[error("Invalid MAC address length: {0} (Should be 6)")]
    InvalidLength(usize),

    #[error("Invalid MAC address octet: {0}")]
    InvalidOctet(#[from] ParseIntError),
}
