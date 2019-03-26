use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use usb_device_generator::{UsbDirection, EndpointType};
use usb_device_generator::builder::{EndpointBuilder, DeviceBuilder, UsbVidPid};
use usb_device_generator::generator::{generate_file, DeviceEndpoint};
use usb_device_generator::cdc::{USB_CLASS_CDC, create_cdc_function};
use usb_device_generator::endpoint::{DeviceAllocator, EndpointBuilderEx, DeviceBuilderEx};


fn create_cdc_device(allocator: &mut DeviceAllocator) -> DeviceBuilder {
    let mut device = DeviceBuilder::new(UsbVidPid(0x5824, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC)
        .allocate(allocator);

    let comm_ep: DeviceEndpoint = EndpointBuilder::new().direction(UsbDirection::In).ep_type(EndpointType::Interrupt).max_packet_size(8).interval(255).allocate(allocator).into();
    let read_ep: DeviceEndpoint = EndpointBuilder::new().direction(UsbDirection::Out).ep_type(EndpointType::Bulk).max_packet_size(64).allocate(allocator).into();
    let write_ep: DeviceEndpoint = EndpointBuilder::new().direction(UsbDirection::In).ep_type(EndpointType::Bulk).max_packet_size(64).allocate(allocator).into();

    create_cdc_function(&mut device, comm_ep, read_ep, write_ep);
    device
}

fn build_device(out_dir: &PathBuf) {
    let mut allocator = DeviceAllocator::new();
    let d = create_cdc_device(&mut allocator);
    let config = d.build();
    let device_config = allocator.into();
    generate_file(out_dir.join("generated.rs"), config, device_config).unwrap();
}

fn main() {
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=memory.x");

    build_device(out);

    println!("cargo:rerun-if-changed=build.rs");
}
