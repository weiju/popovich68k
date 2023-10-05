mod memaccess;

fn main() {
    let mut addr:  memaccess::AddressSpace = memaccess::AddressSpace::new();
    addr.set_ubyte_at(0, 255);
    println!("Hello, world! {}", addr.sbyte_at(0) );
}
