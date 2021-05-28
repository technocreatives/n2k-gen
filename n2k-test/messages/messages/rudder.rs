use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":127245,"Id":"rudder","Length":8,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"instance","Name":"Instance","Signed":false,"BitLength":8,"BitOffset":0,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"2","Id":"directionOrder","Name":"Direction Order","Signed":false,"BitLength":2,"BitOffset":8,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"3","Id":"reserved","Name":"Reserved","Signed":false,"BitLength":6,"BitOffset":10,"Type":"Binary data","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":"Reserved"},{"Order":"4","Id":"angleOrder","Name":"Angle Order","Signed":true,"BitLength":16,"BitOffset":16,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"5","Id":"position","Name":"Position","Signed":true,"BitLength":16,"BitOffset":32,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null}]}}
pub struct Rudder {
    raw: [u8; 8usize],
}
impl core::convert::TryFrom<&[u8]> for Rudder {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 8usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 8usize,
                actual: payload.len(),
                pgn: 127245,
            });
        }
        let mut raw = [0u8; 8usize];
        raw.copy_from_slice(&payload[..8usize]);
        Ok(Self { raw })
    }
}
impl Rudder {
    pub fn instance_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[0usize..8usize].load_le::<u8>()
    }
    pub fn instance(&self) -> Option<u8> {
        let raw_value = self.instance_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn direction_order_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[8usize..10usize].load_le::<u8>()
    }
    pub fn direction_order(&self) -> Option<u8> {
        let raw_value = self.direction_order_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    #[doc = "Unit: rad"]
    pub fn angle_order_raw(&self) -> i16 {
        let value = self.raw.view_bits::<Lsb0>()[16usize..32usize].load_le::<u16>();
        i16::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn angle_order(&self) -> Option<f32> {
        let raw_value = self.angle_order_raw();
        if raw_value == i16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    #[doc = "Unit: rad"]
    pub fn position_raw(&self) -> i16 {
        let value = self.raw.view_bits::<Lsb0>()[32usize..48usize].load_le::<u16>();
        i16::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn position(&self) -> Option<f32> {
        let raw_value = self.position_raw();
        if raw_value == i16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
}
impl core::fmt::Debug for Rudder {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Rudder")
            .field("instance", &self.instance())
            .field("direction_order", &self.direction_order())
            .field("angle_order", &self.angle_order())
            .field("position", &self.position())
            .finish()
    }
}
