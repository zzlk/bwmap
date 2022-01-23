use crate::util::CursorSlicer;
use std::cmp::min;

// Not Required.

// There are 512 wav entires regardless of how many are actually used.

// u32[512]: 1 long for each WAV. Indicates a string index is used for a WAV path in the MPQ. If the entry is not used, it will be 0.

#[derive(Debug)]
pub struct ChkSprp<'a> {
    pub wav_string_number: &'a [u32; 512],
}

pub(crate) fn parse_sprp(sec: &[u8]) -> Result<ChkSprp, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkSprp {
        wav_string_number: slicer.extract_ref()?,
    })
}
