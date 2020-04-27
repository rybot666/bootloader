#[inline(never)]
pub fn println(s: &[u8]) {
    print(s);
    print_char(b'\n');
}

pub fn print(s: &[u8]) {
    for &c in s {
        print_char(c);
    }
}

pub fn print_char(c: u8) {
    let ax = u16::from(c) | 0x0e00;
    unsafe {
        asm!("int 0x10" :: "{ax}"(ax) :: "intel" );
    }
}