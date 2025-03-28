#![no_std]
use core::ffi::c_void;
use core::ptr;
use uefi::prelude::*;
use uefi::proto::unsafe_protocol;
use uefi::{Char8, Result};

pub mod acpi_sdt_hdr;
use crate::acpi_sdt_hdr::EfiAcpiSdtHeader;

mod types;
pub use types::*;

pub mod signature;

type EfiAcpiTableVersion = u32;
type EfiAcpiDataType = u32;

type EfiAcpiNotificationFn = unsafe extern "efiapi" fn(
    table: *mut *mut EfiAcpiSdtHeader,
    version: EfiAcpiTableVersion,
    table_key: usize,
) -> Status;

/// provides services for creating ACPI system description tables.
#[derive(Debug)]
#[repr(C)]
#[unsafe_protocol("eb97088e-cfdf-49c6-be4b-d906a5b20e86")]
pub struct AcpiSdt {
    acpi_version: u32,
    get_acpi_table: unsafe extern "efiapi" fn(
        index: usize,
        table: *mut *mut EfiAcpiSdtHeader,
        version: *mut EfiAcpiTableVersion,
        table_key: *mut usize,
    ) -> Status,
    register_notify:
        unsafe extern "efiapi" fn(register: bool, notification: EfiAcpiNotificationFn) -> Status,
    open: unsafe extern "efiapi" fn(buffer: *mut c_void, handle: *mut Handle) -> Status,
    open_sdt: unsafe extern "efiapi" fn(take_key: usize, handle: *mut Handle) -> Status,
    close: unsafe extern "efiapi" fn(handle: Handle) -> Status,
    get_child: unsafe extern "efiapi" fn(parent_handle: Handle, handle: *mut Handle) -> Status,
    get_option: unsafe extern "efiapi" fn(
        handle: Handle,
        index: usize,
        data_type: *mut EfiAcpiDataType,
        data: *mut *mut c_void,
        data_size: *mut usize,
    ) -> Status,
    set_option: unsafe extern "efiapi" fn(
        handle: Handle,
        index: usize,
        data: *mut c_void,
        data_size: usize,
    ) -> Status,
    find_path: unsafe extern "efiapi" fn(
        handle_in: Handle,
        acpi_path: *mut c_void,
        handle_out: *mut Handle,
    ) -> Status,
}

impl AcpiSdt {
    ///  This function uses the ACPI SDT protocol to search an ACPI table
    ///  with a given signature.
    pub fn locate_table_by_signature<T: AcpiHeadeds + Copy>(&self) -> Result<T> {
        let mut index = 0;
        let mut version: EfiAcpiTableVersion = 0;
        let mut acpi_head: *mut EfiAcpiSdtHeader = ptr::null_mut();
        let mut table_key: usize = 0;

        loop {
            let (status, head) = unsafe {
                let status =
                    (self.get_acpi_table)(index, &mut acpi_head, &mut version, &mut table_key);
                (status, *(acpi_head as *mut T))
            };

            if status.is_success() {
                index += 1;

                if head.get_header().signature() == T::ACPI_TYPE {
                    break Ok(head);
                }
            } else {
                break Err(status.into());
            }
        }
    }
}

pub trait AcpiHeadeds {
    const ACPI_TYPE: u32 = 0u32;

    fn get_header(&self) -> EfiAcpiSdtHeader;
}

impl AcpiHeadeds for EfiAcpiSdtHeader {
    fn get_header(&self) -> EfiAcpiSdtHeader {
        *self
    }
}
