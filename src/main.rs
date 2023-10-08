//use std::fmt;

mod memaccess;


fn main() {
    let mut addr:  memaccess::AddressSpace = memaccess::AddressSpace::new();
    addr.set_u8_at(0, 255);
    addr.set_u8_at(1, 1);
    println!("i8: {}", addr.i8_at(0) );
    println!("u8: {:#x}", addr.u8_at(0) );
    println!("u16: {:#x}", addr.u16_at(0) );

    addr.set_u16_at(0, 12345);
    println!("u16: {:#x}", addr.u16_at(0) );
}

