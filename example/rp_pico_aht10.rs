#![no_std]
#![no_main]

extern crate alloc;

use aht10_embedded::AHT10;
use alloc::boxed::Box;
use alloc::format;
use core::panic::PanicInfo;
use cortex_m::delay::Delay;
use cortex_m::interrupt::free;
use embedded_alloc::Heap;
use embedded_hal::digital::v2::ToggleableOutputPin;

use hal::{clocks::init_clocks_and_plls, clocks::ClockSource, pac, usb::UsbBus, Sio, Watchdog};
use rp_pico::entry;
use rp_pico::hal;

mod pico_usb_serial;
mod serial_buffer;
mod simple_buffer;
use pico_usb_serial::PicoUsbSerial;
use serial_buffer::SerialBuffer;
use simple_buffer::SimpleBuffer;

use fugit::RateExtU32;

#[global_allocator]
static HEAP: Heap = Heap::empty();
const HEAP_SIZE: usize = 4069;

#[entry]
fn main() -> ! {
    // Initialize the allocator BEFORE you use it
    init_heap();

    let mut pac = pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    let clocks = init_clocks_and_plls(
        rp_pico::XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = Sio::new(pac.SIO);
    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let core = pac::CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST, clocks.system_clock.get_freq().to_Hz());

    PicoUsbSerial::init(
        UsbBus::new(
            pac.USBCTRL_REGS,
            pac.USBCTRL_DPRAM,
            clocks.usb_clock,
            true,
            &mut pac.RESETS,
        ),
        Box::new(SimpleBuffer::new(1024)),
        "RP Pico",
        "Temp station",
        "Serial",
    )
    .expect("Failed to init Serial");

    let mut led = pins.led.into_push_pull_output();

    let serial = PicoUsbSerial::get_serial().expect("Failed to get serial!");

    let i2c_sda = pins.gpio2.into_function();
    let i2c_scl = pins.gpio3.into_function();
    let i2c1 = hal::i2c::I2C::i2c1(
        pac.I2C1,
        i2c_sda,
        i2c_scl,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );

    let mut aht = AHT10::new(i2c1);

    match aht.initialize() {
        Ok(_) => (),
        Err(e) => {
            delay.delay_ms(1000);
            free(|_cs| {
                serial
                    .write(format!("Unable to initialize the AHT\r\nError: {:?}\r\n", e).as_bytes())
            })
            .ok();

            loop {}
        }
    }

    loop {
        led.toggle().ok();

        free(|_cs| match aht.read_data(&mut delay) {
            Ok(data) => {
                serial
                    .write(
                        format!(
                            "Read data:\r\n\tTemperature: {} C\r\n\tHumidity: {} %RH\r\n\tStatus: {:?}\r\n",
                            data.temperature_celsius(),
                            data.humidity(),
                            aht.read_status()
                        )
                        .as_bytes(),
                    )
                    .ok();
            }
            Err(e) => {
                serial
                    .write(format!("Unable to read the AHT data\r\nError: {:?}\r\n", e).as_bytes())
                    .ok();
            }
        });

        delay.delay_ms(3000);
    }
}

fn init_heap() -> () {
    use core::mem::MaybeUninit;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) };
}

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    let ser = PicoUsbSerial::get_serial();

    if ser.is_ok() {
        let ser = ser.unwrap();
        let _ = ser.write(b"===== PANIC =====\r\n");
    }

    loop {}
}
