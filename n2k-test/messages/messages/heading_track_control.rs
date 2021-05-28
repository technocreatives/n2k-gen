use super::super::types::*;
use bitvec::prelude::*;
// {"PGN":127237,"Id":"headingTrackControl","Length":21,"Type":"Fast","Fields":{"Field":[{"Order":"1","Id":"rudderLimitExceeded","Name":"Rudder Limit Exceeded","Signed":false,"BitLength":2,"BitOffset":0,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"No"},{"Value":"1","Name":"Yes"},{"Value":"10","Name":"Error"},{"Value":"11","Name":"Unavailable"}]},"Units":null,"Description":null},{"Order":"2","Id":"offHeadingLimitExceeded","Name":"Off-Heading Limit Exceeded","Signed":false,"BitLength":2,"BitOffset":2,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"No"},{"Value":"1","Name":"Yes"},{"Value":"10","Name":"Error"},{"Value":"11","Name":"Unavailable"}]},"Units":null,"Description":null},{"Order":"3","Id":"offTrackLimitExceeded","Name":"Off-Track Limit Exceeded","Signed":false,"BitLength":2,"BitOffset":4,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"No"},{"Value":"1","Name":"Yes"},{"Value":"10","Name":"Error"},{"Value":"11","Name":"Unavailable"}]},"Units":null,"Description":null},{"Order":"4","Id":"override","Name":"Override","Signed":false,"BitLength":2,"BitOffset":6,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"No"},{"Value":"1","Name":"Yes"},{"Value":"10","Name":"Error"},{"Value":"11","Name":"Unavailable"}]},"Units":null,"Description":null},{"Order":"5","Id":"steeringMode","Name":"Steering Mode","Signed":false,"BitLength":3,"BitOffset":8,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"Main Steering"},{"Value":"1","Name":"Non-Follow-up Device"},{"Value":"10","Name":"Follow-up Device"},{"Value":"11","Name":"Heading Control Standalone"},{"Value":"100","Name":"Heading Control"},{"Value":"101","Name":"Track Control"}]},"Units":null,"Description":null},{"Order":"6","Id":"turnMode","Name":"Turn Mode","Signed":false,"BitLength":3,"BitOffset":11,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"Rudder Limit controlled"},{"Value":"1","Name":"turn rate controlled"},{"Value":"10","Name":"radius controlled"}]},"Units":null,"Description":null},{"Order":"7","Id":"headingReference","Name":"Heading Reference","Signed":false,"BitLength":2,"BitOffset":14,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"True"},{"Value":"1","Name":"Magnetic"},{"Value":"2","Name":"Error"},{"Value":"3","Name":"Null"}]},"Units":null,"Description":null},{"Order":"8","Id":"reserved","Name":"Reserved","Signed":false,"BitLength":5,"BitOffset":16,"Type":"Binary data","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":null,"Description":null},{"Order":"9","Id":"commandedRudderDirection","Name":"Commanded Rudder Direction","Signed":false,"BitLength":3,"BitOffset":21,"Type":"Lookup table","Resolution":0.0,"EnumValues":{"EnumPair":[{"Value":"0","Name":"No Order"},{"Value":"1","Name":"Move to starboard"},{"Value":"10","Name":"Move to port"}]},"Units":null,"Description":null},{"Order":"10","Id":"commandedRudderAngle","Name":"Commanded Rudder Angle","Signed":true,"BitLength":16,"BitOffset":24,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"11","Id":"headingToSteerCourse","Name":"Heading-To-Steer (Course)","Signed":false,"BitLength":16,"BitOffset":40,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"12","Id":"track","Name":"Track","Signed":false,"BitLength":16,"BitOffset":56,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"13","Id":"rudderLimit","Name":"Rudder Limit","Signed":false,"BitLength":16,"BitOffset":72,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"14","Id":"offHeadingLimit","Name":"Off-Heading Limit","Signed":false,"BitLength":16,"BitOffset":88,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"15","Id":"radiusOfTurnOrder","Name":"Radius of Turn Order","Signed":true,"BitLength":16,"BitOffset":104,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null},{"Order":"16","Id":"rateOfTurnOrder","Name":"Rate of Turn Order","Signed":true,"BitLength":16,"BitOffset":120,"Type":"","Resolution":0.00003125,"EnumValues":{"EnumPair":[]},"Units":"rad/s","Description":null},{"Order":"17","Id":"offTrackLimit","Name":"Off-Track Limit","Signed":true,"BitLength":16,"BitOffset":136,"Type":"","Resolution":0.0,"EnumValues":{"EnumPair":[]},"Units":"m","Description":null},{"Order":"18","Id":"vesselHeading","Name":"Vessel Heading","Signed":false,"BitLength":16,"BitOffset":152,"Type":"","Resolution":0.0001,"EnumValues":{"EnumPair":[]},"Units":"rad","Description":null}]}}
pub struct HeadingTrackControl {
    raw: [u8; 21usize],
}
impl core::convert::TryFrom<&[u8]> for HeadingTrackControl {
    type Error = N2kError;
    #[inline(always)]
    fn try_from(payload: &[u8]) -> Result<Self, Self::Error> {
        if payload.len() < 21usize {
            return Err(N2kError::InvalidPayloadSize {
                expected: 21usize,
                actual: payload.len(),
                pgn: 127237,
            });
        }
        let mut raw = [0u8; 21usize];
        raw.copy_from_slice(&payload[..21usize]);
        Ok(Self { raw })
    }
}
#[derive(Debug)]
pub enum RudderLimitExceeded {
    No,
    Yes,
    Error,
    Unavailable,
    Other(u8),
}
impl core::convert::From<u8> for RudderLimitExceeded {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::No,
            1 => Self::Yes,
            2 => Self::Error,
            3 => Self::Unavailable,
            v => Self::Other(v),
        }
    }
}
#[derive(Debug)]
pub enum OffHeadingLimitExceeded {
    No,
    Yes,
    Error,
    Unavailable,
    Other(u8),
}
impl core::convert::From<u8> for OffHeadingLimitExceeded {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::No,
            1 => Self::Yes,
            2 => Self::Error,
            3 => Self::Unavailable,
            v => Self::Other(v),
        }
    }
}
#[derive(Debug)]
pub enum OffTrackLimitExceeded {
    No,
    Yes,
    Error,
    Unavailable,
    Other(u8),
}
impl core::convert::From<u8> for OffTrackLimitExceeded {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::No,
            1 => Self::Yes,
            2 => Self::Error,
            3 => Self::Unavailable,
            v => Self::Other(v),
        }
    }
}
#[derive(Debug)]
pub enum Override {
    No,
    Yes,
    Error,
    Unavailable,
    Other(u8),
}
impl core::convert::From<u8> for Override {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::No,
            1 => Self::Yes,
            2 => Self::Error,
            3 => Self::Unavailable,
            v => Self::Other(v),
        }
    }
}
#[derive(Debug)]
pub enum SteeringMode {
    MainSteering,
    NonFollowUpDevice,
    FollowUpDevice,
    HeadingControlStandalone,
    HeadingControl,
    TrackControl,
    Other(u8),
}
impl core::convert::From<u8> for SteeringMode {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::MainSteering,
            1 => Self::NonFollowUpDevice,
            2 => Self::FollowUpDevice,
            3 => Self::HeadingControlStandalone,
            4 => Self::HeadingControl,
            5 => Self::TrackControl,
            v => Self::Other(v),
        }
    }
}
#[derive(Debug)]
pub enum TurnMode {
    RudderLimitControlled,
    TurnRateControlled,
    RadiusControlled,
    Other(u8),
}
impl core::convert::From<u8> for TurnMode {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::RudderLimitControlled,
            1 => Self::TurnRateControlled,
            2 => Self::RadiusControlled,
            v => Self::Other(v),
        }
    }
}
#[derive(Debug)]
pub enum HeadingReference {
    XTrue,
    Magnetic,
    Error,
    Null,
    Other(u8),
}
impl core::convert::From<u8> for HeadingReference {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::XTrue,
            1 => Self::Magnetic,
            2 => Self::Error,
            3 => Self::Null,
            v => Self::Other(v),
        }
    }
}
#[derive(Debug)]
pub enum CommandedRudderDirection {
    NoOrder,
    MoveToStarboard,
    MoveToPort,
    Other(u8),
}
impl core::convert::From<u8> for CommandedRudderDirection {
    #[inline(always)]
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NoOrder,
            1 => Self::MoveToStarboard,
            2 => Self::MoveToPort,
            v => Self::Other(v),
        }
    }
}
impl HeadingTrackControl {
    pub fn rudder_limit_exceeded_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[0usize..2usize].load_le::<u8>()
    }
    pub fn rudder_limit_exceeded(&self) -> RudderLimitExceeded {
        self.rudder_limit_exceeded_raw().into()
    }
    pub fn off_heading_limit_exceeded_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[2usize..4usize].load_le::<u8>()
    }
    pub fn off_heading_limit_exceeded(&self) -> OffHeadingLimitExceeded {
        self.off_heading_limit_exceeded_raw().into()
    }
    pub fn off_track_limit_exceeded_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[4usize..6usize].load_le::<u8>()
    }
    pub fn off_track_limit_exceeded(&self) -> OffTrackLimitExceeded {
        self.off_track_limit_exceeded_raw().into()
    }
    pub fn xoverride_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[6usize..8usize].load_le::<u8>()
    }
    pub fn xoverride(&self) -> Override {
        self.xoverride_raw().into()
    }
    pub fn steering_mode_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[8usize..11usize].load_le::<u8>()
    }
    pub fn steering_mode(&self) -> SteeringMode {
        self.steering_mode_raw().into()
    }
    pub fn turn_mode_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[11usize..14usize].load_le::<u8>()
    }
    pub fn turn_mode(&self) -> TurnMode {
        self.turn_mode_raw().into()
    }
    pub fn heading_reference_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[14usize..16usize].load_le::<u8>()
    }
    pub fn heading_reference(&self) -> HeadingReference {
        self.heading_reference_raw().into()
    }
    pub fn commanded_rudder_direction_raw(&self) -> u8 {
        self.raw.view_bits::<Lsb0>()[21usize..24usize].load_le::<u8>()
    }
    pub fn commanded_rudder_direction(&self) -> CommandedRudderDirection {
        self.commanded_rudder_direction_raw().into()
    }
    #[doc = "Unit: rad"]
    pub fn commanded_rudder_angle_raw(&self) -> i16 {
        let value = self.raw.view_bits::<Lsb0>()[24usize..40usize].load_le::<u16>();
        i16::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn commanded_rudder_angle(&self) -> Option<f32> {
        let raw_value = self.commanded_rudder_angle_raw();
        if raw_value == i16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    #[doc = "Unit: rad"]
    pub fn heading_to_steer_course_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[40usize..56usize].load_le::<u16>()
    }
    pub fn heading_to_steer_course(&self) -> Option<f32> {
        let raw_value = self.heading_to_steer_course_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    #[doc = "Unit: rad"]
    pub fn track_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[56usize..72usize].load_le::<u16>()
    }
    pub fn track(&self) -> Option<f32> {
        let raw_value = self.track_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    #[doc = "Unit: rad"]
    pub fn rudder_limit_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[72usize..88usize].load_le::<u16>()
    }
    pub fn rudder_limit(&self) -> Option<f32> {
        let raw_value = self.rudder_limit_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    #[doc = "Unit: rad"]
    pub fn off_heading_limit_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[88usize..104usize].load_le::<u16>()
    }
    pub fn off_heading_limit(&self) -> Option<f32> {
        let raw_value = self.off_heading_limit_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    #[doc = "Unit: rad"]
    pub fn radius_of_turn_order_raw(&self) -> i16 {
        let value = self.raw.view_bits::<Lsb0>()[104usize..120usize].load_le::<u16>();
        i16::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn radius_of_turn_order(&self) -> Option<f32> {
        let raw_value = self.radius_of_turn_order_raw();
        if raw_value == i16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
    #[doc = "Unit: rad/s"]
    pub fn rate_of_turn_order_raw(&self) -> i16 {
        let value = self.raw.view_bits::<Lsb0>()[120usize..136usize].load_le::<u16>();
        i16::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn rate_of_turn_order(&self) -> Option<f32> {
        let raw_value = self.rate_of_turn_order_raw();
        if raw_value == i16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.00003125 as f32))
        }
    }
    #[doc = "Unit: m"]
    pub fn off_track_limit_raw(&self) -> i16 {
        let value = self.raw.view_bits::<Lsb0>()[136usize..152usize].load_le::<u16>();
        i16::from_ne_bytes(value.to_ne_bytes())
    }
    pub fn off_track_limit(&self) -> Option<i16> {
        let raw_value = self.off_track_limit_raw();
        if raw_value == i16::MAX {
            None
        } else {
            Some(raw_value)
        }
    }
    #[doc = "Unit: rad"]
    pub fn vessel_heading_raw(&self) -> u16 {
        self.raw.view_bits::<Lsb0>()[152usize..168usize].load_le::<u16>()
    }
    pub fn vessel_heading(&self) -> Option<f32> {
        let raw_value = self.vessel_heading_raw();
        if raw_value == u16::MAX {
            None
        } else {
            Some((raw_value as f32) * (0.0001 as f32))
        }
    }
}
impl core::fmt::Debug for HeadingTrackControl {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("HeadingTrackControl")
            .field("rudder_limit_exceeded", &self.rudder_limit_exceeded())
            .field(
                "off_heading_limit_exceeded",
                &self.off_heading_limit_exceeded(),
            )
            .field("off_track_limit_exceeded", &self.off_track_limit_exceeded())
            .field("xoverride", &self.xoverride())
            .field("steering_mode", &self.steering_mode())
            .field("turn_mode", &self.turn_mode())
            .field("heading_reference", &self.heading_reference())
            .field(
                "commanded_rudder_direction",
                &self.commanded_rudder_direction(),
            )
            .field("commanded_rudder_angle", &self.commanded_rudder_angle())
            .field("heading_to_steer_course", &self.heading_to_steer_course())
            .field("track", &self.track())
            .field("rudder_limit", &self.rudder_limit())
            .field("off_heading_limit", &self.off_heading_limit())
            .field("radius_of_turn_order", &self.radius_of_turn_order())
            .field("rate_of_turn_order", &self.rate_of_turn_order())
            .field("off_track_limit", &self.off_track_limit())
            .field("vessel_heading", &self.vessel_heading())
            .finish()
    }
}
