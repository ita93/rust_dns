use query_type::QueryType;
use packet_buffer::BytePacketBuffer;
use std::io::Result;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DnsQuestion{
    pub name: String,
    pub qtype: QueryType,
}

impl DnsQuestion{
    pub fn new(name: String, qtype: QueryType) -> DnsQuestion{
        DnsQuestion{
            name: name,
            qtype: qtype,
        }
    }

    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<()>{
        try!(buffer.read_qname(&mut self.name));
        self.qtype = QueryType::from_num(try!(buffer.read_u16()));
        let _ = try!(buffer.read_u16());

        Ok(())
    }
}