#[derive(PartialEq, Eq, Debug, Clone, Hash, Copy)]
pub enum QueryType{
    UNKNOW(u16),
    A, //Support IPv4
}

impl QueryType{
    pub fn to_num(&self) -> u16{
        match *self{
            QueryType::UNKNOW(x) => x,
            QueryType::A => 1,
        }
    }

    pub fn from_num(num: u16) -> QueryType{
        match num{
            1 => QueryType::A,
            _ => QueryType::UNKNOW(num),
        }
    }
}