#![no_std]
#![no_main]

use cortex_m_rt::entry;
use embassy_stm32::{Config, bind_interrupts, peripherals, usart};
use embassy_stm32::usart::Uart;
use embassy_stm32::rcc::Sysclk;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
        loop {}
}

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
    //info!("Hello World!");
    putc('1');

    // QEMU doesn't emulate the reset/clock controller (RCC), so don't try to
    // initialize it.
    let mut config: Config = Default::default();
    config.rcc.pll = None;
    config.rcc.hse = None;
    config.rcc.sys = Sysclk::HSI;
    let p = embassy_stm32::init(config);

    putc('2');

    let config = usart::Config::default();
    let mut usart = Uart::new_blocking(p.USART1, p.PA10, p.PA9, config).unwrap();
    putc('3');

    /*unwrap!(*/usart.blocking_write(b"Hello Embassy World!\r\n")/*)*/;
    //info!("wrote Hello, starting echo");
    putc('4');

    let mut buf = [0u8; 1];
    loop {
        usart.blocking_read(&mut buf);
        usart.blocking_write(&buf);
    }
}
