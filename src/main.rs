#![no_std]
#![no_main]
#![allow(dead_code)]

use core::ptr::{write_volatile as write, read_volatile as read};

const GPIO_BASE_ADDR: *mut u32 = 0xFF_63_44_00 as *mut u32;

const GPIOA_MODE_SELECT: isize = 0x13;
const GPIOA_OUTPUT: isize = 0x14;
const GPIOA_INPUT: isize = 0x15;

const GPIOX_MODE_SELECT: isize = 0x16;
const GPIOX_OUTPUT: isize = 0x17;
const GPIOX_INPUT: isize = 0x18;

const GPIOH_MODE_SELECT: isize = 0x19;
const GPIOH_OUTPUT: isize = 0x1A;
const GPIOH_INPUT: isize = 0x1B;

const GPIOZ_MODE_SELECT: isize = 0x1c;
const GPIOZ_OUTPUT: isize = 0x1d;
const GPIOZ_INPUT: isize = 0x1e;

const GPIOC_MODE_SELECT: isize = 0x20;
const GPIOC_OUTPUT: isize = 0x21;
const GPIOC_INPUT: isize = 0x22;


#[derive(Copy, Clone)]
enum GPIOX {
    Pin0, Pin1, Pin2, Pin3, Pin4, Pin5, Pin6, Pin7, Pin8, Pin9, Pin10, Pin11, Pin12, Pin13, Pin14, Pin15, Pin16, Pin17, Pin18, Pin19
}

impl GPIOX {
    unsafe fn set_mode(&self, input: bool) {
        let pin = *self as u32;
        let prev_value = read(GPIO_BASE_ADDR.offset(GPIOX_MODE_SELECT));
        write(GPIO_BASE_ADDR.offset(GPIOX_MODE_SELECT), if input {
            prev_value | (1 << pin)
        } else {
            prev_value & !(1 << pin)
        })
    }
    unsafe fn set_output(&self, high: bool) {
        let pin = *self as u32;
        let prev_value = read(GPIO_BASE_ADDR.offset(GPIOX_OUTPUT));
        write(GPIO_BASE_ADDR.offset(GPIOX_OUTPUT),
        if high {
            prev_value | (1 << pin)
        } else {
            prev_value & !(1 << pin)
        }
    )
    }
}

#[link_section = ".text._start"]
#[no_mangle]
pub unsafe extern "C" fn _start() -> ! {
    let pin = GPIOX::Pin13; // с 4 пином не проканало почему-то
    pin.set_mode(false);
    loop {
        pin.set_output(true);
        nop_sleep(100_000_000);

        pin.set_output(false);
        nop_sleep(100_000_000);
    }
}

fn nop_sleep(cycles: u32) {
    unsafe {
        for _ in 1..cycles {
            core::arch::asm!("nop");
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
