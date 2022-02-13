use crate::util::CursorSlicer;
use serde::Serialize;

// Not Required.

// There are 512 wav entires regardless of how many are actually used.

// u32[512]: 1 long for each WAV. Indicates a string index is used for a WAV path in the MPQ. If the entry is not used, it will be 0.

#[derive(Debug, Serialize)]
pub struct ChkWav<'a> {
    pub wav_string_number: &'a [u32],
}

pub(crate) fn parse_wav(sec: &[u8]) -> Result<ChkWav, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    Ok(ChkWav {
        wav_string_number: slicer.extract_rest_as_slice_lax()?,
    })
}
