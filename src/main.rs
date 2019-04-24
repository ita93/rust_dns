mod dns_header;
mod packet_buffer;
mod result_code;
mod query_type;
mod dns_record;
mod dns_packet;
mod dns_question;

use packet_buffer::BytePacketBuffer;
use dns_packet::DnsPacket;
use query_type::QueryType;
use dns_question::DnsQuestion;
use std::net::UdpSocket;

fn lookup(qname: &str, qtype: QueryType, server: (&str, u16)) -> Result<DnsPacket> {
    let socket = try!(UdpSocket::bind(("0.0.0.0", 43210)));

    let mut packet = DnsPacket::new();

    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet.questions.push(DnsQuestion::new(qname.to_string(), qtype));

    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer).unwrap();
    try!(socket.send_to(&req_buffer.buf[0..req_buffer.pos], server));

    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf).unwrap();

    DnsPacket::from_buffer(&mut res_buffer)
}

fn main() {
    println!("{:?}", packet.header);

    for q in packet.questions{
        println!("{:?}", q);
    }

    for rec in packet.answers{
        println!("{:?}", rec);
    }

    for rec in packet.authorities{
        println!("{:?}", rec);
    }

    for rec in packet.resources {
        println!("{:?}", rec);
    }
}
