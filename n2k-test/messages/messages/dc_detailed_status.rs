use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":127506,"Id":"dcDetailedStatus","Length":9,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"sid","Name":"SID","Signed":false,"BitLength":8,"BitOffset":0,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"2","Id":"instance","Name":"Instance","Signed":false,"BitLength":8,"BitOffset":8,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"3","Id":"dcType","Name":"DC Type","Signed":false,"BitLength":8,"BitOffset":16,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"Battery"},{"Value":"1","Name":"Alternator"},{"Value":"2","Name":"Convertor"},{"Value":"3","Name":"Solar Cell"},{"Value":"4","Name":"Wind Generator"}]},"Units":null,"Description":null},{"Order":"4","Id":"stateOfCharge","Name":"State of Charge","Signed":false,"BitLength":8,"BitOffset":24,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"5","Id":"stateOfHealth","Name":"State of Health","Signed":false,"BitLength":8,"BitOffset":32,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"6","Id":"timeRemaining","Name":"Time Remaining","Signed":false,"BitLength":16,"BitOffset":40,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"7","Id":"rippleVoltage","Name":"Ripple Voltage","Signed":false,"BitLength":16,"BitOffset":56,"Type":"","Resolution":0.01,"EnumValues":{"EnumPair":[]},"Units":"V","Description":null}]}}
pub struct DcDetailedStatus {
    raw: [u8; 9usize],
}
impl core::convert::TryFrom<&[u8]> for DcDetailedStatus {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 9usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 9usize,
                actual: payload.len(),
                pgn: 127506,
            });
        }
        let mut raw = [0u8; 9usize];
        raw.copy_from_slice(&payload[..9usize]);
        Ok(Self { raw })
    }
}
#[derive(Debug)]
pub enum DcType {
    Battery,
    Alternator,
    Convertor,
    SolarCell,
    WindGenerator,
    Other(u8),
}
impl core::convert::From<u8> for DcType {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Battery,
            1 => Self::Alternator,
            2 => Self::Convertor,
            3 => Self::SolarCell,
            4 => Self::WindGenerator,
            v => Self::Other(v),
        }
    }
}
impl DcDetailedStatus {
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
    pub fn dc_type_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[16usize..24usize].load_le::<u8>()
    }
    pub fn dc_type(&self) -> DcType {
        self.dc_type_raw().into()
    }
    pub fn state_of_charge_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[24usize..32usize].load_le::<u8>()
    }
    pub fn state_of_charge(&self) -> Option<u8> {
        let raw_value = self.state_of_charge_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn state_of_health_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[32usize..40usize].load_le::<u8>()
    }
    pub fn state_of_health(&self) -> Option<u8> {
        let raw_value = self.state_of_health_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn time_remaining_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[40usize..56usize].load_le::<u16>()
    }
    pub fn time_remaining(&self) -> Option<u16> {
        let raw_value = self.time_remaining_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    #[doc = "Unit: V"]
    pub fn ripple_voltage_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[56usize..72usize].load_le::<u16>()
    }
    pub fn ripple_voltage(&self) -> Option<f32> {
        let raw_value = self.ripple_voltage_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.01 as f32))
        }
    }
}
impl core::fmt::Debug for DcDetailedStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DcDetailedStatus")
            .field("sid", &self.sid())
            .field("instance", &self.instance())
            .field("dc_type", &self.dc_type())
            .field("state_of_charge", &self.state_of_charge())
            .field("state_of_health", &self.state_of_health())
            .field("time_remaining", &self.time_remaining())
            .field("ripple_voltage", &self.ripple_voltage())
            .finish()
    }
}
