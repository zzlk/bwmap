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

use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::chk2::{
    ChkColr::{parse_colr, ChkColr},
    ChkCrgb::{parse_crgb, ChkCrgb},
    ChkForc::{parse_forc, ChkForc},
    ChkStr::{parse_str, ChkStr},
    ChkTrig::{parse_trig, ChkTrig},
    ChkUnis::{parse_unis, ChkUnis},
    ChkVcod::{parse_vcod, ChkVcod},
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
    TRIG(ChkTrig<'a>),
    CRGB(ChkCrgb<'a>),
    FORC(ChkForc<'a>),
    STR(ChkStr<'a>),
    UNIS(ChkUnis<'a>),
    VCOD(ChkVcod<'a>),
}

pub fn parse_merged_chunks(
    chunks: &HashMap<ChunkName, MergedChunk>,
) -> Result<HashMap<ChunkName, ParsedChunk>, anyhow::Error> {
    let mut map = HashMap::new();
    for (chunk_name, chunk) in chunks.into_iter() {
        match chunk_name {
            ChunkName::CRGB => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::CRGB(parse_crgb(chunk.data.as_slice())?),
                );
            }
            ChunkName::TRIG => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::TRIG(parse_trig(chunk.data.as_slice())?),
                );
            }
            ChunkName::COLR => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::COLR(parse_colr(chunk.data.as_slice())?),
                );
            }
            ChunkName::FORC => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::FORC(parse_forc(chunk.data.as_slice())?),
                );
            }
            ChunkName::STR => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::STR(parse_str(chunk.data.as_slice())?),
                );
            }
            ChunkName::UNIS => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::UNIS(parse_unis(chunk.data.as_slice())?),
                );
            }
            ChunkName::VCOD => {
                map.insert(
                    chunk_name.clone(),
                    ParsedChunk::VCOD(parse_vcod(chunk.data.as_slice())?),
                );
            }
            _ => {}
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

        // println!(
        //     "euc_kr: {}, utf8: {}, utf16: {}, win1252: {}",
        //     euc_kr, utf8, utf16, win1252
        // );

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
