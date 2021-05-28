use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":127257,"Id":"attitude","Length":7,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"sid","Name":"SID","Signed":false,"BitLength":8,"BitOffset":0,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"2","Id":"yaw","Name":"Yaw","Signed":true,"BitLength":16,"BitOffset":8,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"3","Id":"pitch","Name":"Pitch","Signed":true,"BitLength":16,"BitOffset":24,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"4","Id":"roll","Name":"Roll","Signed":true,"BitLength":16,"BitOffset":40,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null}]}}
pub struct Attitude {
    raw: [u8; 7usize],
}
impl core::convert::TryFrom<&[u8]> for Attitude {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 7usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 7usize,
                actual: payload.len(),
                pgn: 127257,
            });
        }
        let mut raw = [0u8; 7usize];
        raw.copy_from_slice(&payload[..7usize]);
        Ok(Self { raw })
    }
}
impl Attitude {
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
    pub fn yaw_raw(&self) -> i16 {
        let value = self.raw.view_bits::<Lsb0>()[8usize..24usize].load_le::<u16>();
        i16::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn yaw(&self) -> Option<f32> {
        let raw_value = self.yaw_raw();
        if raw_value == i16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    #[doc = "Unit: rad"]
    pub fn pitch_raw(&self) -> i16 {
        let value = self.raw.view_bits::<Lsb0>()[24usize..40usize].load_le::<u16>();
        i16::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn pitch(&self) -> Option<f32> {
        let raw_value = self.pitch_raw();
        if raw_value == i16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    #[doc = "Unit: rad"]
    pub fn roll_raw(&self) -> i16 {
        let value = self.raw.view_bits::<Lsb0>()[40usize..56usize].load_le::<u16>();
        i16::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn roll(&self) -> Option<f32> {
        let raw_value = self.roll_raw();
        if raw_value == i16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
}
impl core::fmt::Debug for Attitude {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Attitude")
            .field("sid", &self.sid())
            .field("yaw", &self.yaw())
            .field("pitch", &self.pitch())
            .field("roll", &self.roll())
            .finish()
    }
}
