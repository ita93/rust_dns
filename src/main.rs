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

fn main() {
    //Perform an A query for google.com
    let qname = "google.com";
    let qtype = QueryType::A;

    //Using googles public DNS server
    let server = ("8.8.8.8", 53);

    let socket = UdpSocket::bind(("0.0.0.0", 43210)).unwrap();

    //Build our query packet.
    let mut packet = DnsPacket::new();
    packet.header.id = 6666;
    packet.header.questions = 1;
    packet.header.recursion_desired = true;
    packet.questions.push(DnsQuestion::new(qname.to_string(), qtype));

    //Use our new write method to write the packet to a buffer
    let mut req_buffer = BytePacketBuffer::new();
    packet.write(&mut req_buffer).unwrap();

    //send it off to the server using our socket
    socket.send_to(&req_buffer.buf[0..req_buffer.pos], server).unwrap();

    //packet for receiving response.
    let mut res_buffer = BytePacketBuffer::new();
    socket.recv_from(&mut res_buffer.buf).unwrap();

    let packet = DnsPacket::from_buffer(&mut res_buffer).unwrap();
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
