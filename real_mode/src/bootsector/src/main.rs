#![feature(asm, global_asm)]

#![no_std]
#![no_main]

#![allow(dead_code)]

mod errors;

use core::panic::PanicInfo;
use stage_2::second_stage;

use shared::console::println;
use shared::{dap, utils, linker_symbol};

global_asm!(include_str!("bootstrap.s"));

#[no_mangle]
unsafe extern "C" fn rust_start(disk_number: u16) -> ! {
    println(b"Stage 1");

    let dap = dap::DiskAddressPacket::new(
        linker_symbol!(_rest_of_bootloader_start) as u16, 
        (linker_symbol!(_rest_of_bootloader_start) - linker_symbol!(_bootloader_start)) as u64,
        linker_symbol!(_rest_of_bootloader_end) - linker_symbol!(_rest_of_bootloader_start)
    );

    dap.perform_load(disk_number);

    second_stage();

    loop {
    	utils::hlt();
    }
}

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    println(b"[!] Rust Panic");
    loop {
    	utils::hlt()
    }
}