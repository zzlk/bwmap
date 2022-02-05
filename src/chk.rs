// 1.1 "TYPE" - Map Type
// 1.2 "VER " - Format Version
// 1.3 "IVER" - Map Version
// 1.4 "IVE2" - Map Version
// 1.5 "VCOD" - Verification Code
// 1.6 "IOWN" - StarEdit Player Types
// 1.7 "OWNR" - StarCraft Player Types
// 1.8 "ERA " - Tileset
// 1.9 "DIM " - Map Dimensions
// 1.10 "SIDE" - Player Races
// 1.11 "MTXM" - StarCraft Terrain
// 1.12 "PUNI" - Player Unit Restrictions
// 1.13 "UPGR" - Upgrade Restrictions
// 1.14 "PTEC" - Tech Restrictions
// 1.15 "UNIT" - Placed Units
// 1.16 "ISOM" - Isometric Terrain
// 1.17 "TILE" - StarEdit Terrain
// 1.18 "DD2 " - StarEdit Sprites (Doodads)
// 1.19 "THG2" - StarCraft Sprites
// 1.20 "MASK" - Fog of War Layer
// 1.21 "STR " - String Data
// 1.22 "UPRP" - CUWP Slots
// 1.23 "UPUS" - CUWP Slots Used
// 1.24 "MRGN" - Locations
// 1.25 "TRIG" - Triggers
// 1.26 "MBRF" - Mission Briefings
// 1.27 "SPRP" - Scenario Properties
// 1.28 "FORC" - Force Settings
// 1.29 "WAV " - WAV String Indexes
// 1.30 "UNIS" - Unit Settings
// 1.31 "UPGS" - Upgrade Settings
// 1.32 "TECS" - Tech Settings
// 1.33 "SWNM" - Switch Names
// 1.34 "COLR" - Player Colors
// 1.35 "PUPx" - BW Upgrade Restrictions
// 1.36 "PTEx" - BW Tech Restrictions
// 1.37 "UNIx" - BW Unit Settings
// 1.38 "UPGx" - BW Upgrade Settings
// 1.39 "TECx" - BW Tech Settings

use std::{convert::TryFrom, ffi::CStr};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::{
    chk2::{
        chk_colr::{parse_colr, ChkColr},
        chk_crgb::{parse_crgb, ChkCrgb},
        chk_dd2::{parse_dd2, ChkDd2},
        chk_dim::{parse_dim, ChkDim},
        chk_era::{parse_era, ChkEra},
        chk_forc::{parse_forc, ChkForc},
        chk_iown::{parse_iown, ChkIown},
        chk_isom::{parse_isom, ChkIsom},
        chk_ive2::{parse_ive2, ChkIve2},
        chk_iver::{parse_iver, ChkIver},
        chk_mask::{parse_mask, ChkMask},
        chk_mbrf::{parse_mbrf, ChkMbrf},
        chk_mrgn::{parse_mrgn, ChkMrgn},
        chk_mtxm::{parse_mtxm, ChkMtxm},
        chk_ownr::{parse_ownr, ChkOwnr},
        chk_ptec::{parse_ptec, ChkPtec},
        chk_ptex::{parse_ptex, ChkPtex},
        chk_puni::{parse_puni, ChkPuni},
        chk_pupx::{parse_pupx, ChkPupx},
        chk_side::{parse_side, ChkSide},
        chk_sprp::{parse_sprp, ChkSprp},
        chk_str::{parse_str, ChkStr},
        chk_strx::{parse_strx, ChkStrx},
        chk_swnm::{parse_swnm, ChkSwnm},
        chk_tecs::{parse_tecs, ChkTecs},
        chk_tecx::{parse_tecx, ChkTecx},
        chk_thg2::{parse_thg2, ChkThg2},
        chk_tile::{parse_tile, ChkTile},
        chk_trig::{parse_trig, ChkTrig, ChkTrigAction, ChkTrigCondition},
        chk_type::{parse_type, ChkType},
        chk_unis::{parse_unis, ChkUnis},
        chk_unit::{parse_unit, ChkUnit},
        chk_unix::{parse_unix, ChkUnix},
        chk_upgr::{parse_upgr, ChkUpgr},
        chk_upgs::{parse_upgs, ChkUpgs},
        chk_upgx::{parse_upgx, ChkUpgx},
        chk_uprp::{parse_uprp, ChkUprp},
        chk_upus::{parse_upus, ChkUpus},
        chk_vcod::{parse_vcod, ChkVcod},
        chk_ver::{parse_ver, ChkVer},
        chk_wav::{parse_wav, ChkWav},
    },
    util::parse_null_terminated_bytestring_unsigned,
};
use std::str;

//use sha2::{Sha256, Digest};

