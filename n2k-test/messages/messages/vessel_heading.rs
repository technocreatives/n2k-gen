use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":127250,"Id":"vesselHeading","Length":8,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"sid","Name":"SID","Signed":false,"BitLength":8,"BitOffset":0,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"2","Id":"heading","Name":"Heading","Signed":false,"BitLength":16,"BitOffset":8,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"3","Id":"deviation","Name":"Deviation","Signed":true,"BitLength":16,"BitOffset":24,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"4","Id":"variation","Name":"Variation","Signed":true,"BitLength":16,"BitOffset":40,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"5","Id":"reference","Name":"Reference","Signed":false,"BitLength":2,"BitOffset":56,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"True"},{"Value":"1","Name":"Magnetic"},{"Value":"2","Name":"Error"},{"Value":"3","Name":"Null"}]},"Units":null,"Description":null},{"Order":"6","Id":"reserved","Name":"Reserved","Signed":false,"BitLength":6,"BitOffset":58,"Type":"Binary data","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":"Reserved"}]}}
pub struct VesselHeading {
    raw: [u8; 8usize],
}
impl core::convert::TryFrom<&[u8]> for VesselHeading {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 8usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 8usize,
                actual: payload.len(),
                pgn: 127250,
            });
        }
        let mut raw = [0u8; 8usize];
        raw.copy_from_slice(&payload[..8usize]);
        Ok(Self { raw })
    }
}
#[derive(Debug)]
pub enum Reference {
    XTrue,
    Magnetic,
    Error,
    Null,
    Other(u8),
}
impl core::convert::From<u8> for Reference {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::XTrue,
            1 => Self::Magnetic,
            2 => Self::Error,
            3 => Self::Null,
            v => Self::Other(v),
        }
    }
}
impl VesselHeading {
    pub fn sid_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[0usize..8usize].load_le::<u8>()
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
    pub fn heading_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[8usize..24usize].load_le::<u16>()
    }
    pub fn heading(&self) -> Option<f32> {
        let raw_value = self.heading_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    #[doc = "Unit: rad"]
    pub fn deviation_raw(&self) -> i16 {
        let value = self.raw.view_bits::<Lsb0>()[24usize..40usize].load_le::<u16>();
        i16::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn deviation(&self) -> Option<f32> {
        let raw_value = self.deviation_raw();
        if raw_value == i16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    #[doc = "Unit: rad"]
    pub fn variation_raw(&self) -> i16 {
        let value = self.raw.view_bits::<Lsb0>()[40usize..56usize].load_le::<u16>();
        i16::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn variation(&self) -> Option<f32> {
        let raw_value = self.variation_raw();
        if raw_value == i16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    pub fn reference_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[56usize..58usize].load_le::<u8>()
    }
    pub fn reference(&self) -> Reference {
        self.reference_raw().into()
    }
}
impl core::fmt::Debug for VesselHeading {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("VesselHeading")
            .field("sid", &self.sid())
            .field("heading", &self.heading())
            .field("deviation", &self.deviation())
            .field("variation", &self.variation())
            .field("reference", &self.reference())
            .finish()
    }
}
