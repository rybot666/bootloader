#![feature(asm, global_asm)]

#![no_std]
#![no_main]

#![allow(dead_code)]

use core::panic::PanicInfo;
use stage_2::second_stage;

mod dap;
mod console;
#[macro_use] mod macros;

use self::console::*;

global_asm!(include_str!("bootstrap.s"));

#[no_mangle]
unsafe extern "C" fn rust_start(disk_number: u16) -> ! {
    let dap = dap::DiskAddressPacket::new(
        linker_symbol!(_second_stage_start) as u16, 
        (linker_symbol!(_second_stage_start) - linker_symbol!(_bootloader_start)) as u64,
        linker_symbol!(_second_stage_end) - linker_symbol!(_second_stage_start)
    );

    dap.perform_load(disk_number);

    println(b"[#] Loaded Stage 2");

    let val = second_stage();

    println(b"[#] Called Stage 2");

    if val == 12345 {
        println(b"[#] Match");
    } else {
        println(b"[!] No Match");
    }

    loop { hlt(); }
}   

#[no_mangle]
pub extern "C" fn dap_load_failed() -> ! {
    println(b"[!] DAP Load Failed");
    loop {
        hlt()
    }
}

#[no_mangle]
pub extern "C" fn no_int13h_extensions() -> ! {
    println(b"[!] No int13h Extensions");
    loop {
        hlt()
    }
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    println(b"[!] Rust Panic");
    loop {
        hlt()
    }
}

#[inline(always)]
fn hlt() {
    unsafe {
        asm!("hlt" :::: "intel","volatile");
    }
}