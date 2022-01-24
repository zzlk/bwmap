use crate::util::CursorSlicer;
use serde::Serialize;

// Not Required.

// This section goes along with the "UPRP" section. This section just indicates which of the 64 unit properties slot are used.

// u8[64]: 1 byte for each trigger unit properties slot
// 00 - Properties slot is unused
// 01 - Properties slot is used

#[derive(Debug, Serialize)]
pub struct ChkUpus<'a> {
    #[serde(skip_serializing)]
    pub cuwp_slot_is_used: &'a [u8; 64],
}

pub(crate) fn parse_upus(sec: &[u8]) -> Result<ChkUpus, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkUpus {
        cuwp_slot_is_used: slicer.extract_ref()?,
    })
}
