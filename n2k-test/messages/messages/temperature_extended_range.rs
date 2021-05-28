use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":130316,"Id":"temperatureExtendedRange","Length":8,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"sid","Name":"SID","Signed":false,"BitLength":8,"BitOffset":0,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"2","Id":"instance","Name":"Instance","Signed":false,"BitLength":8,"BitOffset":8,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"3","Id":"source","Name":"Source","Signed":false,"BitLength":8,"BitOffset":16,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"Sea Temperature"},{"Value":"1","Name":"Outside Temperature"},{"Value":"2","Name":"Inside Temperature"},{"Value":"3","Name":"Engine Room Temperature"},{"Value":"4","Name":"Main Cabin Temperature"},{"Value":"5","Name":"Live Well Temperature"},{"Value":"6","Name":"Bait Well Temperature"},{"Value":"7","Name":"Refridgeration Temperature"},{"Value":"8","Name":"Heating System Temperature"},{"Value":"9","Name":"Dew Point Temperature"},{"Value":"10","Name":"Apparent Wind Chill Temperature"},{"Value":"11","Name":"Theoretical Wind Chill Temperature"},{"Value":"12","Name":"Heat Index Temperature"},{"Value":"13","Name":"Freezer Temperature"},{"Value":"14","Name":"Exhaust Gas Temperature"}]},"Units":null,"Description":null},{"Order":"4","Id":"temperature","Name":"Temperature","Signed":false,"BitLength":24,"BitOffset":24,"Type":"Temperature (hires)","Resolution":0.001,"EnumValues":{"EnumPair":[]},"Units":"K","Description":null},{"Order":"5","Id":"setTemperature","Name":"Set Temperature","Signed":false,"BitLength":16,"BitOffset":48,"Type":"Temperature","Resolution":0.1,"EnumValues":{"EnumPair":[]},"Units":"K","Description":null}]}}
pub struct TemperatureExtendedRange {
    raw: [u8; 8usize],
}
impl core::convert::TryFrom<&[u8]> for TemperatureExtendedRange {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 8usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 8usize,
                actual: payload.len(),
                pgn: 130316,
            });
        }
        let mut raw = [0u8; 8usize];
        raw.copy_from_slice(&payload[..8usize]);
        Ok(Self { raw })
    }
}
#[derive(Debug)]
pub enum Source {
    SeaTemperature,
    OutsideTemperature,
    InsideTemperature,
    EngineRoomTemperature,
    MainCabinTemperature,
    LiveWellTemperature,
    BaitWellTemperature,
    RefridgerationTemperature,
    HeatingSystemTemperature,
    DewPointTemperature,
    ApparentWindChillTemperature,
    TheoreticalWindChillTemperature,
    HeatIndexTemperature,
    FreezerTemperature,
    ExhaustGasTemperature,
    Other(u8),
}
impl core::convert::From<u8> for Source {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::SeaTemperature,
            1 => Self::OutsideTemperature,
            2 => Self::InsideTemperature,
            3 => Self::EngineRoomTemperature,
            4 => Self::MainCabinTemperature,
            5 => Self::LiveWellTemperature,
            6 => Self::BaitWellTemperature,
            7 => Self::RefridgerationTemperature,
            8 => Self::HeatingSystemTemperature,
            9 => Self::DewPointTemperature,
            10 => Self::ApparentWindChillTemperature,
            11 => Self::TheoreticalWindChillTemperature,
            12 => Self::HeatIndexTemperature,
            13 => Self::FreezerTemperature,
            14 => Self::ExhaustGasTemperature,
            v => Self::Other(v),
        }
    }
}
impl TemperatureExtendedRange {
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
    #[doc = "Unit: K"]
    pub fn temperature_raw(&self) -> u32 {
        self.raw.view_bits::<Lsb0>()[24usize..48usize].load_le::<u32>()
    }
    pub fn temperature(&self) -> Option<f32> {
        let raw_value = self.temperature_raw();
        if raw_value == u32::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.001 as f32))
        }
    }
    #[doc = "Unit: K"]
    pub fn set_temperature_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[48usize..64usize].load_le::<u16>()
    }
    pub fn set_temperature(&self) -> Option<f32> {
        let raw_value = self.set_temperature_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.1 as f32))
        }
    }
}
impl core::fmt::Debug for TemperatureExtendedRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("TemperatureExtendedRange")
            .field("sid", &self.sid())
            .field("instance", &self.instance())
            .field("source", &self.source())
            .field("temperature", &self.temperature())
            .field("set_temperature", &self.set_temperature())
            .finish()
    }
}
