use super::packet_buffer::PacketBuffer;

#[derive(Debug, PartialEq, Eq)]
pub struct DnsHeader {
    pub id: u16,

    pub response: bool,
    pub opcode: u8,
    pub authoritative_answer: bool,
    pub truncated_message: bool,
    pub recursion_desired: bool,

    pub recursion_available: bool,
    pub z: bool,
    pub authed_data: bool,
    pub checking_disabled: bool,
    pub rescode: ResultCode,

    pub questions: u16,
    pub answers: u16,
    pub authoritative_entries: u16,
    pub resource_entries: u16,
}

impl DnsHeader {
    pub fn from_buffer(buffer: &mut PacketBuffer) -> Result<Self, String> {
        let id = buffer.read_u16()?;

        let next_byte = buffer.read_u8()?;
        let response = is_bit_set(next_byte, 1);
        let opcode = (next_byte & 0b0111_1000) << 3;
        let authoritative_answer = is_bit_set(next_byte, 6);
        let truncated_message = is_bit_set(next_byte, 7);
        let recursion_desired = is_bit_set(next_byte, 8);

        let next_byte = buffer.read_u8()?;
        let recursion_available = is_bit_set(next_byte, 1);
        let z = is_bit_set(next_byte, 2);
        let authed_data = is_bit_set(next_byte, 3);
        let checking_disabled = is_bit_set(next_byte, 4);
        let rescode = (next_byte & 0b0000_1111).into();

        let questions = buffer.read_u16()?;
        let answers = buffer.read_u16()?;
        let authoritative_entries = buffer.read_u16()?;
        let resource_entries = buffer.read_u16()?;

        Ok(Self {
            id,
            response,
            opcode,
            authoritative_answer,
            truncated_message,
            recursion_desired,
            recursion_available,
            z,
            authed_data,
            checking_disabled,
            rescode,
            questions,
            answers,
            authoritative_entries,
            resource_entries,
        })
    }
}

/// if `byte` is 0b1000_0000 and we want to test the 1, `nth_bit` should be 1.
fn is_bit_set(byte: u8, nth_bit: u8) -> bool {
    debug_assert!(nth_bit >= 1 && nth_bit <= 8);
    byte & 1 << (8 - nth_bit) > 0
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResultCode {
    NOERROR,
    FORMERR,
    SERVFAIL,
    NXDOMAIN,
    NOTIMP,
    REFUSED,
}

impl From<u8> for ResultCode {
    fn from(byte: u8) -> Self {
        match byte {
            1 => ResultCode::FORMERR,
            2 => ResultCode::SERVFAIL,
            3 => ResultCode::NXDOMAIN,
            4 => ResultCode::NOTIMP,
            5 => ResultCode::REFUSED,
            0 | _ => ResultCode::NOERROR,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::dns_packet::{dns_header::is_bit_set, packet_buffer::PacketBuffer};

    use super::{DnsHeader, ResultCode};

    #[test]
    fn is_bit_set_test() {
        let num = 0b1010_1010;
        assert_eq!(is_bit_set(num, 1), true);
        assert_eq!(is_bit_set(num, 2), false);
        assert_eq!(is_bit_set(num, 8), false);
    }

    #[test]
    fn chapter_example() {
        let bytes = "86 2a 01 20 00 01 00 00 00 00 00 00"
            .split(" ")
            .map(|s| u8::from_str_radix(s, 16).unwrap())
            .collect::<Vec<_>>();
        let mut buffer = PacketBuffer::from_bytes(&bytes);
        let actual = DnsHeader::from_buffer(&mut buffer).unwrap();
        let expected = DnsHeader {
            id: 0x862a,
            response: false,
            opcode: 0,
            authoritative_answer: false,
            truncated_message: false,
            recursion_desired: true,
            recursion_available: false,
            z: false,
            authed_data: true,
            checking_disabled: false,
            rescode: ResultCode::NOERROR,
            questions: 1,
            answers: 0,
            authoritative_entries: 0,
            resource_entries: 0,
        };
        assert_eq!(actual, expected);
    }
}
