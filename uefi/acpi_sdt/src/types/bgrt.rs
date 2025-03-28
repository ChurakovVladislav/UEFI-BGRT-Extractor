use crate::acpi_sdt_hdr::EfiAcpiSdtHeader;
use crate::{signature_32, AcpiHeadeds};
use core::fmt::{self, Display};

/// "BGRT" Boot Graphics Resource Table
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct EfiAcpiBootGraphicsResourceTable {
    header: EfiAcpiSdtHeader,
    version: u16,
    status: u8,
    image_type: u8,
    image_address: u64,
    image_offset_x: u32,
    image_offset_y: u32,
}

impl EfiAcpiBootGraphicsResourceTable {
    pub fn address(&self) -> u64 {
        self.image_address
    }

    pub fn offset(&self) -> (u32, u32) {
        (self.image_offset_x, self.image_offset_y)
    }
}

impl AcpiHeadeds for EfiAcpiBootGraphicsResourceTable {
    const ACPI_TYPE: u32 = signature_32!('B', 'G', 'R', 'T');

    fn get_header(&self) -> EfiAcpiSdtHeader {
        self.header
    }
}

impl Display for EfiAcpiBootGraphicsResourceTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "  [BGRT] Header: {}", self.header)?;
        writeln!(f, "  [BGRT] Version ID: {}", self.version)?;
        writeln!(f, "  [BGRT] Image Type: {}", self.image_type)?;
        writeln!(f, "  [BGRT] Image Address: 0x{:x}", self.image_address)?;
        writeln!(f, "  [BGRT] Image OffsetX: {}", self.image_offset_x)?;
        writeln!(f, "  [BGRT] Image OffsetY: {}", self.image_offset_y)
    }
}
