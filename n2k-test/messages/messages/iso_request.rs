use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":59904,"Id":"isoRequest","Length":3,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"pgn","Name":"PGN","Signed":false,"BitLength":24,"BitOffset":0,"Type":"Integer","Resolution":1.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null}]}}
pub struct IsoRequest {
    raw: [u8; 3usize],
}
impl core::convert::TryFrom<&[u8]> for IsoRequest {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 3usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 3usize,
                actual: payload.len(),
                pgn: 59904,
            });
        }
        let mut raw = [0u8; 3usize];
        raw.copy_from_slice(&payload[..3usize]);
        Ok(Self { raw })
    }
}
impl IsoRequest {
    pub fn pgn_raw(&self) -> u32 {
        self.raw.view_bits::<Lsb0>()[0usize..24usize].load_le::<u32>()
    }
    pub fn pgn(&self) -> Option<u32> {
        let raw_value = self.pgn_raw();
        if raw_value == u32::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
}
impl core::fmt::Debug for IsoRequest {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IsoRequest")
            .field("pgn", &self.pgn())
            .finish()
    }
}
