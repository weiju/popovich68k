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

    // 32 Bit accessors
    pub fn u32_at(&mut self, addr: u32) -> u32 {
        let a: u32 = (self.chipmem[addr as usize]).into();
        let b: u32 = (self.chipmem[(addr + 1) as usize]).into();
        let c: u32 = (self.chipmem[(addr + 2) as usize]).into();
        let d: u32 = (self.chipmem[(addr + 3) as usize]).into();
        return (a << 24) | (b << 16) | (c << 8) | d;
    }

    pub fn set_u32_at(&mut self, addr: u32, value: u32) {
        let a: u8 = (value >> 24) as u8;
        let b: u8 = ((value >> 16) & 0xff) as u8;
        let c: u8 = ((value >> 8) & 0xff) as u8;
        let d: u8 = (value & 0xff) as u8;

        self.chipmem[addr as usize] = a;
        self.chipmem[(addr + 1) as usize] = b;
        self.chipmem[(addr + 2) as usize] = c;
        self.chipmem[(addr + 3) as usize] = d;
    }

}

// This part only gets compiled in test scenarios
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let addr: AddressSpace = AddressSpace::new();
        assert_eq!(0, addr.chipmem[0]);
    }

    #[test]
    fn test_u8_at() {
        let mut addr: AddressSpace = AddressSpace::new();
        addr.chipmem[0] = 12;
        assert_eq!(12, addr.u8_at(0));
    }

    #[test]
    fn test_i8_at() {
        let mut addr: AddressSpace = AddressSpace::new();
        addr.chipmem[0] = 254;
        assert_eq!(-2, addr.i8_at(0));
    }

    #[test]
    fn test_set_u8_at() {
        let mut addr: AddressSpace = AddressSpace::new();
        addr.set_u8_at(0, 13);
        assert_eq!(13, addr.u8_at(0));
    }

    #[test]
    fn test_u16_at() {
        let mut addr: AddressSpace = AddressSpace::new();
        addr.chipmem[0] = 0;
        addr.chipmem[1] = 32;
        assert_eq!(32, addr.u16_at(0));
    }

    #[test]
    fn test_set_u16_at() {
        let mut addr: AddressSpace = AddressSpace::new();
        addr.set_u16_at(0, 32123);
        assert_eq!(32123, addr.u16_at(0));
    }

    #[test]
    fn test_u32_at() {
        let mut addr: AddressSpace = AddressSpace::new();
        addr.chipmem[0] = 0;
        addr.chipmem[1] = 0;
        addr.chipmem[2] = 0;
        addr.chipmem[3] = 32;
        assert_eq!(32, addr.u32_at(0));
    }

    #[test]
    fn test_set_u32_at() {
        let mut addr: AddressSpace = AddressSpace::new();
        addr.set_u32_at(0, 32123123);
        assert_eq!(32123123, addr.u32_at(0));
    }
}
