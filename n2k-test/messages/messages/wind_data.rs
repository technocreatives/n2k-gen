use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":130306,"Id":"windData","Length":8,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"sid","Name":"SID","Signed":false,"BitLength":8,"BitOffset":0,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"2","Id":"windSpeed","Name":"Wind Speed","Signed":false,"BitLength":16,"BitOffset":8,"Type":"","Resolution":0.01,"EnumValues":{"EnumPair":[]},"Units":"m/s","Description":null},{"Order":"3","Id":"windAngle","Name":"Wind Angle","Signed":false,"BitLength":16,"BitOffset":24,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"4","Id":"reference","Name":"Reference","Signed":false,"BitLength":3,"BitOffset":40,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"True (ground referenced to North)"},{"Value":"1","Name":"Magnetic (ground referenced to Magnetic North)"},{"Value":"2","Name":"Apparent"},{"Value":"3","Name":"True (boat referenced)"},{"Value":"4","Name":"True (water referenced)"}]},"Units":null,"Description":null},{"Order":"5","Id":"reserved","Name":"Reserved","Signed":false,"BitLength":21,"BitOffset":43,"Type":"Binary data","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null}]}}
pub struct WindData {
    raw: [u8; 8usize],
}
impl core::convert::TryFrom<&[u8]> for WindData {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 8usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 8usize,
                actual: payload.len(),
                pgn: 130306,
            });
        }
        let mut raw = [0u8; 8usize];
        raw.copy_from_slice(&payload[..8usize]);
        Ok(Self { raw })
    }
}
#[derive(Debug)]
pub enum Reference {
    TrueGroundReferencedToNorth,
    MagneticGroundReferencedToMagneticNorth,
    Apparent,
    TrueBoatReferenced,
    TrueWaterReferenced,
    Other(u8),
}
impl core::convert::From<u8> for Reference {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::TrueGroundReferencedToNorth,
            1 => Self::MagneticGroundReferencedToMagneticNorth,
            2 => Self::Apparent,
            3 => Self::TrueBoatReferenced,
            4 => Self::TrueWaterReferenced,
            v => Self::Other(v),
        }
    }
}
impl WindData {
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
    #[doc = "Unit: m/s"]
    pub fn wind_speed_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[8usize..24usize].load_le::<u16>()
    }
    pub fn wind_speed(&self) -> Option<f32> {
        let raw_value = self.wind_speed_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.01 as f32))
        }
    }
    #[doc = "Unit: rad"]
    pub fn wind_angle_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[24usize..40usize].load_le::<u16>()
    }
    pub fn wind_angle(&self) -> Option<f32> {
        let raw_value = self.wind_angle_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    pub fn reference_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[40usize..43usize].load_le::<u8>()
    }
    pub fn reference(&self) -> Reference {
        self.reference_raw().into()
    }
}
impl core::fmt::Debug for WindData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("WindData")
            .field("sid", &self.sid())
            .field("wind_speed", &self.wind_speed())
            .field("wind_angle", &self.wind_angle())
            .field("reference", &self.reference())
            .finish()
    }
}
