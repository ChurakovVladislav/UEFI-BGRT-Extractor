#![no_main]
#![no_std]
extern crate alloc;

use alloc::slice;
use uefi::prelude::*;
use uefi::{boot, println, Result};

use acpi_sdt::{AcpiSdt, EfiAcpiBootGraphicsResourceTable};
use uefi::boot::ScopedProtocol;
use uefi::fs::FileSystem;
use uefi::proto::ProtocolPointer;

fn locate_protocol<P: ProtocolPointer + ?Sized>() -> ScopedProtocol<P> {
    use uefi::boot::{OpenProtocolAttributes, OpenProtocolParams};

    let handle = boot::get_handle_for_protocol::<P>().expect("missing protocol");

    unsafe {
        boot::open_protocol::<P>(
            OpenProtocolParams {
                handle,
                agent: boot::image_handle(),
                controller: None,
            },
            // For this test, don't open in exclusive mode. That
            // would break the connection between stdout and the
            // video console.
            OpenProtocolAttributes::GetProtocol,
        )
        .expect("failed to open")
    }
}

fn save_bgrt_image() -> Result {
    let table = locate_protocol::<AcpiSdt>();

    // Find the BGRT table
    let bgrt_table = table
        .locate_table_by_signature::<EfiAcpiBootGraphicsResourceTable>()
        .map_err(|_| Status::NOT_FOUND)?;

    println!("{}", bgrt_table);
    let addr = bgrt_table.address();
    let (x, y) = bgrt_table.offset();
    let len = (x * y) as usize;

    let slice: &[u8] = unsafe { slice::from_raw_parts(addr as *const u8, len) };

    boot::get_image_file_system(boot::image_handle()).map(|file_system| {
        let mut fs = FileSystem::new(file_system);
        let _ = fs.write(cstr16!("BGRTImage.bmp"), slice);
    })
}

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    save_bgrt_image().status()
}