#[derive(Debug, Serialize, Deserialize)]
struct ChunkDataType {
    type_of_scenario: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChunkDataDim {
    width: u16,
    height: u16,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChunkDataEra {
    tileset: u16,
}

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

enum ChunkNameUpdateType {
    FullOverwrite,
    PartialOverwrite,
    Append,
}

fn get_update_type(name: &ChunkName) -> ChunkNameUpdateType {
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

fn parse_name(chunk_name: &[u8]) -> ChunkName {
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

#[derive(Debug, Serialize, Deserialize)]
pub struct RawChunk {
    pub name: ChunkName,
    pub size: i32,
    pub offset: usize,
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}

pub fn parse_chk(chk: &[u8]) -> Vec<RawChunk> {
    let mut offset = 0;
    let mut ret = Vec::new();

    loop {
        let start_offset = offset;
        if offset + 4 >= chk.len() {
            break;
        }
        let name = parse_name(&chk[offset..offset + 4]);
        offset += 4;

        if offset + 4 >= chk.len() {
            break;
        }
        let size: i32 = crate::util::parse_slice(&chk[offset..offset + 4]);
        offset += 4;

        // TODO: I have heard? some map protectors use negative sizes to stack sections, how to do.
        if size < 0 {
            break;
        }

        if offset + size as usize > chk.len() {
            break;
        }

        let data = chk[offset..offset + size.abs() as usize].to_vec();
        offset += usize::try_from(size).unwrap();

        ret.push(RawChunk {
            name,
            size,
            offset: start_offset,
            data,
        });
    }

    ret
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergedChunk {
    pub name: ChunkName,
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}

pub fn merge_raw_chunks(chunks: &[RawChunk]) -> HashMap<ChunkName, MergedChunk> {
    let mut map = HashMap::new();

    for v in chunks {
        match get_update_type(&v.name) {
            ChunkNameUpdateType::FullOverwrite => {
                let merged = MergedChunk {
                    name: v.name.clone(),
                    data: v.data.clone(),
                };

                map.insert(merged.name.clone(), merged);
            }

            ChunkNameUpdateType::PartialOverwrite => {
                if let Some(c) = map.get(&v.name) {
                    let mut merged = MergedChunk {
                        name: c.name.clone(),
                        data: Vec::new(),
                    };

                    merged.data.extend(v.data.as_slice());

                    if c.data.len() > v.data.len() {
                        merged.data.extend(&c.data[v.data.len()..]);
                    }

                    map.insert(merged.name.clone(), merged);
                } else {
                    let merged = MergedChunk {
                        name: v.name.clone(),
                        data: v.data.clone(),
                    };

                    map.insert(v.name.clone(), merged);
                }
            }

            ChunkNameUpdateType::Append => {
                if let Some(c) = map.get(&v.name) {
                    let mut merged = MergedChunk {
                        name: c.name.clone(),
                        data: Vec::new(),
                    };

                    merged.data.extend_from_slice(v.data.as_slice());
                    merged.data.extend_from_slice(&c.data);

                    map.insert(merged.name.clone(), merged);
                } else {
                    let merged = MergedChunk {
                        name: v.name.clone(),
                        data: v.data.clone(),
                    };

                    map.insert(v.name.clone(), merged);
                }
            }
        }
    }

    map
}

#[derive(Debug, Serialize)]
pub enum ParsedChunk<'a> {
    COLR(ChkColr<'a>),
    CRGB(ChkCrgb<'a>),
    DD2(ChkDd2<'a>),
    DIM(ChkDim<'a>),
    ERA(ChkEra<'a>),
    FORC(ChkForc<'a>),
    IOWN(ChkIown<'a>),
    ISOM(ChkIsom),
    IVE2(ChkIve2<'a>),
    IVER(ChkIver<'a>),
    MASK(ChkMask<'a>),
    MBRF(ChkMbrf<'a>),
    MRGN(ChkMrgn<'a>),
    MTXM(ChkMtxm),
    OWNR(ChkOwnr<'a>),
    PTEC(ChkPtec<'a>),
    PTEx(ChkPtex<'a>),
    PUNI(ChkPuni<'a>),
    PUPx(ChkPupx<'a>),
    SIDE(ChkSide<'a>),
    SPRP(ChkSprp<'a>),
    STR(ChkStr<'a>),
    STRx(ChkStrx<'a>),
    SWNM(ChkSwnm<'a>),
    TECS(ChkTecs<'a>),
    TECx(ChkTecx<'a>),
    THG2(ChkThg2<'a>),
    TILE(ChkTile),
    TRIG(ChkTrig<'a>),
    TYPE(ChkType<'a>),
    UNIS(ChkUnis<'a>),
    UNIT(ChkUnit<'a>),
    UNIx(ChkUnix<'a>),
    UPGR(ChkUpgr<'a>),
    UPGS(ChkUpgs<'a>),
    UPGx(ChkUpgx<'a>),
    UPRP(ChkUprp<'a>),
    UPUS(ChkUpus<'a>),
    VCOD(ChkVcod<'a>),
    VER(ChkVer<'a>),
    WAV(ChkWav<'a>),
}

pub fn parse_merged_chunks(
    chunks: &HashMap<ChunkName, MergedChunk>,
) -> Result<HashMap<ChunkName, ParsedChunk>, anyhow::Error> {
    let mut map = HashMap::new();
    for (chunk_name, chunk) in chunks.into_iter() {
        match chunk_name {
            ChunkName::COLR => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::COLR(parse_colr(chunk.data.as_slice())?),
                );
            }
            ChunkName::CRGB => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::CRGB(parse_crgb(chunk.data.as_slice())?),
                );
            }
            ChunkName::DD2 => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::DD2(parse_dd2(chunk.data.as_slice())?),
                );
            }
            ChunkName::DIM => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::DIM(parse_dim(chunk.data.as_slice())?),
                );
            }
            ChunkName::ERA => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::ERA(parse_era(chunk.data.as_slice())?),
                );
            }
            ChunkName::FORC => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::FORC(parse_forc(chunk.data.as_slice())?),
                );
            }
            ChunkName::IOWN => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::IOWN(parse_iown(chunk.data.as_slice())?),
                );
            }
            ChunkName::ISOM => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::ISOM(parse_isom(chunk.data.as_slice())?),
                );
            }
            ChunkName::IVE2 => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::IVE2(parse_ive2(chunk.data.as_slice())?),
                );
            }
            ChunkName::IVER => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::IVER(parse_iver(chunk.data.as_slice())?),
                );
            }
            ChunkName::MASK => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::MASK(parse_mask(chunk.data.as_slice())?),
                );
            }
            ChunkName::MBRF => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::MBRF(parse_mbrf(chunk.data.as_slice())?),
                );
            }
            ChunkName::MRGN => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::MRGN(parse_mrgn(chunk.data.as_slice())?),
                );
            }
            ChunkName::MTXM => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::MTXM(parse_mtxm(chunk.data.as_slice())?),
                );
            }
            ChunkName::OWNR => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::OWNR(parse_ownr(chunk.data.as_slice())?),
                );
            }
            ChunkName::PTEC => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::PTEC(parse_ptec(chunk.data.as_slice())?),
                );
            }
            ChunkName::PTEx => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::PTEx(parse_ptex(chunk.data.as_slice())?),
                );
            }
            ChunkName::PUNI => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::PUNI(parse_puni(chunk.data.as_slice())?),
                );
            }
            ChunkName::PUPx => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::PUPx(parse_pupx(chunk.data.as_slice())?),
                );
            }
            ChunkName::SIDE => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::SIDE(parse_side(chunk.data.as_slice())?),
                );
            }
            ChunkName::SPRP => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::SPRP(parse_sprp(chunk.data.as_slice())?),
                );
            }
            ChunkName::STR => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::STR(parse_str(chunk.data.as_slice())?),
                );
            }
            ChunkName::STRx => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::STRx(parse_strx(chunk.data.as_slice())?),
                );
            }
            ChunkName::SWNM => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::SWNM(parse_swnm(chunk.data.as_slice())?),
                );
            }
            ChunkName::TECS => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::TECS(parse_tecs(chunk.data.as_slice())?),
                );
            }
            ChunkName::TECx => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::TECx(parse_tecx(chunk.data.as_slice())?),
                );
            }
            ChunkName::THG2 => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::THG2(parse_thg2(chunk.data.as_slice())?),
                );
            }
            ChunkName::TILE => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::TILE(parse_tile(chunk.data.as_slice())?),
                );
            }
            ChunkName::TRIG => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::TRIG(parse_trig(chunk.data.as_slice())?),
                );
            }
            ChunkName::TYPE => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::TYPE(parse_type(chunk.data.as_slice())?),
                );
            }
            ChunkName::UNIS => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::UNIS(parse_unis(chunk.data.as_slice())?),
                );
            }
            ChunkName::UNIx => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::UNIx(parse_unix(chunk.data.as_slice())?),
                );
            }
            ChunkName::UNIT => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::UNIT(parse_unit(chunk.data.as_slice())?),
                );
            }
            ChunkName::UPGR => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::UPGR(parse_upgr(chunk.data.as_slice())?),
                );
            }
            ChunkName::UPGS => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::UPGS(parse_upgs(chunk.data.as_slice())?),
                );
            }
            ChunkName::UPGx => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::UPGx(parse_upgx(chunk.data.as_slice())?),
                );
            }
            ChunkName::UPRP => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::UPRP(parse_uprp(chunk.data.as_slice())?),
                );
            }
            ChunkName::UPUS => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::UPUS(parse_upus(chunk.data.as_slice())?),
                );
            }
            ChunkName::VCOD => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::VCOD(parse_vcod(chunk.data.as_slice())?),
                );
            }
            ChunkName::VER => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::VER(parse_ver(chunk.data.as_slice())?),
                );
            }
            ChunkName::WAV => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::WAV(parse_wav(chunk.data.as_slice())?),
                );
            }
            ChunkName::UNKNOWN(_) => {}
        }
    }

    Ok(map)
}

