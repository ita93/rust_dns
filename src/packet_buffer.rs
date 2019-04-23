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
    fn pos(&self) -> usize{
        self.pos
    }
}