use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":60928,"Id":"isoAddressClaim","Length":8,"Type":"Single","Fields":{"Field":[{"Order":"1","Id":"uniqueNumber","Name":"Unique Number","Signed":false,"BitLength":21,"BitOffset":0,"Type":"Binary data","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":"ISO Identity Number"},{"Order":"2","Id":"manufacturerCode","Name":"Manufacturer Code","Signed":false,"BitLength":11,"BitOffset":21,"Type":"Manufacturer code","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"3","Id":"deviceInstanceLower","Name":"Device Instance Lower","Signed":false,"BitLength":3,"BitOffset":32,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":"ISO ECU Instance"},{"Order":"4","Id":"deviceInstanceUpper","Name":"Device Instance Upper","Signed":false,"BitLength":5,"BitOffset":35,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":"ISO Function Instance"},{"Order":"5","Id":"deviceFunction","Name":"Device Function","Signed":false,"BitLength":8,"BitOffset":40,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":"ISO Function"},{"Order":"6","Id":"reserved","Name":"Reserved","Signed":false,"BitLength":1,"BitOffset":48,"Type":"Binary data","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"7","Id":"deviceClass","Name":"Device Class","Signed":false,"BitLength":7,"BitOffset":49,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"Reserved for 2000 Use"},{"Value":"10","Name":"System tools"},{"Value":"20","Name":"Safety systems"},{"Value":"25","Name":"Internetwork device"},{"Value":"30","Name":"Electrical Distribution"},{"Value":"35","Name":"Electrical Generation"},{"Value":"40","Name":"Steering and Control surfaces"},{"Value":"50","Name":"Propulsion"},{"Value":"60","Name":"Navigation"},{"Value":"70","Name":"Communication"},{"Value":"75","Name":"Sensor Communication Interface"},{"Value":"80","Name":"Instrumentation/general systems"},{"Value":"85","Name":"External Environment"},{"Value":"90","Name":"Internal Environment"},{"Value":"100","Name":"Deck + cargo + fishing equipment systems"},{"Value":"120","Name":"Display"},{"Value":"125","Name":"Entertainment"}]},"Units":null,"Description":null},{"Order":"8","Id":"systemInstance","Name":"System Instance","Signed":false,"BitLength":4,"BitOffset":56,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":"ISO Device Class Instance"},{"Order":"9","Id":"industryGroup","Name":"Industry Group","Signed":false,"BitLength":3,"BitOffset":60,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"Global"},{"Value":"1","Name":"Highway"},{"Value":"2","Name":"Agriculture"},{"Value":"3","Name":"Construction"},{"Value":"4","Name":"Marine"},{"Value":"5","Name":"Industrial"}]},"Units":null,"Description":null},{"Order":"10","Id":"reserved","Name":"Reserved","Signed":false,"BitLength":1,"BitOffset":63,"Type":"Binary data","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":"ISO Self Configurable"}]}}
pub struct IsoAddressClaim {
    raw: [u8; 8usize],
}
impl core::convert::TryFrom<&[u8]> for IsoAddressClaim {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 8usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 8usize,
                actual: payload.len(),
                pgn: 60928,
            });
        }
        let mut raw = [0u8; 8usize];
        raw.copy_from_slice(&payload[..8usize]);
        Ok(Self { raw })
    }
}
#[derive(Debug)]
pub enum DeviceClass {
    ReservedFor2000Use,
    SystemTools,
    SafetySystems,
    InternetworkDevice,
    ElectricalDistribution,
    ElectricalGeneration,
    SteeringAndControlSurfaces,
    Propulsion,
    Navigation,
    Communication,
    SensorCommunicationInterface,
    InstrumentationGeneralSystems,
    ExternalEnvironment,
    InternalEnvironment,
    DeckCargoFishingEquipmentSystems,
    Display,
    Entertainment,
    Other(u8),
}
impl core::convert::From<u8> for DeviceClass {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::ReservedFor2000Use,
            10 => Self::SystemTools,
            20 => Self::SafetySystems,
            25 => Self::InternetworkDevice,
            30 => Self::ElectricalDistribution,
            35 => Self::ElectricalGeneration,
            40 => Self::SteeringAndControlSurfaces,
            50 => Self::Propulsion,
            60 => Self::Navigation,
            70 => Self::Communication,
            75 => Self::SensorCommunicationInterface,
            80 => Self::InstrumentationGeneralSystems,
            85 => Self::ExternalEnvironment,
            90 => Self::InternalEnvironment,
            100 => Self::DeckCargoFishingEquipmentSystems,
            120 => Self::Display,
            125 => Self::Entertainment,
            v => Self::Other(v),
        }
    }
}
#[derive(Debug)]
pub enum IndustryGroup {
    Global,
    Highway,
    Agriculture,
    Construction,
    Marine,
    Industrial,
    Other(u8),
}
impl core::convert::From<u8> for IndustryGroup {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Global,
            1 => Self::Highway,
            2 => Self::Agriculture,
            3 => Self::Construction,
            4 => Self::Marine,
            5 => Self::Industrial,
            v => Self::Other(v),
        }
    }
}
impl IsoAddressClaim {
    #[doc = "Description: ISO Identity Number"]
    pub fn unique_number_raw(&self) -> u32 {
        self.raw.view_bits::<Lsb0>()[0usize..21usize].load_le::<u32>()
    }
    pub fn unique_number(&self) -> Option<u32> {
        let raw_value = self.unique_number_raw();
        if raw_value == u32::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn manufacturer_code_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[21usize..32usize].load_le::<u16>()
    }
    pub fn manufacturer_code(&self) -> Option<u16> {
        let raw_value = self.manufacturer_code_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    #[doc = "Description: ISO ECU Instance"]
    pub fn device_instance_lower_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[32usize..35usize].load_le::<u8>()
    }
    pub fn device_instance_lower(&self) -> Option<u8> {
        let raw_value = self.device_instance_lower_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    #[doc = "Description: ISO Function Instance"]
    pub fn device_instance_upper_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[35usize..40usize].load_le::<u8>()
    }
    pub fn device_instance_upper(&self) -> Option<u8> {
        let raw_value = self.device_instance_upper_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    #[doc = "Description: ISO Function"]
    pub fn device_function_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[40usize..48usize].load_le::<u8>()
    }
    pub fn device_function(&self) -> Option<u8> {
        let raw_value = self.device_function_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn device_class_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[49usize..56usize].load_le::<u8>()
    }
    pub fn device_class(&self) -> DeviceClass {
        self.device_class_raw().into()
    }
    #[doc = "Description: ISO Device Class Instance"]
    pub fn system_instance_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[56usize..60usize].load_le::<u8>()
    }
    pub fn system_instance(&self) -> Option<u8> {
        let raw_value = self.system_instance_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn industry_group_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[60usize..63usize].load_le::<u8>()
    }
    pub fn industry_group(&self) -> IndustryGroup {
        self.industry_group_raw().into()
    }
}
impl core::fmt::Debug for IsoAddressClaim {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("IsoAddressClaim")
            .field("unique_number", &self.unique_number())
            .field("manufacturer_code", &self.manufacturer_code())
            .field("device_instance_lower", &self.device_instance_lower())
            .field("device_instance_upper", &self.device_instance_upper())
            .field("device_function", &self.device_function())
            .field("device_class", &self.device_class())
            .field("system_instance", &self.system_instance())
            .field("industry_group", &self.industry_group())
            .finish()
    }
}