pub fn merge_rawchunks(chunks: &[RawChunk]) -> Vec<MergedChunk> {
    let mut map = HashMap::new();

    for v in chunks {
        match get_update_type(&v.name) {
            ChunkNameUpdateType::FullOverwrite => {
                let merged = MergedChunk {
                    name: v.name.clone(),
                    data: v.data.clone(),
                };

                map.insert(merged.name.clone(), merged);
            }

            ChunkNameUpdateType::PartialOverwrite => {
                if let Some(c) = map.get(&v.name) {
                    let mut merged = MergedChunk {
                        name: c.name.clone(),
                        data: Vec::new(),
                    };

                    merged.data.extend(v.data.as_slice());

                    if c.data.len() > v.data.len() {
                        merged.data.extend(&c.data[v.data.len()..]);
                    }

                    map.insert(merged.name.clone(), merged);
                } else {
                    let merged = MergedChunk {
                        name: v.name.clone(),
                        data: v.data.clone(),
                    };

                    map.insert(v.name.clone(), merged);
                }
            }

            ChunkNameUpdateType::Append => {
                if let Some(c) = map.get(&v.name) {
                    let mut merged = MergedChunk {
                        name: c.name.clone(),
                        data: Vec::new(),
                    };

                    merged.data.extend_from_slice(v.data.as_slice());
                    merged.data.extend_from_slice(&c.data);

                    map.insert(merged.name.clone(), merged);
                } else {
                    let merged = MergedChunk {
                        name: v.name.clone(),
                        data: v.data.clone(),
                    };

                    map.insert(v.name.clone(), merged);
                }
            }
        }
    }

    let mut ret = Vec::new();
    for v in chunks.iter().rev() {
        if let Some(c) = map.remove(&v.name) {
            ret.push(c);
        }
    }

    ret.reverse();

    ret
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UnitSetting {
    pub id: u8,
    pub config: u8,
    pub points_hit: u32,
    pub points_shield: u16,
    pub points_armor: u8,
    pub time_build: u16,
    pub cost_minerals: u16,
    pub cost_gas: u16,
    pub string_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpgradeSettings {
    pub id: u8,
    pub base_weapon_damage: u16,
    pub bonus_weapon_damage: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Forces {
    pub force_players: Vec<u8>,
    pub force_names: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChkDump {
    pub map_ver: u16,
    pub map_width: u16,
    pub map_height: u16,
    pub scenario_name: String,
    pub scenario_description: String,
    pub era: u16,
    pub mtxm: Vec<u16>,
    pub tile: Vec<u16>,
    pub unit_settings: Vec<UnitSetting>,
    pub forces: Forces,
    pub strings: Vec<String>,
}

fn parse_string_table(str_section: &[u8], is_strx: bool) -> Vec<Vec<u8>> {
    let mut strings = Vec::new();

    let max_index = if is_strx {
        if str_section.len() <= 4 {
            return strings;
        }

        crate::util::parse_slice::<u32>(&str_section[0..4]) as usize
    } else {
        if str_section.len() <= 2 {
            return strings;
        }
        crate::util::parse_slice::<u16>(&str_section[0..2]) as usize
    };

    for index in 0..max_index {
        let offset_string = if is_strx {
            let offset_index = (4 + (index * 4)) as usize;

            if offset_index + 4 > str_section.len() {
                break;
            }

            crate::util::parse_slice::<u32>(&str_section[offset_index..offset_index + 4]) as usize
        } else {
            let offset_index = (2 + (index * 2)) as usize;

            if offset_index + 2 > str_section.len() {
                break;
            }

            crate::util::parse_slice::<u16>(&str_section[offset_index..offset_index + 2]) as usize
        };

        if offset_string > str_section.len() {
            strings.push(vec![0]);
            continue;
        }

        let mut len_string = 0;
        for &c in &str_section[offset_string..] {
            if c == 0u8 {
                break;
            }

            len_string += 1;
        }

        strings.push(str_section[offset_string..offset_string + len_string].to_vec());
    }

    strings
}

struct ParsedUNIS {
    id: u8,
    config: u8,
    points_hit: u32,
    points_shield: u16,
    points_armor: u8,
    time_build: u16,
    cost_minerals: u16,
    cost_gas: u16,
    string_number: u16,
}

#[allow(non_snake_case)]
fn parse_UNIS(sec: &[u8]) -> Vec<ParsedUNIS> {
    // u8[228]: 1 byte for each unit, in order of Unit ID
    // 00 - Unit does not use default settings
    // 01 - Unit does use default settings
    // u32[228]: Hit points for unit (Note the displayed value is this value / 256, with the low byte being a fractional HP value)
    // u16[228]: Shield points, in order of Unit ID
    // u8[228]: Armor points, in order of Unit ID
    // u16[228]: Build time (1/60 seconds), in order of Unit ID
    // u16[228]: Mineral cost, in order of Unit ID
    // u16[228]: Gas cost, in order of Unit ID
    // u16[228]: String number, in order of Unit ID
    // u16[100]: Base weapon damage the weapon does, in weapon ID order (#List of Unit Weapon IDs)
    // u16[100]: Upgrade bonus weapon damage, in weapon ID order

    let mut ret = Vec::new();

    let mut offset = 0;
    let slice_config = crate::util::reinterpret_slice::<u8>(&sec[offset..offset + 228]);
    offset += 228;
    let slice_points_hit = crate::util::reinterpret_slice::<u32>(&sec[offset..offset + 4 * 228]);
    offset += 4 * 228;
    let slice_points_shield = crate::util::reinterpret_slice::<u16>(&sec[offset..offset + 2 * 228]);
    offset += 2 * 228;
    let slice_points_armor = crate::util::reinterpret_slice::<u8>(&sec[offset..offset + 228]);
    offset += 228;
    let slice_time_build = crate::util::reinterpret_slice::<u16>(&sec[offset..offset + 2 * 228]);
    offset += 2 * 228;
    let slice_cost_minerals = crate::util::reinterpret_slice::<u16>(&sec[offset..offset + 2 * 228]);
    offset += 2 * 228;
    let slice_cost_gas = crate::util::reinterpret_slice::<u16>(&sec[offset..offset + 2 * 228]);
    offset += 2 * 228;
    let slice_strings = crate::util::reinterpret_slice::<u16>(&sec[offset..offset + 2 * 228]); //offset += 2*228;

    // I suppose these are UNIx only?
    //let slice_base_weapon_damage = &sec[offset ..offset + 2*100]; offset += 2*228;
    //let slice_bonus_weapon_damage = &sec[offset ..offset + 2*100]; offset += 2*228;

    for unit_id in 0..228usize {
        ret.push(ParsedUNIS {
            id: unit_id as u8,
            config: slice_config[unit_id],
            points_hit: slice_points_hit[unit_id],
            points_shield: slice_points_shield[unit_id],
            points_armor: slice_points_armor[unit_id],
            time_build: slice_time_build[unit_id],
            cost_minerals: slice_cost_minerals[unit_id],
            cost_gas: slice_cost_gas[unit_id],
            string_number: slice_strings[unit_id],
        });
    }

    ret
}

pub fn get_parsed_chk(merged_chunks: &Vec<MergedChunk>) -> anyhow::Result<ChkDump, anyhow::Error> {
    let mut hash_table = std::collections::HashMap::new();

    for chunk in merged_chunks.into_iter() {
        hash_table.insert(chunk.name.clone(), chunk.data.clone());
    }

    let (map_width, map_height) = if let Some(data) = hash_table.get(&ChunkName::DIM) {
        let map_width = crate::util::parse_slice(&data.as_slice()[0..2]);
        let map_height = crate::util::parse_slice(&data.as_slice()[2..4]);

        (map_width, map_height)
    } else {
        (0, 0)
    };

    let mtxm = if let Some(data) = hash_table.get(&ChunkName::MTXM) {
        crate::util::reinterpret_slice::<u16>(&data[0..(data.len() - (data.len() % 2))]).to_vec()
    } else {
        Vec::new()
    };

    let tile = if let Some(data) = hash_table.get(&ChunkName::TILE) {
        crate::util::reinterpret_slice::<u16>(&data[0..(data.len() - (data.len() % 2))]).to_vec()
    } else {
        Vec::new()
    };

    let era = if let Some(data) = hash_table.get(&ChunkName::ERA) {
        crate::util::parse_slice::<u16>(&data.as_slice()[0..2]) % 8 // some map protectors specify ridiculous values here
    } else {
        0
    };

    let map_ver = if let Some(data) = hash_table.get(&ChunkName::VER) {
        crate::util::parse_slice::<u16>(&data.as_slice())
    } else {
        65535
    };

    let string_table = if let Some(data) = hash_table.get(&ChunkName::STR) {
        parse_string_table(data, false)
    } else if let Some(data) = hash_table.get(&ChunkName::STRx) {
        parse_string_table(data, true)
    } else {
        Vec::new()
    };

    let unit_config = if let Some(data) = hash_table.get(&ChunkName::UNIx) {
        parse_UNIS(data)
    } else if let Some(data) = hash_table.get(&ChunkName::UNIS) {
        parse_UNIS(data)
    } else {
        Vec::new()
    };

    let (sprp_scenario_string_number, sprp_scenario_description_string_number) =
        if let Some(data) = hash_table.get(&ChunkName::SPRP) {
            let sprp_scenario_string_number: u16 = crate::util::parse_slice(&data.as_slice()[0..2]);
            let sprp_scenario_description_string_number: u16 =
                crate::util::parse_slice(&data.as_slice()[2..4]);

            // if scenario_string_index == 0 {
            //     scenario_name = "filename".to_string();
            // } else {
            //     let t = get_str(scenario_string_index - 1)?;
            //     scenario_name = t;
            // }

            // if description_string_index == 0 {
            //     scenario_description = "<empty>".to_string();
            // } else {
            //     let t = get_str(description_string_index - 1)?;
            //     scenario_description = t;
            // }

            (
                sprp_scenario_string_number,
                sprp_scenario_description_string_number,
            )
        } else {
            (0, 0)
        };

    let force_string_numbers = if let Some(data) = hash_table.get(&ChunkName::FORC) {
        let mut force_names = Vec::new();

        if data.len() < 8 + 2 * 4 {
        } else {
            for &x in crate::util::reinterpret_slice::<u16>(&data.as_slice()[8..8 + 2 * 4]) {
                force_names.push(x);
            }
        }

        // Required for all versions and all game types.
        // Validation: Must be less than or equal to 20 bytes.

        // This section specifies the forces and the information about them.

        // u8[8]: 1 byte for each active player, specifying which of the 4 forces (0-based) that the player's on
        // u16[4]: 1 integer for each force, string number of the name of the force
        // u8[4]: 1 byte for each force specifying the properties:
        // Bit 0 - Random start location
        // Bit 1 - Allies
        // Bit 2 - Allied victory
        // Bit 3 - Shared vision
        // Bit 4-7 - Unused
        // Notes about FORC:

        // If there is no string specified for a force name, it will default to a "Force #" string.
        // If this section is less than 20 bytes, the remaining bytes are defaulted to 0.
        // Players can be on a force greater than 4, however they will not appear in the game lobby.

        force_names
    } else {
        Vec::new()
    };

    let mut string_references = Vec::new();
    if sprp_scenario_string_number > 0 {
        string_references.push(sprp_scenario_string_number - 1)
    }
    if sprp_scenario_description_string_number > 0 {
        string_references.push(sprp_scenario_description_string_number - 1)
    }
    string_references.extend(
        force_string_numbers
            .iter()
            .filter(|&&x| x > 0)
            .map(|x| x - 1),
    );
    string_references.extend(
        unit_config
            .iter()
            .map(|x| x.string_number)
            .filter(|&x| x > 0)
            .map(|x| x - 1),
    );

    // calculate string usage now
    let string_table = {
        let mut euc_kr: i64 = 0;
        let mut utf8: i64 = 0;
        let mut utf16: i64 = 0;
        let win1252: i64 = 0;

        for v in string_references {
            if let Some(s) = string_table.get(v as usize) {
                let x = encoding_rs::EUC_KR.decode(s.as_slice()).0.to_string();
                //println!("{}", x);
                if x.contains('�') {
                    euc_kr -= 50;
                } else {
                    euc_kr += x.chars().filter(|&c| c >= '가' && c <= '힣').count() as i64;
                    //euc_kr += x.chars().filter(|&c| c >= '\u{7f}').count() as i64;
                }

                let x = encoding_rs::UTF_16LE.decode(s.as_slice()).0.to_string();
                //println!("{}", x);
                if x.contains('�') {
                    utf16 -= 50;
                }

                let x = encoding_rs::UTF_8.decode(s.as_slice()).0.to_string();
                //println!("{}", x);
                if x.contains('�') {
                    utf8 -= 50;
                } else {
                    utf8 += x.chars().filter(|&c| c >= '\u{7f}').count() as i64;
                }

                // let x = encoding_rs::WINDOWS_1252.decode(s.as_slice()).0.to_string();
                // let x: String = x.chars().filter(|&c| c == '\0' || c > '\u{1f}').collect();
                // if x.len() > 5
                //     && x.chars()
                //         .filter(|&c| c >= ' ' && c <= '~')
                //         .collect::<String>()
                //         == x
                // {
                //     win1252 += 1;
                // }
            }
        }

        let mut encodings = [
            (encoding_rs::WINDOWS_1252, win1252),
            (encoding_rs::EUC_KR, euc_kr),
            (encoding_rs::UTF_8, utf8),
            (encoding_rs::UTF_16LE, utf16),
        ];
        // sort_by is stable, so becuase wind_1252 is first, it is biased toward that when tied with other ones.
        // This is intentional.
        encodings.sort_by(|&(_, a), &(_, b)| a.cmp(&b).reverse());

        println!(
            "euc_kr: {}, utf8: {}, utf16: {}, win1252: {}",
            euc_kr, utf8, utf16, win1252
        );

        //println!("encodings: {:?}", encodings);

        // if utf8 == 0 && (map_ver == 64 || map_ver == 65) {
        //     win1252 = -1;
        // }

        let encoding = encodings[0].0;

        let string_table: Vec<_> = string_table
            .into_iter()
            .map(|v| encoding.decode(v.as_slice()).0.to_string())
            .collect();

        string_table
    };

    let scenario_name = if sprp_scenario_string_number == 0 {
        ">>>>Zero SPRP scenario index????<<<<".to_string()
    } else {
        string_table
            .get(sprp_scenario_string_number as usize - 1)
            .unwrap_or(&"<<<EMPTY>>>".to_string())
            .clone()
    };

    let scenario_description = if sprp_scenario_description_string_number == 0 {
        "".to_string()
    } else {
        string_table
            .get(sprp_scenario_description_string_number as usize - 1)
            .unwrap_or(&"<<<EMPTY>>>".to_string())
            .clone()
    };

    let unit_settings: Vec<_> = unit_config
        .into_iter()
        .map(|c| UnitSetting {
            id: c.id,
            config: c.config,
            points_hit: c.points_hit,
            points_shield: c.points_shield,
            points_armor: c.points_armor,
            time_build: c.time_build,
            cost_minerals: c.cost_minerals,
            cost_gas: c.cost_gas,
            string_name: if c.string_number == 0 {
                "".to_string()
            } else {
                string_table
                    .get(c.string_number as usize - 1)
                    .unwrap_or(&"".to_string())
                    .clone()
            },
        })
        .filter(|x| !x.string_name.is_empty())
        .collect();

    let forces_names: Vec<_> = force_string_numbers
        .into_iter()
        .map(|c| {
            if c == 0 {
                "".to_string()
            } else {
                string_table
                    .get(c as usize - 1)
                    .unwrap_or(&"".to_string())
                    .clone()
            }
        })
        .filter(|x| !x.is_empty())
        .collect();

    let forces = Forces {
        force_players: Vec::new(),
        force_names: forces_names,
    };

    Ok(ChkDump {
        map_ver,
        map_width,
        map_height,
        scenario_name: scenario_name,
        scenario_description: scenario_description,
        era,
        mtxm,
        tile,
        unit_settings: unit_settings,
        forces: forces,
        strings: string_table,
    })
}

pub(crate) fn verify_is_valid_chk(chk: &[u8]) -> bool {
    let raw_chunks = parse_chk(&chk);
    let merged_chunks = merge_raw_chunks(&raw_chunks);
    let parsed_chunks = parse_merged_chunks(&merged_chunks).unwrap();

    parsed_chunks.get(&ChunkName::VCOD).is_some()
}

pub(crate) fn get_all_string_references(
    map: &HashMap<ChunkName, ParsedChunk>,
) -> Result<Vec<u32>, anyhow::Error> {
    let mut ret = Vec::new();

    if let Some(ParsedChunk::SPRP(x)) = map.get(&ChunkName::SPRP) {
        ret.push(*x.scenario_name_string_number as u32);
        ret.push(*x.description_string_number as u32);
    }

    if let Some(ParsedChunk::FORC(x)) = map.get(&ChunkName::FORC) {
        for string_number in x.force_name {
            ret.push(*string_number as u32);
        }
    }

    if let Some(ParsedChunk::UNIx(x)) = map.get(&ChunkName::UNIx) {
        for i in 0..x.string_number.len() {
            if x.config[i] == 0 {
                ret.push(x.string_number[i] as u32);
            }
        }
    } else if let Some(ParsedChunk::UNIS(x)) = map.get(&ChunkName::UNIS) {
        for i in 0..x.string_number.len() {
            if x.config[i] == 0 {
                ret.push(x.string_number[i] as u32);
            }
        }
    }

    if let Some(ParsedChunk::WAV(x)) = map.get(&ChunkName::WAV) {
        for string_number in x.wav_string_number {
            ret.push(*string_number as u32);
        }
    }

    if let Some(ParsedChunk::SWNM(x)) = map.get(&ChunkName::SWNM) {
        for switch_name_string_number in x.switch_name_string_number {
            ret.push(*switch_name_string_number);
        }
    }

    if let Some(ParsedChunk::MRGN(x)) = map.get(&ChunkName::MRGN) {
        for location in x.locations {
            ret.push(location.name_string_number as u32);
        }
    }

    if let Some(ParsedChunk::MBRF(x)) = map.get(&ChunkName::MBRF) {
        for trigger in x.triggers {
            for action in trigger.actions {
                ret.push(action.string_number);
            }
        }
    }

    if let Some(ParsedChunk::TRIG(x)) = map.get(&ChunkName::TRIG) {
        for trigger in x.triggers {
            for action in trigger.actions {
                if action.string_number > 65535 {
                    println!("{action:?}");
                }
                ret.push(action.string_number);
            }
        }
    }

    for index in ret.iter().filter(|&&x| x != 0) {
        println!("INDEX: {index}");
    }

    Ok(ret.into_iter().filter(|&x| x != 0).collect())
}

// pub(crate) fn guess_encoding_order(
//     map: &HashMap<ChunkName, ParsedChunk>,
// ) -> Result<Vec<&'static encoding_rs::Encoding>, anyhow::Error> {
//     let string_references = get_all_string_references(map)?;

//     let get_string = |index: u32| {
//         //println!("index: {index}");
//         anyhow::ensure!(index > 0);
//         let index = index - 1;
//         if let Some(ParsedChunk::STRx(x)) = map.get(&ChunkName::STRx) {
//             anyhow::ensure!((index as usize) < x.string_offsets.len());
//             anyhow::ensure!((x.string_offsets[index as usize] as usize) < x.strings.len());
//             Ok(parse_null_terminated_bytestring_unsigned(
//                 &x.strings[x.string_offsets[index as usize] as usize..],
//             ))
//         } else if let Some(ParsedChunk::STR(x)) = map.get(&ChunkName::STR) {
//             anyhow::ensure!((index as usize) < x.string_offsets.len());
//             anyhow::ensure!((x.string_offsets[index as usize] as usize) < x.strings.len());
//             Ok(parse_null_terminated_bytestring_unsigned(
//                 &x.strings[x.string_offsets[index as usize] as usize..],
//             ))
//         } else {
//             anyhow::bail!("No STR or STRx section")
//         }
//     };

//     let mut euc_kr_failures: i64 = 0;
//     let mut euc_kr_characters_decoded_successfully: i64 = 0;
//     let mut utf8_failures: i64 = 0;
//     let mut utf8_characters_decoded_successfully: i64 = 0;

//     let mut win1252_total_characters: i64 = 0;
//     let mut win1252_characters_7f_or_above: i64 = 0;

//     for v in string_references {
//         if let Ok(bytestring) = get_string(v) {
//             {
//                 let conversion = encoding_rs::EUC_KR.decode(bytestring);

//                 if conversion.2 {
//                     euc_kr_failures += 1;
//                 } else {
//                     euc_kr_characters_decoded_successfully += conversion
//                         .0
//                         .chars()
//                         .filter(|&c| c >= '가' && c <= '힣')
//                         .count()
//                         as i64;
//                 }
//                 println!("-------------------------------\nEUCKR: {}", conversion.0);
//             }

//             {
//                 let conversion = encoding_rs::UTF_8.decode(bytestring);

//                 if conversion.2 {
//                     utf8_failures += 1;
//                 } else {
//                     utf8_characters_decoded_successfully +=
//                         conversion.0.chars().filter(|&c| c >= '\u{7f}').count() as i64;
//                 }
//                 println!("-------------------------------\nUTF-8: {}", conversion.0);
//             }

//             {
//                 let conversion = encoding_rs::WINDOWS_1252.decode(bytestring);
//                 //println!("STRING: {}", conversion.0);
//                 win1252_total_characters += conversion.0.chars().count() as i64;
//                 win1252_characters_7f_or_above +=
//                     conversion.0.chars().filter(|&c| c >= '\u{7f}').count() as i64;

//                 println!("-------------------------------\nWIN1252: {}", conversion.0);
//             }
//         }
//     }

//     println!("euc_kr_failures: {euc_kr_failures}, euc_kr_characters_decoded_successfully: {euc_kr_characters_decoded_successfully}, utf8_failures: {utf8_failures}, utf8_characters_decoded_successfully: {utf8_characters_decoded_successfully}, win1252_characters_7f_or_above: {win1252_characters_7f_or_above}, win1252_total_characters: {win1252_total_characters}");

//     // let mut encodings = [
//     //     (encoding_rs::WINDOWS_1252, win1252),
//     //     (encoding_rs::EUC_KR, euc_kr),
//     //     (encoding_rs::UTF_8, utf8),
//     //     (encoding_rs::UTF_16LE, utf16),
//     // ];
//     // // sort_by is stable, so becuase wind_1252 is first, it is biased toward that when tied with other ones.
//     // // This is intentional.
//     // encodings.sort_by(|&(_, a), &(_, b)| a.cmp(&b).reverse());

//     // println!(
//     //     "euc_kr: {}, utf8: {}, utf16: {}, win1252: {}",
//     //     euc_kr, utf8, utf16, win1252
//     // );

//     if euc_kr_characters_decoded_successfully == 0 && utf8_characters_decoded_successfully == 0 {
//         Ok(vec![
//             encoding_rs::WINDOWS_1252,
//             encoding_rs::EUC_KR,
//             encoding_rs::UTF_8,
//         ])
//     } else {
//         Ok(vec![
//             encoding_rs::UTF_8,
//             encoding_rs::EUC_KR,
//             encoding_rs::WINDOWS_1252,
//         ])
//     }
// }

pub(crate) fn get_string(
    map: &HashMap<ChunkName, ParsedChunk>,
    // encoding_order: &Vec<&'static encoding_rs::Encoding>,
    index: usize,
) -> Result<String, anyhow::Error> {
    if index == 0 {
        return Ok("Zero SPRP Index provided".to_owned());
    }

    let index = index - 1;

    let bytes = if let Some(ParsedChunk::STRx(x)) = map.get(&ChunkName::STRx) {
        if index as usize >= x.string_offsets.len() {
            return Ok("Out of bounds access".to_owned());
        }
        if x.string_offsets[index as usize] as usize >= x.strings.len() {
            return Ok("Out of bounds access".to_owned());
        }
        parse_null_terminated_bytestring_unsigned(
            &x.strings[x.string_offsets[index as usize] as usize..],
        )
    } else if let Some(ParsedChunk::STR(x)) = map.get(&ChunkName::STR) {
        if index as usize >= x.string_offsets.len() {
            return Ok("Out of bounds access".to_owned());
        }
        if x.string_offsets[index as usize] as usize >= x.strings.len() {
            return Ok("Out of bounds access".to_owned());
        }
        parse_null_terminated_bytestring_unsigned(
            &x.strings[x.string_offsets[index as usize] as usize..],
        )
    } else {
        anyhow::bail!("No STR or STRx section")
    };

    if bytes.len() == 0 {
        return Ok("".to_owned());
    }

    let mut euc_kr_failed = false;
    let mut euc_kr_characters_decoded_successfully: i64 = 0;
    let mut euc_kr_characters_len: usize = 0;
    let mut utf8_failed = false;
    let mut utf8_characters_decoded_successfully: i64 = 0;
    let mut utf8_characters_len: usize = 0;

    let mut win1252_total_characters: i64 = 0;
    let mut win1252_characters_7f_or_above: i64 = 0;

    {
        let conversion = encoding_rs::EUC_KR.decode(bytes);
        println!("EUC_KR: {}", conversion.0);
        euc_kr_characters_len = conversion.0.chars().count();

        if conversion.2 {
            euc_kr_failed = true;
        } else {
            euc_kr_characters_decoded_successfully += conversion
                .0
                .chars()
                .filter(|&c| c >= '가' && c <= '힣')
                .count() as i64;
        }
    }

    {
        let conversion = encoding_rs::UTF_8.decode(bytes);
        println!("UTF-8: {}", conversion.0);
        utf8_characters_len = conversion.0.chars().count();

        if conversion.2 {
            utf8_failed = true;
        } else {
            utf8_characters_decoded_successfully +=
                conversion.0.chars().filter(|&c| c >= '\u{7f}').count() as i64;
        }
    }

    {
        let conversion = encoding_rs::WINDOWS_1252.decode(bytes);
        println!("WIN1252: {}", conversion.0);
        win1252_total_characters += conversion.0.chars().count() as i64;
        win1252_characters_7f_or_above +=
            conversion.0.chars().filter(|&c| c >= '\u{7f}').count() as i64;
    }

    let uchardet_guessed_encoding = unsafe {
        let handle = uchardet_bindings::uchardet_new();
        scopeguard::defer! {
            uchardet_bindings::uchardet_delete(handle);
        }

        // filter out color codes because it might be breaking uchardet.
        let vec: Vec<_> = bytes.iter().filter(|&&x| x >= 0x20).map(|x| *x).collect();

        if uchardet_bindings::uchardet_handle_data(
            handle,
            vec.as_ptr() as *const i8,
            vec.len() as uchardet_bindings::size_t,
        ) != 0
        {
            panic!();
        }

        uchardet_bindings::uchardet_data_end(handle);

        let charset = CStr::from_ptr(uchardet_bindings::uchardet_get_charset(handle))
            .to_str()?
            .to_string();

        match charset.as_str() {
            "UTF-8" => anyhow::Ok(Some(encoding_rs::UTF_8)),
            "UHC" => anyhow::Ok(Some(encoding_rs::EUC_KR)),
            "ASCII" | "ISO-8859-7" | "ISO-8859-2" | "WINDOWS-1252" | "WINDOWS-1250"
            | "MAC-CENTRALEUROPE" | "WINDOWS-1257" | "ISO-8859-10" | "ISO-8859-1" => {
                anyhow::Ok(Some(encoding_rs::WINDOWS_1252))
            }
            "" | "IBM852" | "VISCII" | "ISO-8859-13" | "ISO-8859-9" | "ISO-8859-3" => {
                anyhow::Ok(None)
            }
            _ => {
                anyhow::Ok(None)
                //panic!("{charset}")
            }
        }
    }?;

    let (compact_enc_det_encoding_guess, compact_enc_det_encoding_guess_is_reliable) = unsafe {
        // filter out color codes because it might be breaking uchardet.
        let vec: Vec<_> = bytes.iter().filter(|&&x| x >= 0x20).map(|x| *x).collect();

        let mut bytes_consumed = 0;
        let mut is_reliable = false;

        let encoding = compact_enc_det_bindings::CompactEncDet_DetectEncoding(
            vec.as_ptr() as *const i8,
            vec.len() as i32,
            0 as *const i8,
            0 as *const i8,
            0 as *const i8,
            compact_enc_det_bindings::Encoding_UNKNOWN_ENCODING as i32,
            compact_enc_det_bindings::Language_UNKNOWN_LANGUAGE,
            compact_enc_det_bindings::CompactEncDet_TextCorpusType_WEB_CORPUS,
            true,
            &mut bytes_consumed,
            &mut is_reliable,
        );

        println!("is_reliable: {is_reliable}, bytes_consumed: {bytes_consumed}");

        (
            match encoding {
                compact_enc_det_bindings::Encoding_UTF8 => Some(encoding_rs::UTF_8),
                compact_enc_det_bindings::Encoding_KOREAN_EUC_KR => Some(encoding_rs::EUC_KR),
                compact_enc_det_bindings::Encoding_ISO_8859_1
                | compact_enc_det_bindings::Encoding_MSFT_CP1252
                | compact_enc_det_bindings::Encoding_ASCII_7BIT => Some(encoding_rs::WINDOWS_1252),
                compact_enc_det_bindings::Encoding_CHINESE_GB
                | compact_enc_det_bindings::Encoding_CHINESE_BIG5
                | compact_enc_det_bindings::Encoding_JAPANESE_EUC_JP => None,
                _ => {
                    None
                    //panic!("encoding panic'd on: {encoding}")
                }
            },
            is_reliable,
        )
    };

    let mut encoding_map = std::collections::HashMap::new();

    encoding_map.insert(encoding_rs::UTF_8, 0.0);
    encoding_map.insert(encoding_rs::EUC_KR, 0.0);
    encoding_map.insert(encoding_rs::WINDOWS_1252, 0.0);

    if let Some(uchardet_guessed_encoding) = uchardet_guessed_encoding {
        *encoding_map.get_mut(uchardet_guessed_encoding).unwrap() += 0.7;
    }

    if let Some(compact_enc_det_encoding_guess) = compact_enc_det_encoding_guess {
        *encoding_map
            .get_mut(compact_enc_det_encoding_guess)
            .unwrap() += if compact_enc_det_encoding_guess_is_reliable {
            0.7
        } else {
            0.2
        };
    }

    *encoding_map.get_mut(encoding_rs::EUC_KR).unwrap() +=
        (euc_kr_characters_decoded_successfully as f64) / (euc_kr_characters_len as f64);

    *encoding_map.get_mut(encoding_rs::WINDOWS_1252).unwrap() -=
        (win1252_characters_7f_or_above as f64) / (bytes.len() as f64);

    *encoding_map.get_mut(encoding_rs::UTF_8).unwrap() +=
        (utf8_characters_decoded_successfully as f64) / (utf8_characters_len as f64);

    println!(
        "\n\
        euc_kr_failed: {euc_kr_failed}, \
        euc_kr_characters_decoded_successfully: {euc_kr_characters_decoded_successfully}, \
        euc_kr_characters_len: {euc_kr_characters_len}, \
        utf8_failed: {utf8_failed}, \
        utf8_characters_decoded_successfully: {utf8_characters_decoded_successfully}, \
        utf8_characters_len: {utf8_characters_len}, \
        win1252_characters_7f_or_above: {win1252_characters_7f_or_above}, \
        win1252_total_characters: {win1252_total_characters}, \
        \n\
        uchardet_guess: {uchardet_guessed_encoding:?}, \
        \n\
        compact_enc_det_encoding_guess: {compact_enc_det_encoding_guess:?}, \
        \n\
        encoding_map: {encoding_map:?}, str: {}\n\
        ------------------------------------------------------------------------------------",
        encoding_rs::WINDOWS_1252.decode(bytes).0
    );

    if euc_kr_failed {
        encoding_map.remove(encoding_rs::EUC_KR);
    }

    if utf8_failed {
        encoding_map.remove(encoding_rs::UTF_8);
    }

    let mut encodings: Vec<_> = encoding_map.drain().collect();

    encodings.sort_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap().reverse());

    Ok(encodings[0].0.decode(bytes).0.to_string())

    // TODO: Implement voting idea.
    // decoding failure = complete veto.
    // udetchar can vote with some weight.
    // number of chars successfully decoded specifically in that range can also vote as some weight.
    // Other strings in the map can also vote with some weight but not sure how to implement that exactly.
    // Table of exceptions can also make a vote.
}
