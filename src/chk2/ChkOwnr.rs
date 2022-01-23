use crate::util::CursorSlicer;
use std::cmp::min;

// Required for all versions and all game types.
// Validation: Must be size of 12 bytes.

// This section designates the controller of a particular player. It is exactly the same as the "IOWN" section, except there is an additional value, 0x00 for Inactive.

// u8[12]: One byte for each player, specifies the owner of the player:
// 00 - Inactive
// 01 - Computer (game)
// 02 - Occupied by Human Player
// 03 - Rescue Passive
// 04 - Unused
// 05 - Computer
// 06 - Human (Open Slot)
// 07 - Neutral
// 08 - Closed slot
// Italicized settings denote invalid map options, which may involve a buffer overflow.

#[derive(Debug)]
pub struct ChkOwnr<'a> {
    pub player_owner: &'a [u8; 12],
}

pub(crate) fn parse_ownr(sec: &[u8]) -> Result<ChkOwnr, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkOwnr {
        player_owner: slicer.extract_ref()?,
    })
}
