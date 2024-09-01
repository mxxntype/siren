pub mod cli;
pub mod error;

use crate::error::ParseMacError;

/// Translate a MAC address into a [`Vec<u8>`].
///
/// # Errors
///
/// This function will return an error if the provided MAC address does not have
/// precisely 6 octets (separated by `':'`), or any of those octets could not be
/// parsed into a byte, represented by a [`u8`].
pub fn translate_mac<M>(mac: M) -> Result<Vec<u8>, ParseMacError>
where
    M: AsRef<str>,
{
    let mac = mac.as_ref();

    let octet_count = mac.chars().filter(|c| *c == ':').count() + 1;
    match octet_count {
        6 => { /* Fine, go on... */ }
        ..6 | 7.. => return Err(ParseMacError::InvalidLength(octet_count)),
    }

    let bytes = mac
        .split(':')
        .map(|byte| u8::from_str_radix(byte, 16))
        .collect::<Result<Vec<_>, _>>()?;

    // NOTE: We've checked that `mac` contains exactly 5 ':' characters before,
    // which means we *should* always have 6 octets after splitting, but let's
    // still check in debug builds.
    debug_assert_eq!(bytes.len(), 6);

    Ok(bytes)
}

/// Build a `WakeOnLAN` packet with the provided MAC address.
///
/// # Panics
///
/// This function will panic if the provided [`Vec<u8>`] that represents the
/// MAC's bytes does not have a length of 6.
#[must_use]
pub fn build_wakeonlan_packet(mac: &[u8]) -> Vec<u8> {
    assert_eq!(mac.len(), 6);

    let mut packet = vec![0xFF_u8; 6];
    for _ in 0..16 {
        packet.extend_from_slice(mac);
    }

    packet
}

#[cfg(test)]
mod tests {
    use crate::error::ParseMacError;
    use std::num::IntErrorKind;

    #[test]
    fn translate_valid_mac() {
        let mac = "18:31:bf:6e:ca:0c";
        let bytes = super::translate_mac(mac).unwrap();

        assert_eq!(bytes, vec![24, 49, 191, 110, 202, 12]);
    }

    #[test]
    fn translate_mac_with_invalid_octet() {
        let mac = "188:31:bf:6e:ca:0c";
        let error =
            super::translate_mac(mac).expect_err("An invalid MAC was treated like a valid one");

        match error {
            ParseMacError::InvalidLength(wrong_error_kind) => {
                panic!("Wrong ParseMacError kind: {wrong_error_kind:?}")
            }
            ParseMacError::InvalidOctet(error) => match error.kind() {
                IntErrorKind::PosOverflow => {}
                wrong_error_kind => panic!("Wrong IntErrorKind: {wrong_error_kind:?}"),
            },
        }
    }

    #[test]
    fn translate_mac_too_short() {
        let mac = "00:00:00";
        let error = super::translate_mac(mac).unwrap_err();

        match error {
            ParseMacError::InvalidLength(3) => {}
            wrong_error_kind => panic!("Wrong ParseMacError kind: {wrong_error_kind:?}"),
        }
    }

    #[test]
    fn translate_mac_too_long() {
        let mac = "00:00:00:00:00:00:00:00:00";
        let error = super::translate_mac(mac).unwrap_err();

        match error {
            ParseMacError::InvalidLength(9) => {}
            wrong_error_kind => panic!("Wrong ParseMacError kind: {wrong_error_kind:?}"),
        }
    }
}
