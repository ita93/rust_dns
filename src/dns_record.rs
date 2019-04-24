use std::net::Ipv4Addr;
use query_type::QueryType;
use std::io::Result;

use packet_buffer::BytePacketBuffer;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[allow(dead_code)]

//use enum because it allow us to add new record types later.
pub enum DnsRecord{
    UNKNOW{
        domain: String,
        qtype: u16,
        data_len: u16,
        ttl: u32
    },
    A{
        domain: String,
        addr: Ipv4Addr,
        ttl: u32,
    },
}

impl DnsRecord{
    pub fn read(buffer: &mut BytePacketBuffer) -> Result<DnsRecord> {
        let mut domain = String::new();
        try!(buffer.read_qname(&mut domain));

        let qtype_num = try!(buffer.read_u16());
        let qtype = QueryType::from_num(qtype_num);
        let _ = try!(buffer.read_u16());
        let ttl = try!(buffer.read_u32());
        let data_len = try!(buffer.read_u16());

        match qtype{
            QueryType::A => {
                let raw_addr = try!(buffer.read_u32());
                let addr = Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8,
                    ((raw_addr >> 8) & 0xFF) as u8,
                    ((raw_addr >> 0) & 0xFF) as u8
                );
                Ok(DnsRecord::A{
                    domain,
                    addr,
                    ttl
                })
            },
            QueryType::UNKNOW(_) => {
                try!(buffer.step(data_len as usize));
                Ok(DnsRecord::UNKNOW{
                    domain,
                    qtype: qtype_num,
                    data_len,
                    ttl
                })
            }
        }
    }
}