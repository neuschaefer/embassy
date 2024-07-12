#![no_std]
#![no_main]

use cortex_m_rt::entry;
use defmt::*;
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::{bind_interrupts, peripherals, usart};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USART1 => usart::InterruptHandler<peripherals::USART1>;
});

fn putc(c: char) {
    use core::sync::atomic::Ordering;
    use core::sync::atomic::compiler_fence;
    unsafe {
        let usart = 0x40011000 as *mut u32;
        *usart.add(1) = c as u32;
        compiler_fence(Ordering::SeqCst);
    }
}

#[entry]
fn main() -> ! {
    putc('0');
    let p = embassy_stm32::init(Default::default());
    putc('1');

    let config = Config::default();
    let mut usart = Uart::new_blocking(p.USART1, p.PD9, p.PD8, config).unwrap();
    putc('2');

    unwrap!(usart.blocking_write(b"Hello Embassy World!\r\n"));
    putc('3');


    loop {
        putc('A');
    }
}
