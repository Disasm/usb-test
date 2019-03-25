#![no_std]
#![no_main]

/// CDC-ACM serial port example using polling in a busy loop.

extern crate panic_semihosting;

use cortex_m_rt::entry;
use stm32f1xx_hal::{prelude::*, stm32};

use usb_device::prelude::*;
use stm32f103xx_usb::UsbBus;
use usb_device::device::CustomStringDescriptorProvider;

mod cdc_acm;


include!(concat!(env!("OUT_DIR"), "/generated.rs"));

impl<B: ::usb_device::bus::UsbBus> CustomStringDescriptorProvider<B> for generated::GeneratedDevice {
}

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr
        .use_hse(8.mhz())
        .sysclk(48.mhz())
        .pclk1(24.mhz())
        .freeze(&mut flash.acr);

    assert!(clocks.usbclk_valid());

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let _gen_device = generated::GeneratedDevice;
    let usb_bus = UsbBus::usb_with_reset(dp.USB,
                                         &mut rcc.apb1, &clocks, &mut gpioa.crh, gpioa.pa12);

    let mut serial = cdc_acm::SerialPort::new(&usb_bus);

    let mut usb_dev = UsbDeviceBuilder::new(&usb_bus, UsbVidPid(0x5824, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(cdc_acm::USB_CLASS_CDC)
        .build();

    usb_dev.force_reset().expect("reset failed");

    loop {
        if !usb_dev.poll(&mut [&mut serial]) {
            continue;
        }

        let mut buf = [0u8; 64];

        match serial.read(&mut buf) {
            Ok(count) if count > 0 => {
                // Echo back in upper case
                for c in buf[0..count].iter_mut() {
                    if 0x61 <= *c && *c <= 0x7a {
                        *c &= !0x20;
                    }
                }

                serial.write(&buf[0..count]).ok();
            },
            _ => { },
        }
    }
}
