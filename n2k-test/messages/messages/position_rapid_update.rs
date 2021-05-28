use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":129025,"Id":"positionRapidUpdate","Length":8,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"latitude","Name":"Latitude","Signed":true,"BitLength":32,"BitOffset":0,"Type":"Latitude","Resolution":1e-7,"EnumValues":{"EnumPair":[]},"Units":"deg","Description":null},{"Order":"2","Id":"longitude","Name":"Longitude","Signed":true,"BitLength":32,"BitOffset":32,"Type":"Longitude","Resolution":1e-7,"EnumValues":{"EnumPair":[]},"Units":"deg","Description":null}]}}
pub struct PositionRapidUpdate {
    raw: [u8; 8usize],
}
impl core::convert::TryFrom<&[u8]> for PositionRapidUpdate {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 8usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 8usize,
                actual: payload.len(),
                pgn: 129025,
            });
        }
        let mut raw = [0u8; 8usize];
        raw.copy_from_slice(&payload[..8usize]);
        Ok(Self { raw })
    }
}
impl PositionRapidUpdate {
    #[doc = "Unit: deg"]
    pub fn latitude_raw(&self) -> i32 {
        let value = self.raw.view_bits::<Lsb0>()[0usize..32usize].load_le::<u32>();
        i32::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn latitude(&self) -> Option<f32> {
        let raw_value = self.latitude_raw();
        if raw_value == i32::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0000001 as f32))
        }
    }
    #[doc = "Unit: deg"]
    pub fn longitude_raw(&self) -> i32 {
        let value = self.raw.view_bits::<Lsb0>()[32usize..64usize].load_le::<u32>();
        i32::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn longitude(&self) -> Option<f32> {
        let raw_value = self.longitude_raw();
        if raw_value == i32::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0000001 as f32))
        }
    }
}
impl core::fmt::Debug for PositionRapidUpdate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PositionRapidUpdate")
            .field("latitude", &self.latitude())
            .field("longitude", &self.longitude())
            .finish()
    }
}
