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

use core::fmt::write;
use core::panic::PanicInfo;
//use volition_init::boot_sequence;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let mut output = String::new;
    fmt::write(&mut output, format_args!("[Valition OS] Hello World!"));
    //writeln!("Volition OS has loaded");
    loop {}
    //boot_sequence::boot_os()
}

// struct HStderr {
//     buffer: [u8, 512],
//     buffer_pos: usize,
// }
// 
// impl HStderr  {
//     pub fn new() -> Self {
//         HStderr {
//             buffer: [0;512],
//             buffer_pos: 0,
//         }
//     }
// }

#[panic_handler]
pub fn panic(_info: &PanicInfo) -> ! {
    // let mut host_stderr = HStderr::new();

    let mut output = String::new;
    fmt::write(&mut output, format_args!("[Valition OS] Panic: {}", _info));

    /* writeln!(host_stderr, "{}", info).ok(); */

    loop {}
}