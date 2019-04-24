#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum QueryType{
    UNKNOW(u16),
    A, //Support IPv4
    NS,
    CNAME,
    MX,
    AAAA,
}

impl QueryType{
    pub fn to_num(&self) -> u16{
        match *self{
            QueryType::UNKNOW(x) => x,
            QueryType::A => 1,
            QueryType::NS => 2,
            QueryType::CNAME => 5,
            QueryType::MX => 15,
            QueryType::AAAA => 28,
        }
    }

    pub fn from_num(num: u16) -> QueryType{
        match num{
            1 => QueryType::A,
            2 => QueryType::NS,
            5 => QueryType::CNAME,
            15 => QueryType::MX,
            28 => QueryType::AAAA,
            _ => QueryType::UNKNOW(num),
        }
    }
}