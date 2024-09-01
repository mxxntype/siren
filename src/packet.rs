use crate::Mac;

/// A Wake-on-LAN magic packet.
///
/// The Wake-on-LAN magic packet is a frame that is most often sent as a broadcast and that contains
/// (anywhere within its payload) 6 bytes of all 255 (FF FF FF FF FF FF in hexadecimal), followed by
/// sixteen repetitions of the target computer's 48-bit MAC address, for a total of 102 bytes.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Packet {
    // The Wake-on-LAN payload: 6 bytes of 0xFF and the target's MAC address, repeated 16 times.
    bytes: [u8; Self::LENGTH],
}

impl Packet {
    /// The length (in bytes) of the Wake-on-LAN payload.
    pub const LENGTH: usize = 6 + 6 * 16;

    /// Build a new Wake-on-LAN magic packet for the specified MAC address.
    #[must_use]
    pub fn new_with_mac(mac: &Mac) -> Self {
        let mut bytes = [0u8; Self::LENGTH];
        let mac = mac.as_bytes();

        for (offset, byte) in bytes.iter_mut().enumerate() {
            match offset {
                ..6 => *byte = 0xFFu8,
                6.. => *byte = mac[(offset - 6) % 6],
            }
        }

        Self { bytes }
    }

    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; Self::LENGTH] {
        &self.bytes
    }
}

#[cfg(test)]
mod tests {
    use crate::{Mac, Packet};
    use pretty_assertions::assert_eq;

    #[test]
    fn build_from_valid_mac() {
        // 24, 49, 191, 110, 202, 12.
        let mac = "18:31:bf:6e:ca:0c"
            .parse::<Mac>()
            .expect("A valid MAC address couldn't be parsed");

        let packet = Packet::new_with_mac(&mac);
        let packet_bytes = *packet.as_bytes();

        #[rustfmt::skip]
        let expected_bytes: [u8; Packet::LENGTH] = [
            0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
            0x18, 0x31, 0xBF, 0x6E, 0xCA, 0xC,
        ];

        assert_eq!(packet_bytes, expected_bytes);
    }
}
