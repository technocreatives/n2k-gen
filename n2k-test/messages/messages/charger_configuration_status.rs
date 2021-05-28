use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":127510,"Id":"chargerConfigurationStatus","Length":13,"Type":"Fast","Fields":{"Field":[{"Order":"1","Id":"instance","Name":"Instance","Signed":false,"BitLength":8,"BitOffset":0,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"2","Id":"batteryInstance","Name":"Battery Instance","Signed":false,"BitLength":8,"BitOffset":8,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"3","Id":"chargerEnableDisable","Name":"Charger Enable/Disable","Signed":false,"BitLength":2,"BitOffset":16,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"4","Id":"reserved","Name":"Reserved","Signed":false,"BitLength":6,"BitOffset":18,"Type":"Binary data","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"5","Id":"chargeCurrentLimit","Name":"Charge Current Limit","Signed":false,"BitLength":16,"BitOffset":24,"Type":"","Resolution":0.1,"EnumValues":{"EnumPair":[]},"Units":"A","Description":null},{"Order":"6","Id":"chargingAlgorithm","Name":"Charging Algorithm","Signed":false,"BitLength":8,"BitOffset":40,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"7","Id":"chargerMode","Name":"Charger Mode","Signed":false,"BitLength":8,"BitOffset":48,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"8","Id":"estimatedTemperature","Name":"Estimated Temperature","Signed":false,"BitLength":16,"BitOffset":56,"Type":"Temperature","Resolution":0.01,"EnumValues":{"EnumPair":[]},"Units":null,"Description":"When no sensor present"},{"Order":"9","Id":"equalizeOneTimeEnableDisable","Name":"Equalize One Time Enable/Disable","Signed":false,"BitLength":4,"BitOffset":72,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"10","Id":"overChargeEnableDisable","Name":"Over Charge Enable/Disable","Signed":false,"BitLength":4,"BitOffset":76,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"11","Id":"equalizeTime","Name":"Equalize Time","Signed":false,"BitLength":16,"BitOffset":80,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null}]}}
pub struct ChargerConfigurationStatus {
    raw: [u8; 13usize],
}
impl core::convert::TryFrom<&[u8]> for ChargerConfigurationStatus {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 13usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 13usize,
                actual: payload.len(),
                pgn: 127510,
            });
        }
        let mut raw = [0u8; 13usize];
        raw.copy_from_slice(&payload[..13usize]);
        Ok(Self { raw })
    }
}
impl ChargerConfigurationStatus {
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
    pub fn battery_instance_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[8usize..16usize].load_le::<u8>()
    }
    pub fn battery_instance(&self) -> Option<u8> {
        let raw_value = self.battery_instance_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn charger_enable_disable_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[16usize..18usize].load_le::<u8>()
    }
    pub fn charger_enable_disable(&self) -> Option<u8> {
        let raw_value = self.charger_enable_disable_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    #[doc = "Unit: A"]
    pub fn charge_current_limit_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[24usize..40usize].load_le::<u16>()
    }
    pub fn charge_current_limit(&self) -> Option<f32> {
        let raw_value = self.charge_current_limit_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.1 as f32))
        }
    }
    pub fn charging_algorithm_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[40usize..48usize].load_le::<u8>()
    }
    pub fn charging_algorithm(&self) -> Option<u8> {
        let raw_value = self.charging_algorithm_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn charger_mode_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[48usize..56usize].load_le::<u8>()
    }
    pub fn charger_mode(&self) -> Option<u8> {
        let raw_value = self.charger_mode_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    #[doc = "Description: When no sensor present"]
    pub fn estimated_temperature_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[56usize..72usize].load_le::<u16>()
    }
    pub fn estimated_temperature(&self) -> Option<f32> {
        let raw_value = self.estimated_temperature_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.01 as f32))
        }
    }
    pub fn equalize_one_time_enable_disable_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[72usize..76usize].load_le::<u8>()
    }
    pub fn equalize_one_time_enable_disable(&self) -> Option<u8> {
        let raw_value = self.equalize_one_time_enable_disable_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn over_charge_enable_disable_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[76usize..80usize].load_le::<u8>()
    }
    pub fn over_charge_enable_disable(&self) -> Option<u8> {
        let raw_value = self.over_charge_enable_disable_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn equalize_time_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[80usize..96usize].load_le::<u16>()
    }
    pub fn equalize_time(&self) -> Option<u16> {
        let raw_value = self.equalize_time_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
}
impl core::fmt::Debug for ChargerConfigurationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ChargerConfigurationStatus")
            .field("instance", &self.instance())
            .field("battery_instance", &self.battery_instance())
            .field("charger_enable_disable", &self.charger_enable_disable())
            .field("charge_current_limit", &self.charge_current_limit())
            .field("charging_algorithm", &self.charging_algorithm())
            .field("charger_mode", &self.charger_mode())
            .field("estimated_temperature", &self.estimated_temperature())
            .field(
                "equalize_one_time_enable_disable",
                &self.equalize_one_time_enable_disable(),
            )
            .field(
                "over_charge_enable_disable",
                &self.over_charge_enable_disable(),
            )
            .field("equalize_time", &self.equalize_time())
            .finish()
    }
}
