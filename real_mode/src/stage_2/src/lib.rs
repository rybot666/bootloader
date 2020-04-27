#![feature(asm)]
#![no_std]

#[no_mangle]
pub fn second_stage() -> u16 {
	println(b"[S2] Called");
    return 12345;
}

#[inline(never)]
fn println(s: &[u8]) {
    print(s);
    print_char(b'\n');
}

fn print(s: &[u8]) {
    for &c in s {
        print_char(c);
    }
}

fn print_char(c: u8) {
    let ax = u16::from(c) | 0x0e00;
    unsafe {
        asm!("int 0x10" :: "{ax}"(ax) :: "intel" );
    }
}