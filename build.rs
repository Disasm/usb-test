use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use usb_device_generator::{UsbDirection, EndpointAddress};
use usb_device_generator::builder::{EndpointBuilder, DeviceBuilder, UsbVidPid};
use usb_device_generator::generator::{generate_file, DeviceEndpoint};
use usb_device_generator::cdc::{USB_CLASS_CDC, create_cdc_function};

fn create_cdc_device() -> DeviceBuilder {
    // TODO: allocate these
    let _comm_ep_addr = EndpointAddress::from_parts(1, UsbDirection::In);
    let _read_ep_addr = EndpointAddress::from_parts(2, UsbDirection::Out);
    let _write_ep_addr = EndpointAddress::from_parts(2, UsbDirection::In);
    let comm_ep: DeviceEndpoint = EndpointBuilder::new().input().interrupt().number(1).max_packet_size(8).interval(255).into();
    let read_ep: DeviceEndpoint = EndpointBuilder::new().output().bulk().number(2).max_packet_size(64).into();
    let write_ep: DeviceEndpoint = EndpointBuilder::new().input().bulk().number(2).max_packet_size(64).into();

    let mut device = DeviceBuilder::new(UsbVidPid(0x5824, 0x27dd))
        .manufacturer("Fake company")
        .product("Serial port")
        .serial_number("TEST")
        .device_class(USB_CLASS_CDC);

    create_cdc_function(&mut device, comm_ep, read_ep, write_ep);
    device
}

fn build_device(out_dir: &PathBuf) {
    let d = create_cdc_device();
    let config = d.build();
    generate_file(out_dir.join("generated.rs"), config).unwrap();
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
