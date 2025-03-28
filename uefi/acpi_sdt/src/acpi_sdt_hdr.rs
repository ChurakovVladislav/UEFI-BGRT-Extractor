use core::fmt::{self, Display};
use uefi::Char8;

#[derive(Clone, Copy, Debug)]
#[repr(C, packed(1))]
pub struct EfiAcpiSdtHeader {
    signature: u32,
    lenght: u32,
    revision: u8,
    checksum: u8,
    oem_id: [Char8; 6],
    oem_table_id: [Char8; 8],
    oem_revision: u32,
    creator_id: u32,
    creator_revision: u32,
}

impl EfiAcpiSdtHeader {
    pub fn signature(&self) -> u32 {
        self.signature
    }

    pub fn char8_signature(&self) -> [Char8; 4] {
        let bytes = self.signature.to_be_bytes();

        [
            Char8::from(bytes[3]),
            Char8::from(bytes[2]),
            Char8::from(bytes[1]),
            Char8::from(bytes[0]),
        ]
    }

    pub fn lenght(&self) -> u32 {
        self.lenght
    }

    pub fn oem_revision(&self) -> u32 {
        self.oem_revision
    }

    pub fn creator_id(&self) -> u32 {
        self.creator_id
    }

    pub fn creator_revision(&self) -> u32 {
        self.creator_revision
    }
}

impl Display for EfiAcpiSdtHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "  Table (signature (0x{:x}): {:?})",
            self.signature(),
            self.char8_signature()
        )?;
        writeln!(f, "  Lenght: {}", self.lenght())?;
        writeln!(f, "  Revision: {}", self.revision)?;
        writeln!(f, "  Checksum: {}", self.checksum)?;
        writeln!(f, "  OEM ID: {:?}", self.oem_id)?;
        writeln!(f, "  OEM Table ID: {:?}", self.oem_table_id)?;
        writeln!(f, "  OEM Revision: 0x{:x}", self.oem_revision())?;
        writeln!(f, "  Creator ID: 0x{:x}", self.creator_id())?;
        writeln!(f, "  Creator revision: 0x{:x}", self.creator_revision())
    }
}
