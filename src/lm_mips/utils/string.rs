#[derive(Clone, Debug, PartialEq)]
pub struct LmString{
    buffer: [char; 48],
    capacity: usize,
    size: usize
}

impl LmString{
    pub fn new_lmstring() -> LmString{
        LmString{
            buffer: ['\0'; 48],
            capacity: 48,
            size: 0
        }
    }
    pub fn len(&self) -> usize{
        self.size
    }
    pub fn _clear(&mut self) -> (){
        for i in 0..self.size{
            self.buffer[i] = '\0';
        }
        self.size = 0;
    }
    pub fn append_char(&mut self, char: char) -> bool{
        if self.size >= self.capacity{
            return false
        }
        self.buffer[self.size] = char;
        self.size += 1;
        true
    }
    pub fn append_str(&mut self, src: &str) -> bool{
        if self.capacity - self.size < src.len(){
            return false
        }
        for i in src.chars(){
            self.buffer[self.size] = i;
            self.size += 1;
        }
        true
    }
    pub fn append_string(&mut self, src: &LmString) -> bool{
        if self.capacity - self.size < src.len(){
            return false
        }
        for i in 0..self.size{
            self.buffer[self.size] = src.buffer[i];
            self.size += 1;
        }
        true
    }
    pub fn data(&self) -> &[char]{
        return &self.buffer;
    }
    pub fn num_to_str(&mut self, mut num: u64) -> (){
        let digit_num: usize;
        let mut i: usize;
        
        self.buffer[0] = '0';
        self.buffer[1] = 'x';
    
        if num < 0x100{
            digit_num = 2;
        }
        else if num < 0x10000{
            digit_num = 4;
        }
        else if num < 0x100000000{
            digit_num = 8;
        }
        else {
            digit_num = 16;
        }
    
        i = digit_num;
        while i > 0{
            match num & 0xf{
                0..=9 => self.buffer[i + 1] = ((num & 0xf) + 0x30) as u8 as char,
                _ => self.buffer[i + 1] = ((num & 0xf) + 0x57) as u8 as char
            }
            num >>= 4;
            i-=1;
        }
    }    
}