use std::io::{Result};
use std::io::{Error, ErrorKind};


pub struct BytePacketBuffer {
    pub buf: [u8; 512],     //content   
    pub pos: usize          //current position (reading/writing)
}

impl BytePacketBuffer {
    // add code here
    pub fn new() -> Self{
        BytePacketBuffer{
            buf: [0; 512],
            pos: 0,
        }
    }

    //get rwpos
    pub fn pos(&self) -> usize{
        self.pos
    }

    //Increasing pos by steps bytes
    pub fn step(&mut self, steps: usize) -> Result<()>{
        self.pos += steps;
        Ok(())
    }

    //go to pos position
    fn seek(&mut self, pos: usize) ->  Result<()>{
        self.pos = pos;
        Ok(())
    }

    //read a single byte at self.pos position and increase pos.
    fn read(&mut self) -> Result<u8> {
        //maximum packet size is 512
        if self.pos >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        }
        let res = self.buf[self.pos];
        self.pos += 1;
        Ok(res)
    }

    //Reading a single byte at sepecified position
    fn get(&mut self, pos: usize) -> Result<u8> {
        if pos >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        }
        Ok(self.buf[pos])
    }

    fn get_range(&mut self, start:usize, len: usize) -> Result<&[u8]>{
        if start + len >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        }
        Ok(&self.buf[start..start+len as usize])
    }

    //read u16 value.
    pub fn read_u16(&mut self) -> Result<u16> {
        let res = ((try!(self.read()) as u16) << 8) | (try!(self.read()) as u16);
        Ok(res)
    }
    //read u32 value.
    pub fn read_u32(&mut self) -> Result<u32> {
        let res =   ((try!(self.read()) as u32) << 24) | 
                    ((try!(self.read()) as u32) << 16) |
                    ((try!(self.read()) as u32) << 8) |
                    ((try!(self.read()) as u32) << 0);
        Ok(res)
    }

    //The trickly part: Reading domain names, taking labels into consideration.
    pub fn read_qname(&mut self, outstr: &mut String) -> Result<()> {
        let mut pos = self.pos();

        //track wether or not we've jumped
        let mut jumped = false;
        //delimiter 
        let mut delim = "";
        loop{
            //label start with a len byte
            let len = try!(self.get(pos));
            //if len buffer position to a point past the current label
            //We don't need to touch it any further
            if (len & 0xC0) == 0xC0 {
                if !jumped {
                    try!(self.seek(pos + 2));
                }
                //read another byte, 
                let b2 = try!(self.get(pos+1)) as u16;
                let offset = (((len as u16) ^ 0xC0) << 8) | b2;
                pos = offset as usize;
                //jumped
                jumped = true;
            } else {
                pos += 1;
                if len == 0{
                    break;
                }

                outstr.push_str(delim);
                let str_buffer = try!(self.get_range(pos, len as usize));
                outstr.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());
                delim = ".";

                pos += len as usize;
            }
        }

        if !jumped {
            try!(self.seek(pos));
        }

        Ok(())
    }

    //Write [u8] val to buffer at current position.
    fn write(&mut self, val: u8) -> Result<()>{
        if self.pos >= 512 {
            return Err(Error::new(ErrorKind::InvalidInput, "End of buffer"));
        }

        self.buf[self.pos] = val;
        self.pos += 1;
        Ok(())
    }

    pub fn write_u8(&mut self, val: u8) -> Result<()>{
        try!(self.write(val));
        Ok(())
    }
    
    pub fn write_u16(&mut self, val: u16) -> Result<()>{
        try!(self.write((val>>8) as u8));
        try!(self.write((val & 0xFF) as u8));

        Ok(())
    }

    pub fn write_u32(&mut self, val: u32) -> Result<()> {
        try!(self.write(((val >> 24) & 0xFF) as u8));
        try!(self.write(((val >> 16) & 0xFF) as u8));
        try!(self.write(((val >> 8) & 0xFF) as u8));
        try!(self.write(((val >> 0) & 0xFF) as u8));

        Ok(())
    }

    pub fn write_qname(&mut self, qname: &str) -> Result<()> {
        let split_str = qname.split('.').collect::<Vec<&str>>();

        for label in split_str{
            let len = label.len();
            if len > 0x34{
                return Err(Error::new(ErrorKind::InvalidInput, "Single label exceeds 63 characters of length"));
            }

            try!(self.write_u8(len as u8));
            for b in label.as_bytes(){
                try!(self.write_u8(*b));
            }
        }

        try!(self.write_u8(0));
        Ok(())
    }

    pub fn set(&mut self, pos: usize, val: u8) -> Result<()> {
        self.buf[pos] = val;
        Ok(())
    }

    pub fn set_u16(&mut self, pos: usize, val: u16) -> Result<()> {
        try!(self.set(pos, (val >> 8) as u8));
        try!(self.set(pos+1, (val & 0xFF) as u8));

        Ok(())
    }
}