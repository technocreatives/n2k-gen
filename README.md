# n2k-gen

Code generator for NMEA2000 messages from pgns.xml of canboat (https://github.com/canboat/canboat/blob/master/analyzer/pgns.xml) and NMEA2000 CAN bus parsing library based on embedded_hal_can.

## n2k-codegen

Code generator for PGN parsers from completed N2K messages.

## n2k

Built to transparently handle multi-part n2k messages on top of a CAN bus abstracted by embedded_hal_can. Interfaces with the generated code by the code generator through the `PgnRegistry` trait.

## TODO
- [x] Interface to identify fast packets and assemble
- [ ] Sending
  - [ ] Single frames
  - [ ] Fast packets
  - [x] ISO Transport Protocol multi-part messages
- [ ] Receiving and re-assembling of ISO Transport Protocol multi-part messages
- [ ] ISO functions
  - [ ] Address claim
  - [ ] Product Information
  - [ ] Device Information
  - [ ] Transmit Messages
- [ ] Example test project
- [ ] Sub-PGN message decoding/proprietary PGNS. Example: PGN 130820 represents multiple possible Fusion messages, the <Match> key in the XML disambiguates between different types

### Failing PGNs
These PGNs currently don't generate properly for reasons:
- 129540 / gnssSatsInView: variable length
- 130820 / Fusion: proprietary PGN