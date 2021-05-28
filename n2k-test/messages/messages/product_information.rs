use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":126996,"Id":"productInformation","Length":134,"Type":"Fast","Fields":{"Field":[{"Order":"1","Id":"nmea2000Version","Name":"NMEA 2000 Version","Signed":false,"BitLength":16,"BitOffset":0,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"2","Id":"productCode","Name":"Product Code","Signed":false,"BitLength":16,"BitOffset":16,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"3","Id":"modelId","Name":"Model ID","Signed":false,"BitLength":256,"BitOffset":32,"Type":"ASCII text","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"4","Id":"softwareVersionCode","Name":"Software Version Code","Signed":false,"BitLength":256,"BitOffset":288,"Type":"ASCII text","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"5","Id":"modelVersion","Name":"Model Version","Signed":false,"BitLength":256,"BitOffset":544,"Type":"ASCII text","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"6","Id":"modelSerialCode","Name":"Model Serial Code","Signed":false,"BitLength":256,"BitOffset":800,"Type":"ASCII text","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"7","Id":"certificationLevel","Name":"Certification Level","Signed":false,"BitLength":8,"BitOffset":1056,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"8","Id":"loadEquivalency","Name":"Load Equivalency","Signed":false,"BitLength":8,"BitOffset":1064,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null}]}}
pub struct ProductInformation {
    raw: [u8; 134usize],
}
impl core::convert::TryFrom<&[u8]> for ProductInformation {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 134usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 134usize,
                actual: payload.len(),
                pgn: 126996,
            });
        }
        let mut raw = [0u8; 134usize];
        raw.copy_from_slice(&payload[..134usize]);
        Ok(Self { raw })
    }
}
impl ProductInformation {
    pub fn nmea2000_version_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[0usize..16usize].load_le::<u16>()
    }
    pub fn nmea2000_version(&self) -> Option<u16> {
        let raw_value = self.nmea2000_version_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn product_code_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[16usize..32usize].load_le::<u16>()
    }
    pub fn product_code(&self) -> Option<u16> {
        let raw_value = self.product_code_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn model_id_raw<'a>(&'a self) -> &'a [u8] {
        self.raw.view_bits::<Lsb0>()[32usize..288usize].as_raw_slice()
    }
    pub fn model_id<'a>(&'a self) -> Result<&'a str, core::str::Utf8Error> {
        core::str::from_utf8(self.model_id_raw())
    }
    pub fn software_version_code_raw<'a>(&'a self) -> &'a [u8] {
        self.raw.view_bits::<Lsb0>()[288usize..544usize].as_raw_slice()
    }
    pub fn software_version_code<'a>(&'a self) -> Result<&'a str, core::str::Utf8Error> {
        core::str::from_utf8(self.software_version_code_raw())
    }
    pub fn model_version_raw<'a>(&'a self) -> &'a [u8] {
        self.raw.view_bits::<Lsb0>()[544usize..800usize].as_raw_slice()
    }
    pub fn model_version<'a>(&'a self) -> Result<&'a str, core::str::Utf8Error> {
        core::str::from_utf8(self.model_version_raw())
    }
    pub fn model_serial_code_raw<'a>(&'a self) -> &'a [u8] {
        self.raw.view_bits::<Lsb0>()[800usize..1056usize].as_raw_slice()
    }
    pub fn model_serial_code<'a>(&'a self) -> Result<&'a str, core::str::Utf8Error> {
        core::str::from_utf8(self.model_serial_code_raw())
    }
    pub fn certification_level_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[1056usize..1064usize].load_le::<u8>()
    }
    pub fn certification_level(&self) -> Option<u8> {
        let raw_value = self.certification_level_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    pub fn load_equivalency_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[1064usize..1072usize].load_le::<u8>()
    }
    pub fn load_equivalency(&self) -> Option<u8> {
        let raw_value = self.load_equivalency_raw();
        if raw_value == u8::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
}
impl core::fmt::Debug for ProductInformation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ProductInformation")
            .field("nmea2000_version", &self.nmea2000_version())
            .field("product_code", &self.product_code())
            .field("model_id", &self.model_id())
            .field("software_version_code", &self.software_version_code())
            .field("model_version", &self.model_version())
            .field("model_serial_code", &self.model_serial_code())
            .field("certification_level", &self.certification_level())
            .field("load_equivalency", &self.load_equivalency())
            .finish()
    }
}
