use dns_packet::DnsPacket;

mod dns_packet;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = std::fs::read("response_packet.txt")?;
    let packet = DnsPacket::from_bytes(&bytes);
    Ok(())
}
