use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":65359,"Id":"seatalkPilotHeading","Length":8,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"manufacturerCode","Name":"Manufacturer Code","Signed":false,"BitLength":11,"BitOffset":0,"Type":"Manufacturer code","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":"Raymarine"},{"Order":"2","Id":"reserved","Name":"Reserved","Signed":false,"BitLength":2,"BitOffset":11,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"3","Id":"industryCode","Name":"Industry Code","Signed":false,"BitLength":3,"BitOffset":13,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":"Marine Industry"},{"Order":"4","Id":"sid","Name":"SID","Signed":false,"BitLength":8,"BitOffset":16,"Type":"Binary data","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"5","Id":"headingTrue","Name":"Heading True","Signed":false,"BitLength":16,"BitOffset":24,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"6","Id":"headingMagnetic","Name":"Heading Magnetic","Signed":false,"BitLength":16,"BitOffset":40,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"7","Id":"reserved","Name":"Reserved","Signed":false,"BitLength":8,"BitOffset":56,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null}]}}
pub struct SeatalkPilotHeading {
    raw: [u8; 8usize],
}
impl core::convert::TryFrom<&[u8]> for SeatalkPilotHeading {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 8usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 8usize,
                actual: payload.len(),
                pgn: 65359,
            });
        }
        let mut raw = [0u8; 8usize];
        raw.copy_from_slice(&payload[..8usize]);
        Ok(Self { raw })
    }
}
impl SeatalkPilotHeading {
    #[doc = "Description: Raymarine"]
    pub fn manufacturer_code_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[0usize..11usize].load_le::<u16>()
    }
    pub fn manufacturer_code(&self) -> Option<u16> {
        let raw_value = self.manufacturer_code_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    #[doc = "Description: Marine Industry"]
    pub fn industry_code_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[13usize..16usize].load_le::<u8>()
    }
    pub fn industry_code(&self) -> Option<u8> {
        let raw_value = self.industry_code_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn sid_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[16usize..24usize].load_le::<u8>()
    }
    pub fn sid(&self) -> Option<u8> {
        let raw_value = self.sid_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    #[doc = "Unit: rad"]
    pub fn heading_true_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[24usize..40usize].load_le::<u16>()
    }
    pub fn heading_true(&self) -> Option<f32> {
        let raw_value = self.heading_true_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    #[doc = "Unit: rad"]
    pub fn heading_magnetic_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[40usize..56usize].load_le::<u16>()
    }
    pub fn heading_magnetic(&self) -> Option<f32> {
        let raw_value = self.heading_magnetic_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
}
impl core::fmt::Debug for SeatalkPilotHeading {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("SeatalkPilotHeading")
            .field("manufacturer_code", &self.manufacturer_code())
            .field("industry_code", &self.industry_code())
            .field("sid", &self.sid())
            .field("heading_true", &self.heading_true())
            .field("heading_magnetic", &self.heading_magnetic())
            .finish()
    }
}
