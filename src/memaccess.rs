/*
 * memaccess.rs - Memory access for 68k systems
 * Big Endian memory model
 */

pub struct AddressSpace {
    // 512 KB
    chipmem: [u8; 524288]
    //fastmem: [u8; 524288]
}

impl AddressSpace {
    pub fn new() -> AddressSpace {
        AddressSpace { chipmem: [0; 524288]}
    }
    // 8 Bit accessors
    pub fn u8_at(&mut self, addr: u32) -> u8 {
        return self.chipmem[addr as usize];
    }
    pub fn i8_at(&mut self, addr: u32) -> i8 {
        return self.chipmem[addr as usize] as i8;
    }
    pub fn set_u8_at(&mut self, addr: u32, value: u8) {
        self.chipmem[addr as usize] = value;
    }

    // 16 Bit accessors
    pub fn u16_at(&mut self, addr: u32) -> u16 {
        let a: u16 = (self.chipmem[addr as usize]).into();
        let b: u16 = (self.chipmem[(addr + 1) as usize]).into();
        return (a << 8) | b;
    }

    pub fn set_u16_at(&mut self, addr: u32, value: u16) {
        let a: u8 = (value >> 8) as u8;
        let b: u8 = (value & 0xff) as u8;
        self.chipmem[addr as usize] = a;
        self.chipmem[(addr + 1) as usize] = b;
    }
}
