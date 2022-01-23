use crate::util::CursorSlicer;
use serde::Serialize;
use std::cmp::min;

// Required for all versions and all game types.
// Validation: Must be size of 4 bytes.

// u16: String number of the scenario name
// u16: String number of the scenarios description.
// A string index of 0 for the map name will default it to its file name. A string index of 0 description will default to a predefined string.

#[derive(Debug, Serialize)]
pub struct ChkSprp<'a> {
    pub scenario_name_string_number: &'a u16,
    pub description_string_number: &'a u16,
}

pub(crate) fn parse_sprp(sec: &[u8]) -> Result<ChkSprp, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkSprp {
        scenario_name_string_number: slicer.extract_ref()?,
        description_string_number: slicer.extract_ref()?,
    })
}
