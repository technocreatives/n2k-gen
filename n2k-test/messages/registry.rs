pub struct PgnRegistry;
impl n2k::PgnRegistry for PgnRegistry {
    type Message = super::Pgn;
    type Error = super::types::N2kError;
    fn is_fast_packet(pgn: u32) -> bool {
        matches!(pgn, 126996 | 127237 | 127510)
    }
    fn build_message(pgn: u32, data: &[u8]) -> Result<Self::Message, Self::Error> {
        super::Pgn::try_from_bytes(pgn, data)
    }
}
