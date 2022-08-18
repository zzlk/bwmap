use serde::{Deserialize, Serialize};
use tracing::instrument;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
pub enum ChunkName {
    TYPE,
    VER,
    IVER,
    IVE2,
    VCOD,
    IOWN,
    OWNR,
    ERA,
    DIM,
    SIDE,
    MTXM,
    PUNI,
    UPGR,
    PTEC,
    UNIT,
    ISOM,
    TILE,
    DD2,
    THG2,
    MASK,
    STR,
    STRx,
    UPRP,
    UPUS,
    MRGN,
    TRIG,
    MBRF,
    SPRP,
    FORC,
    WAV,
    UNIS,
    UPGS,
    TECS,
    SWNM,
    COLR,
    CRGB,
    PUPx,
    PTEx,
    UNIx,
    UPGx,
    TECx,
    UNKNOWN(String),
}

#[instrument(level = "trace", skip_all)]
pub(crate) fn parse_chunk_name(chunk_name: &[u8]) -> ChunkName {
    match chunk_name {
        b"TYPE" => ChunkName::TYPE,
        b"VER " => ChunkName::VER,
        b"IVER" => ChunkName::IVER,
        b"IVE2" => ChunkName::IVE2,
        b"VCOD" => ChunkName::VCOD,
        b"IOWN" => ChunkName::IOWN,
        b"OWNR" => ChunkName::OWNR,
        b"ERA " => ChunkName::ERA,
        b"DIM " => ChunkName::DIM,
        b"SIDE" => ChunkName::SIDE,
        b"MTXM" => ChunkName::MTXM,
        b"PUNI" => ChunkName::PUNI,
        b"UPGR" => ChunkName::UPGR,
        b"PTEC" => ChunkName::PTEC,
        b"UNIT" => ChunkName::UNIT,
        b"ISOM" => ChunkName::ISOM,
        b"TILE" => ChunkName::TILE,
        b"DD2 " => ChunkName::DD2,
        b"THG2" => ChunkName::THG2,
        b"MASK" => ChunkName::MASK,
        b"STR " => ChunkName::STR,
        b"STRx" => ChunkName::STRx,
        b"UPRP" => ChunkName::UPRP,
        b"UPUS" => ChunkName::UPUS,
        b"MRGN" => ChunkName::MRGN,
        b"TRIG" => ChunkName::TRIG,
        b"MBRF" => ChunkName::MBRF,
        b"SPRP" => ChunkName::SPRP,
        b"FORC" => ChunkName::FORC,
        b"WAV " => ChunkName::WAV,
        b"UNIS" => ChunkName::UNIS,
        b"UPGS" => ChunkName::UPGS,
        b"TECS" => ChunkName::TECS,
        b"SWNM" => ChunkName::SWNM,
        b"COLR" => ChunkName::COLR,
        b"CRGB" => ChunkName::CRGB,
        b"PUPx" => ChunkName::PUPx,
        b"PTEx" => ChunkName::PTEx,
        b"UNIx" => ChunkName::UNIx,
        b"UPGx" => ChunkName::UPGx,
        b"TECx" => ChunkName::TECx,
        _ => ChunkName::UNKNOWN(encoding_rs::WINDOWS_1252.decode(chunk_name).0.to_string()),
    }
}

pub(crate) enum ChunkNameUpdateType {
    FullOverwrite,
    PartialOverwrite,
    Append,
}

#[instrument(level = "trace", skip_all)]
pub(crate) fn get_chunk_update_type(name: &ChunkName) -> ChunkNameUpdateType {
    match name {
        ChunkName::MTXM => ChunkNameUpdateType::PartialOverwrite,
        ChunkName::STR => ChunkNameUpdateType::PartialOverwrite,
        ChunkName::STRx => ChunkNameUpdateType::PartialOverwrite,
        ChunkName::TILE => ChunkNameUpdateType::PartialOverwrite,

        ChunkName::UNIT => ChunkNameUpdateType::Append,
        ChunkName::THG2 => ChunkNameUpdateType::Append,
        ChunkName::TRIG => ChunkNameUpdateType::Append,
        ChunkName::MBRF => ChunkNameUpdateType::Append,

        _ => ChunkNameUpdateType::FullOverwrite,
    }
}
