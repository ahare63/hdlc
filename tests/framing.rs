extern crate hdlc;

#[cfg(test)]
mod tests {
    use hdlc::hdlc::{decode, encode, SpecialChars};

    #[test]
    fn packetizes() {
        let msg: Vec<u8> = vec![0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09];
        let chars = SpecialChars::default();

        assert_eq!(encode(msg, chars), [126, 1, 80, 0, 0, 0, 5, 128, 9, 126])
    }

    #[test]
    fn pack_byte_swaps() {
        let msg: Vec<u8> = vec![0x01, 0x7E, 0x00, 0x7D, 0x00, 0x05, 0x80, 0x09];
        let chars = SpecialChars::default();

        assert_eq!(
            encode(msg, chars),
            [126, 1, 125, 94, 0, 125, 93, 0, 5, 128, 9, 126]
        )
    }

    #[test]
    fn pack_custom_s_chars() {
        let chars = SpecialChars::new(0x71, 0x70, 0x51, 0x50);
        let msg: Vec<u8> = vec![0x01, 0x7E, 0x70, 0x7D, 0x00, 0x05, 0x80, 0x09];

        assert_eq!(
            encode(msg, chars),
            [0x71, 1, 126, 112, 80, 125, 0, 5, 128, 9, 0x71]
        )
    }

    #[test]
    fn pack_rejects_dupe_s_chars() {
        let chars = SpecialChars::new(0x7E, 0x7D, 0x5D, 0x5D);
        let msg: Vec<u8> = vec![0x01, chars.fend, 0x00, chars.fesc, 0x00, 0x05, 0x80, 0x09];

        assert_eq!(encode(msg, chars), [])
    }

    #[test]
    fn depacketizes() {
        let chars = SpecialChars::default();
        let msg: Vec<u8> = vec![
            chars.fend, 0x01, 0x50, 0x00, 0x00, 0x00, 0x05, 0x80, 0x09, chars.fend
        ];

        assert_eq!(decode(msg, chars), [1, 80, 0, 0, 0, 5, 128, 9])
    }

    #[test]
    fn depack_it_swaps() {
        let chars = SpecialChars::default();
        let msg: Vec<u8> = vec![
            chars.fend,
            0x01,
            chars.fesc,
            chars.tfesc,
            0x00,
            0x00,
            chars.fesc,
            chars.tfend,
            0x05,
            0x80,
            0x09,
            chars.fend,
        ];

        assert_eq!(decode(msg, chars), [1, 125, 0, 0, 126, 5, 128, 9])
    }

    #[test]
    fn depack_custom_s_chars() {
        let chars = SpecialChars::new(0x71, 0x70, 0x51, 0x50);
        let msg: Vec<u8> = vec![
            chars.fend,
            0x01,
            0x7E,
            chars.fesc,
            chars.tfend,
            0x00,
            0x05,
            0x80,
            chars.fesc,
            chars.tfesc,
            0x09,
            0x71,
        ];

        assert_eq!(decode(msg, chars), [1, 126, 0x71, 0, 5, 128, 0x70, 9])
    }

    #[test]
    fn depack_rejects_dupe_s_chars() {
        let chars = SpecialChars::new(0x7E, 0x7D, 0x5D, 0x5D);
        let msg: Vec<u8> = vec![0x01, chars.fend, 0x00, chars.fesc, 0x00, 0x05, 0x80, 0x09];

        assert_eq!(decode(msg, chars), [])
    }
}