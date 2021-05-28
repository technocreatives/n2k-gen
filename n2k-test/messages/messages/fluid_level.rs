use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":127505,"Id":"fluidLevel","Length":8,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"instance","Name":"Instance","Signed":false,"BitLength":4,"BitOffset":0,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"2","Id":"type","Name":"Type","Signed":false,"BitLength":4,"BitOffset":4,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"Fuel"},{"Value":"1","Name":"Water"},{"Value":"2","Name":"Gray water"},{"Value":"3","Name":"Live well"},{"Value":"4","Name":"Oil"},{"Value":"5","Name":"Black water"}]},"Units":null,"Description":null},{"Order":"3","Id":"level","Name":"Level","Signed":false,"BitLength":16,"BitOffset":8,"Type":"","Resolution":0.004,"EnumValues":{"EnumPair":[]},"Units":"%","Description":null},{"Order":"4","Id":"capacity","Name":"Capacity","Signed":false,"BitLength":32,"BitOffset":24,"Type":"","Resolution":0.1,"EnumValues":{"EnumPair":[]},"Units":"L","Description":null},{"Order":"5","Id":"reserved","Name":"Reserved","Signed":false,"BitLength":8,"BitOffset":56,"Type":"Binary data","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":"Reserved"}]}}
pub struct FluidLevel {
    raw: [u8; 8usize],
}
impl core::convert::TryFrom<&[u8]> for FluidLevel {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 8usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 8usize,
                actual: payload.len(),
                pgn: 127505,
            });
        }
        let mut raw = [0u8; 8usize];
        raw.copy_from_slice(&payload[..8usize]);
        Ok(Self { raw })
    }
}
#[derive(Debug)]
pub enum Type {
    Fuel,
    Water,
    GrayWater,
    LiveWell,
    Oil,
    BlackWater,
    Other(u8),
}
impl core::convert::From<u8> for Type {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Fuel,
            1 => Self::Water,
            2 => Self::GrayWater,
            3 => Self::LiveWell,
            4 => Self::Oil,
            5 => Self::BlackWater,
            v => Self::Other(v),
        }
    }
}
impl FluidLevel {
    pub fn instance_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[0usize..4usize].load_le::<u8>()
    }
    pub fn instance(&self) -> Option<u8> {
        let raw_value = self.instance_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn xtype_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[4usize..8usize].load_le::<u8>()
    }
    pub fn xtype(&self) -> Type {
        self.xtype_raw().into()
    }
    #[doc = "Unit: %"]
    pub fn level_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[8usize..24usize].load_le::<u16>()
    }
    pub fn level(&self) -> Option<f32> {
        let raw_value = self.level_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.004 as f32))
        }
    }
    #[doc = "Unit: L"]
    pub fn capacity_raw(&self) -> u32 {
        self.raw.view_bits::<Lsb0>()[24usize..56usize].load_le::<u32>()
    }
    pub fn capacity(&self) -> Option<f32> {
        let raw_value = self.capacity_raw();
        if raw_value == u32::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.1 as f32))
        }
    }
}
impl core::fmt::Debug for FluidLevel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("FluidLevel")
            .field("instance", &self.instance())
            .field("xtype", &self.xtype())
            .field("level", &self.level())
            .field("capacity", &self.capacity())
            .finish()
    }
}
