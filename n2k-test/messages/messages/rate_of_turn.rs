use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":127251,"Id":"rateOfTurn","Length":5,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"sid","Name":"SID","Signed":false,"BitLength":8,"BitOffset":0,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"2","Id":"rate","Name":"Rate","Signed":true,"BitLength":32,"BitOffset":8,"Type":"","Resolution":3.125e-8,"EnumValues":{"EnumPair":[]},"Units":"rad/s","Description":null}]}}
pub struct RateOfTurn {
    raw: [u8; 5usize],
}
impl core::convert::TryFrom<&[u8]> for RateOfTurn {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 5usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 5usize,
                actual: payload.len(),
                pgn: 127251,
            });
        }
        let mut raw = [0u8; 5usize];
        raw.copy_from_slice(&payload[..5usize]);
        Ok(Self { raw })
    }
}
impl RateOfTurn {
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
    #[doc = "Unit: rad/s"]
    pub fn rate_raw(&self) -> i32 {
        let value = self.raw.view_bits::<Lsb0>()[8usize..40usize].load_le::<u32>();
        i32::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn rate(&self) -> Option<f32> {
        let raw_value = self.rate_raw();
        if raw_value == i32::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.00000003125 as f32))
        }
    }
}
impl core::fmt::Debug for RateOfTurn {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("RateOfTurn")
            .field("sid", &self.sid())
            .field("rate", &self.rate())
            .finish()
    }
}
