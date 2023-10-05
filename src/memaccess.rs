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

    pub fn ubyte_at(self, addr: u32) -> u8 {
        return self.chipmem[addr as usize];
    }
    pub fn set_ubyte_at(&mut self, addr: u32, value: u8) {
        return self.chipmem[addr as usize] = value;
    }

    pub fn sbyte_at(self, addr: u32) -> i8 {
        return self.chipmem[addr as usize] as i8;
    }
}
