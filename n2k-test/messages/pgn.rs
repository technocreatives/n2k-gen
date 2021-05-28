use super::types::*;
use core::convert::TryFrom;
#[derive(Debug)]
pub enum Pgn {
    IsoRequest(super::IsoRequest),
    IsoAddressClaim(super::IsoAddressClaim),
    SeatalkPilotHeading(super::SeatalkPilotHeading),
    ProductInformation(super::ProductInformation),
    HeadingTrackControl(super::HeadingTrackControl),
    Rudder(super::Rudder),
    VesselHeading(super::VesselHeading),
    RateOfTurn(super::RateOfTurn),
    Attitude(super::Attitude),
    FluidLevel(super::FluidLevel),
    DcDetailedStatus(super::DcDetailedStatus),
    ChargerConfigurationStatus(super::ChargerConfigurationStatus),
    PositionRapidUpdate(super::PositionRapidUpdate),
    WindData(super::WindData),
    Temperature(super::Temperature),
    ActualPressure(super::ActualPressure),
    TemperatureExtendedRange(super::TemperatureExtendedRange),
}
impl Pgn {
    pub fn try_from_bytes(pgn: u32, bytes: &[u8]) -> Result<Pgn, N2kError> {
        Ok(match pgn {
            59904u32 => Pgn::IsoRequest(super::IsoRequest::try_from(bytes)?),
            60928u32 => Pgn::IsoAddressClaim(super::IsoAddressClaim::try_from(bytes)?),
            65359u32 => Pgn::SeatalkPilotHeading(super::SeatalkPilotHeading::try_from(bytes)?),
            126996u32 => Pgn::ProductInformation(super::ProductInformation::try_from(bytes)?),
            127237u32 => Pgn::HeadingTrackControl(super::HeadingTrackControl::try_from(bytes)?),
            127245u32 => Pgn::Rudder(super::Rudder::try_from(bytes)?),
            127250u32 => Pgn::VesselHeading(super::VesselHeading::try_from(bytes)?),
            127251u32 => Pgn::RateOfTurn(super::RateOfTurn::try_from(bytes)?),
            127257u32 => Pgn::Attitude(super::Attitude::try_from(bytes)?),
            127505u32 => Pgn::FluidLevel(super::FluidLevel::try_from(bytes)?),
            127506u32 => Pgn::DcDetailedStatus(super::DcDetailedStatus::try_from(bytes)?),
            127510u32 => {
                Pgn::ChargerConfigurationStatus(super::ChargerConfigurationStatus::try_from(bytes)?)
            }
            129025u32 => Pgn::PositionRapidUpdate(super::PositionRapidUpdate::try_from(bytes)?),
            130306u32 => Pgn::WindData(super::WindData::try_from(bytes)?),
            130312u32 => Pgn::Temperature(super::Temperature::try_from(bytes)?),
            130314u32 => Pgn::ActualPressure(super::ActualPressure::try_from(bytes)?),
            130316u32 => {
                Pgn::TemperatureExtendedRange(super::TemperatureExtendedRange::try_from(bytes)?)
            }
            pgn => return Err(N2kError::UnknownPgn(pgn)),
        })
    }
}
