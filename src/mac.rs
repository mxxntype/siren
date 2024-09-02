use crate::error::ParseMacError;
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Mac {
    bytes: [u8; 6],
}

impl Mac {
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 6] {
        &self.bytes
    }
}

impl FromStr for Mac {
    type Err = ParseMacError;

    fn from_str(mac: &str) -> Result<Self, Self::Err> {
        let octet_count = mac.chars().filter(|c| *c == ':').count() + 1;
        match octet_count {
            6 => { /* Fine, go on... */ }
            _ => return Err(ParseMacError::InvalidLength(octet_count)),
        };

        let mut bytes = [0u8; 6];
        for (index, byte) in mac.split(':').enumerate() {
            let byte = u8::from_str_radix(byte, 16)?;
            bytes[index] = byte;
        }

        Ok(Self { bytes })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Mac, ParseMacError};
    use pretty_assertions::assert_eq;
    use std::num::IntErrorKind;

    #[test]
    fn translate_valid_mac() {
        let mac = "18:31:bf:6e:ca:0c".parse::<Mac>().unwrap();
        assert_eq!(mac.bytes, [24, 49, 191, 110, 202, 12]);
    }

    #[test]
    fn translate_invalid_mac_too_long() {
        let error = "11:22:33:44:55:66:77:88".parse::<Mac>().unwrap_err();
        assert_eq!(error, ParseMacError::InvalidLength(8));
    }

    #[test]
    fn translate_invalid_mac_too_short() {
        let error = "11:22:33:44".parse::<Mac>().unwrap_err();
        assert_eq!(error, ParseMacError::InvalidLength(4));
    }

    #[test]
    fn translate_invalid_mac_syntax() {
        let error = "11::22:::44".parse::<Mac>().unwrap_err();
        match error {
            ParseMacError::InvalidLength(_) => panic!("Wrong error: {error:?}"),
            ParseMacError::InvalidOctet(error) => match error.kind() {
                IntErrorKind::Empty => { /* Correct error kind */ }
                _ => panic!("Wrong error: {:?}", error.kind()),
            },
        }
    }

    #[test]
    fn translate_mac_with_invalid_octet() {
        let error = "11:22:33:44:55:666".parse::<Mac>().unwrap_err();
        match error {
            ParseMacError::InvalidLength(_) => panic!("Wrong error: {error:?}"),
            ParseMacError::InvalidOctet(error) => match error.kind() {
                IntErrorKind::PosOverflow => { /* Correct error kind */ }
                _ => panic!("Wrong error kind: {:?}", error.kind()),
            },
        }
    }

    #[test]
    fn translate_mac_with_invalid_negative_octet() {
        let error = "11:22:33:44:55:-6".parse::<Mac>().unwrap_err();
        match error {
            ParseMacError::InvalidLength(_) => panic!("Wrong error: {error:?}"),
            ParseMacError::InvalidOctet(error) => match error.kind() {
                IntErrorKind::InvalidDigit => { /* Correct error kind */ }
                _ => panic!("Wrong error kind: {:?}", error.kind()),
            },
        }
    }
}
