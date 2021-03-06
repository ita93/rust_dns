mod dns_header;
mod packet_buffer;
mod result_code;
mod query_type;
mod dns_record;
mod dns_packet;
mod dns_question;

use result_code::ResultCode;
use packet_buffer::BytePacketBuffer;
use dns_packet::DnsPacket;
use query_type::QueryType;
use dns_question::DnsQuestion;

use std::net::UdpSocket;
use std::io::Result;

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
    //Forward queries to Google's public DNS
    let server = ("8.8.8.8", 53);

    let socket = UdpSocket::bind(("0.0.0.0", 2053)).unwrap(); //Bind UDP.

    loop{
        //Read packet from socket buffer.
        let mut req_buffer = BytePacketBuffer::new();
        let (_, src) = match socket.recv_from(&mut req_buffer.buf){
            Ok(x) => x,
            Err(e) => {
                println!("Failed to read from UDP socket: {:?}", e);
                continue;
            }
        };

        let request = match DnsPacket::from_buffer(&mut req_buffer){
            Ok(x) => x,
            Err(e) => {
                println!("Failed to parse UDP query packet: {:?}", e);
                continue;
            }
        };

        let mut packet = DnsPacket::new();
        packet.header.id = request.header.id;
        packet.header.recursion_desired = true;
        packet.header.recursion_available = true;
        packet.header.response = true;

        if request.questions.is_empty(){
            packet.header.rescode = ResultCode::FORMERR;
        }
        else {
            let question = &request.questions[0];
            println!("Received query: {:?}", question);

            if let Ok(result) = lookup(&question.name, question.qtype, server){
                println!("{:?}", result);
                packet.questions.push(question.clone());
                packet.header.rescode = result.header.rescode;

                for rec in result.answers {
                    println!("Answer: {:?}", rec);
                    packet.answers.push(rec);
                }

                for rec in result.authorities {
                    println!("Authority: {:?}", rec);
                    packet.authorities.push(rec);
                }

                for rec in result.resources {
                    println!("Recource: {:?}", rec);
                    packet.resources.push(rec);
                }
            } else {
                println!("Server failed");
                packet.header.rescode = ResultCode::SERVFAIL;
            }

            let mut res_buffer = BytePacketBuffer::new();
            match packet.write(&mut res_buffer){
                Ok(_) => {},
                Err(e) => {
                    println!("Failed to encode UDP response packet: {:?}", e);
                    continue;
                }
            };

            let len = res_buffer.pos();
            let data = match res_buffer.get_range(0, len){
                Ok(x) => x,
                Err(e) => {
                    println!("Failed to retrieve response buffer: {:?}", e);
                    continue;
                }
            };

            match socket.send_to(data, src){
                Ok(_) => {},
                Err(e) => {
                    println!("Failed to send response buffer: {:?}", e);
                    continue;
                }
            }
        }
    }
}
