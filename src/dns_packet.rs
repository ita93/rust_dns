use std::io::Result;
use packet_buffer::BytePacketBuffer;
use query_type::QueryType;
use dns_question::DnsQuestion;
use dns_record::DnsRecord;
use dns_header::DnsHeader;

#[derive(Clone, Debug)]
pub struct DnsPacket{
    pub header: DnsHeader,
    pub questions: Vec<DnsQuestion>,
    pub answers: Vec<DnsRecord>,
    pub authorities: Vec<DnsRecord>,
    pub resources: Vec<DnsRecord>
}

impl DnsPacket{
    pub fn new() -> DnsPacket{
        DnsPacket{
            header: DnsHeader::new(),
            questions: Vec::new(),
            answers: Vec::new(),
            authorities: Vec::new(),
            resources: Vec::new()
        }
    }

    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<DnsPacket> {
        let mut result = DnsPacket::new();
        try!(result.header.read(buffer));

        for _ in 0..result.header.questions {
            let mut question = DnsQuestion::new("".to_string(), QueryType::UNKNOW(0));
            try!(question.read(buffer));
            result.questions.push(question);
        }

        for _ in 0..result.header.answers{
            let rec = try!(DnsRecord::read(buffer));
            result.answers.push(rec);
        }

        for _ in 0..result.header.authoritative_entries {
            let rec = try!(DnsRecord::read(buffer));
            result.authorities.push(rec);
        }

        Ok(result)
    }
}

