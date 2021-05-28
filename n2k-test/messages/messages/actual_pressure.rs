use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":130314,"Id":"actualPressure","Length":8,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"sid","Name":"SID","Signed":false,"BitLength":8,"BitOffset":0,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"2","Id":"instance","Name":"Instance","Signed":false,"BitLength":8,"BitOffset":8,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"3","Id":"source","Name":"Source","Signed":false,"BitLength":8,"BitOffset":16,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"Atmospheric"},{"Value":"1","Name":"Water"},{"Value":"2","Name":"Steam"},{"Value":"3","Name":"Compressed Air"},{"Value":"4","Name":"Hydraulic"}]},"Units":null,"Description":null},{"Order":"4","Id":"pressure","Name":"Pressure","Signed":true,"BitLength":32,"BitOffset":24,"Type":"Pressure (hires)","Resolution":0.1,"EnumValues":{"EnumPair":[]},"Units":"dPa","Description":null}]}}
pub struct ActualPressure {
    raw: [u8; 8usize],
}
impl core::convert::TryFrom<&[u8]> for ActualPressure {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 8usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 8usize,
                actual: payload.len(),
                pgn: 130314,
            });
        }
        let mut raw = [0u8; 8usize];
        raw.copy_from_slice(&payload[..8usize]);
        Ok(Self { raw })
    }
}
#[derive(Debug)]
pub enum Source {
    Atmospheric,
    Water,
    Steam,
    CompressedAir,
    Hydraulic,
    Other(u8),
}
impl core::convert::From<u8> for Source {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Atmospheric,
            1 => Self::Water,
            2 => Self::Steam,
            3 => Self::CompressedAir,
            4 => Self::Hydraulic,
            v => Self::Other(v),
        }
    }
}
impl ActualPressure {
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
    pub fn instance_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[8usize..16usize].load_le::<u8>()
    }
    pub fn instance(&self) -> Option<u8> {
        let raw_value = self.instance_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn source_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[16usize..24usize].load_le::<u8>()
    }
    pub fn source(&self) -> Source {
        self.source_raw().into()
    }
    #[doc = "Unit: dPa"]
    pub fn pressure_raw(&self) -> i32 {
        let value = self.raw.view_bits::<Lsb0>()[24usize..56usize].load_le::<u32>();
        i32::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn pressure(&self) -> Option<f32> {
        let raw_value = self.pressure_raw();
        if raw_value == i32::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.1 as f32))
        }
    }
}
impl core::fmt::Debug for ActualPressure {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ActualPressure")
            .field("sid", &self.sid())
            .field("instance", &self.instance())
            .field("source", &self.source())
            .field("pressure", &self.pressure())
            .finish()
    }
}
