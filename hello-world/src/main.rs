#![no_std]
#![no_main]
#![allow(unused_assignments)]

const GPIO_FSEL0: u32 = 0x2020_0000;
const GPIO_FSEL1: u32 = 0x2020_0004;
const GPIO_FSEL2: u32 = 0x2020_0008;

const GPIO_SET0: u32 = 0x2020_001C;
const GPIO_CLR0: u32 = 0x2020_0028;

use core::arch::asm;
use core::panic::PanicInfo;

mod boot {
    use core::arch::global_asm;

    global_asm!(".section .text._start");
}

struct GPIO;

impl GPIO {
    fn set_output(pin: u32) {
        let reg = pin / 10;
        let register = match reg {
            0 => GPIO_FSEL0,
            1 => GPIO_FSEL1,
            2 => GPIO_FSEL2,
            _ => panic!("Invalid pin"),
        };

        let mut val: u32 = 0;

        unsafe {
            val = core::ptr::read_volatile(register as *const u32);
        }

        let mut mask: u32 = 0b111;
        let pin_num = pin % 10;
        mask = mask << (pin_num * 3);
        val = val & !mask;
        val |= 1 << (pin_num * 3);

        unsafe {
            core::ptr::write_volatile(register as *mut u32, val);
        }
    }

    pub fn set(pin: u32) {
        let bit_pos = pin;
        let mut val: u32 = 0;
        unsafe {
            val = core::ptr::read_volatile(GPIO_SET0 as *const u32);
        }
        val |= 1 << bit_pos;
        unsafe {
            core::ptr::write_volatile(GPIO_SET0 as *mut u32, val);
        }
    }
    pub fn clear(pin: u32) {
        let bit_pos = pin;
        let mut val: u32 = 0;
        unsafe {
            val = core::ptr::read_volatile(GPIO_CLR0 as *const u32);
        }
        val |= 1 << bit_pos;
        unsafe {
            core::ptr::write_volatile(GPIO_CLR0 as *mut u32, val);
        }
    }
}

#[link_section = ".text._start"]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    GPIO::set_output(21);
    loop {
        GPIO::set(21);

        for _ in 1..50000 {
            unsafe {
                asm!("nop");
            }
        }

        GPIO::clear(21);

        for _ in 1..50000 {
            unsafe {
                asm!("nop");
            }
        }
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
