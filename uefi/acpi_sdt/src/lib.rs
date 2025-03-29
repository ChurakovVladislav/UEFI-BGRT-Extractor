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

/// EfiAcpiTableVersion defines the supported versions of ACP
#[repr(u32)]
pub enum EfiAcpiTableVersion {
    None = 1,
    Version1 = (1 << 1),
    Version2 = (1 << 2),
    Version3 = (1 << 3),
    Version4 = (1 << 4),
    Version5 = (1 << 5),
}

/// EfiAcpiDataType defines the different types of data that can be used in ACPI
#[repr(u32)]
pub enum EfiAcpiDataType {
    /// The None indicates that the specified ACPI object does not support the specified option.
    None = 0,
    /// The Option indicates that the option is an ACPI opcode.
    Option,
    /// The NameString indicates that the option is an ACPI name string.
    NameString,
    /// The Op indicates that the option is an ACPI opcode.
    /// The Open() function can be used to manipulate the contents of this ACPI opcode.
    Op,
    /// The Unit indicates that the option is an unsigned integer. 
    /// The size of the integer is indicated by DataSize .
    Unit,
    /// The String indicates that the option is a string whose length is indicated by DataSize . 
    String,
    /// The Child indicates that the opcode has child data, pointed to by Data , with the size DataSize.
    Child,
}

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
    /// A bit map containing all the ACPI versions supported by this protocol
    pub fn version(&self) -> u32 {
        self.acpi_version
    }

    ///  This function uses the ACPI SDT protocol to search an ACPI table
    ///  with a given signature.
    pub fn locate_table_by_signature<T: AcpiHeadeds + Copy>(&self) -> Result<T> {
        let mut index = 0;
        let mut version: EfiAcpiTableVersion = EfiAcpiTableVersion::None;
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
