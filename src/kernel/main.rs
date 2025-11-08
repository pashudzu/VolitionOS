/*
 * main.rs - Primary system entry point and core initialization
 * Copyright (C) 2025 pashudzu
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, version 3 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use subsystems::console::vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //TODO: print "[Valition OS] Hello World!""
    //Console.write_str();

    loop {}
    //boot_sequence::boot_os()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    //TODO: print "[Valition OS] Panic: {}", _info
    //Console.write_fmt();

    loop {}
}