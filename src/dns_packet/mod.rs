use self::{dns_header::DnsHeader, dns_question::DnsQuestion};

mod dns_header;
mod dns_question;
mod dns_record;
mod packet_buffer;

#[derive(Debug)]
pub struct DnsPacket {
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    // pub answers: Vec<DnsRecord>,
    // pub authorities: Vec<DnsRecord>,
    // pub resources: Vec<DnsRecord>,
}

impl DnsPacket {
    pub fn from_bytes(bytes: &[u8]) -> Result<DnsPacket, String> {
        todo!()
    }
}
