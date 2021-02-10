#![no_std]
#![no_main]

use {
    core::{
        panic::PanicInfo,
        sync::atomic::{compiler_fence, Ordering},
    },
    hal::{
        gpio::{p0::Parts, Input, Level, Pin, PullUp},
        nfct::Nfct,
        pac::{self, interrupt},
        time::*,
    },
    nrf52840_hal as hal, panic_rtt_target as _,
    rtt_target::{rprintln, rtt_init_print},
};

#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Init");

    let periph = hal::pac::Peripherals::take().unwrap();
    let nfc = Nfct::new(periph.NFCT);

    loop {}
}

#[interrupt]
fn NFCT() {
    rprintln!("INTERRUPT");
}
