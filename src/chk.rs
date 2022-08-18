use anyhow::Result;

#[cfg(feature = "full")]
use std::ffi::CStr;

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

#[cfg(feature = "full")]
use crate::util::parse_null_terminated_bytestring_unsigned;

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
        chk_trig::{parse_trig, ChkTrig},
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
    chunk_name::{get_chunk_update_type, parse_chunk_name, ChunkName, ChunkNameUpdateType},
};
use std::str;
use tracing::instrument;

#[derive(Debug, Serialize, Deserialize)]
pub struct RawChunk<'a> {
    pub name: ChunkName,
    pub size: i32,
    pub offset: usize,
    #[serde(skip_serializing)]
    pub data: &'a [u8],
}

#[instrument(level = "trace", skip_all)]
pub fn parse_chk(chk: &[u8]) -> Vec<RawChunk> {
    let mut offset = 0;
    let mut ret = Vec::new();

    loop {
        let start_offset = offset;
        if offset + 4 >= chk.len() {
            break;
        }
        let name = parse_chunk_name(&chk[offset..offset + 4]);

        offset += 4;
        if offset + 4 >= chk.len() {
            break;
        }
        let size: i32 = crate::util::parse_slice(&chk[offset..offset + 4]);
        let size = size as i64;

        offset += 4;
        if (offset as i64 + size) > chk.len() as i64 {
            break;
        }

        if (offset as i64 + size) < 0 {
            break;
        }

        if size >= 0 {
            let data = &chk[offset..(offset as i64 + size) as usize];

            ret.push(RawChunk {
                name,
                size: size as i32,
                offset: start_offset,
                data,
            });
        }

        offset = (offset as i64 + size).try_into().unwrap();
    }

    ret
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MergedChunk {
    pub name: ChunkName,
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}

#[instrument(level = "trace", skip_all)]
pub fn merge_raw_chunks(chunks: &[RawChunk]) -> HashMap<ChunkName, MergedChunk> {
    let mut map = HashMap::new();

    for v in chunks {
        match get_chunk_update_type(&v.name) {
            ChunkNameUpdateType::FullOverwrite => {
                let merged = MergedChunk {
                    name: v.name.clone(),
                    data: v.data.to_owned(),
                };

                map.insert(merged.name.clone(), merged);
            }

            ChunkNameUpdateType::PartialOverwrite => {
                if let Some(c) = map.get(&v.name) {
                    let mut merged = MergedChunk {
                        name: c.name.clone(),
                        data: Vec::new(),
                    };

                    merged.data.extend(v.data);

                    if c.data.len() > v.data.len() {
                        merged.data.extend(&c.data[v.data.len()..]);
                    }

                    map.insert(merged.name.clone(), merged);
                } else {
                    let merged = MergedChunk {
                        name: v.name.clone(),
                        data: v.data.to_owned(),
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

                    merged.data.extend_from_slice(v.data);
                    merged.data.extend_from_slice(&c.data.as_slice());

                    map.insert(merged.name.clone(), merged);
                } else {
                    let merged = MergedChunk {
                        name: v.name.clone(),
                        data: v.data.to_owned(),
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
    DD2(ChkDd2),
    DIM(ChkDim<'a>),
    ERA(ChkEra<'a>),
    FORC(ChkForc<'a>),
    IOWN(ChkIown<'a>),
    ISOM(ChkIsom),
    IVE2(ChkIve2<'a>),
    IVER(ChkIver<'a>),
    MASK(ChkMask<'a>),
    MBRF(ChkMbrf),
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
    THG2(ChkThg2),
    TILE(ChkTile),
    TRIG(ChkTrig),
    TYPE(ChkType<'a>),
    UNIS(ChkUnis<'a>),
    UNIT(ChkUnit),
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

#[instrument(level = "trace", skip_all)]
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

#[instrument(level = "trace", skip_all)]
pub(crate) fn verify_is_valid_chk(chk: &[u8]) -> bool {
    let raw_chunks = parse_chk(&chk);
    let merged_chunks = merge_raw_chunks(&raw_chunks);
    let parsed_chunks = parse_merged_chunks(&merged_chunks).unwrap();

    parsed_chunks.get(&ChunkName::VCOD).is_some()
}

pub fn get_all_string_references(
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
        for trigger in &x.triggers {
            for action in trigger.actions {
                ret.push(action.string_number);
            }
        }
    }

    if let Some(ParsedChunk::TRIG(x)) = map.get(&ChunkName::TRIG) {
        for trigger in &x.triggers {
            for action in trigger.actions {
                if action.string_number > 65535 {
                    println!("{action:?}");
                }
                ret.push(action.string_number);
            }
        }
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

#[cfg(feature = "full")]
#[instrument(level = "trace", skip(map))]
pub(crate) fn get_location_name(
    map: &HashMap<ChunkName, ParsedChunk>,
    index: usize,
) -> Result<String, anyhow::Error> {
    if index == 0 {
        return Ok("No Location".to_owned());
    }

    if let Some(ParsedChunk::MRGN(mrgn)) = map.get(&ChunkName::MRGN) {
        if mrgn.locations.len() <= index {
            return Ok("Location index out of bounds".to_owned());
        }

        get_string(map, mrgn.locations[index - 1].name_string_number as usize)
    } else {
        Ok("No Location".to_owned())
    }
}

#[cfg(feature = "full")]
#[instrument(level = "trace", skip(map))]
pub fn get_string(
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
            return Ok(format!(
                "Out of bounds access STRx offset table. index: {index}, string_offsets.len(): {}",
                x.string_offsets.len()
            ));
        }
        if x.string_offsets[index as usize] as usize >= x.strings.len() {
            return Ok(format!(
                "Out of bounds access STRx. index: {index}, string_offsets.len(): {}, offset: {}, strings.len(): {}",
                x.string_offsets.len(),
                x.string_offsets[index as usize],
                x.strings.len()
            ));
        }
        parse_null_terminated_bytestring_unsigned(
            &x.strings[x.string_offsets[index as usize] as usize..],
        )
    } else if let Some(ParsedChunk::STR(x)) = map.get(&ChunkName::STR) {
        if index as usize >= x.string_offsets.len() {
            return Ok(format!(
                "Out of bounds access STR offset table. index: {index}, string_offsets.len(): {}",
                x.string_offsets.len()
            ));
        }
        if x.string_offsets[index as usize] as usize >= x.strings.len() {
            return Ok(format!(
                "Out of bounds access STR. index: {index}, string_offsets.len(): {}, offset: {}, strings.len(): {}",
                x.string_offsets.len(),
                x.string_offsets[index as usize],
                x.strings.len()
            ));
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
    let euc_kr_characters_len: usize;
    let mut utf8_failed = false;
    let mut utf8_characters_decoded_successfully: i64 = 0;
    let utf8_characters_len: usize;

    let mut win1252_characters_7f_or_above: i64 = 0;

    {
        let conversion = encoding_rs::EUC_KR.decode(bytes);
        // println!("EUC_KR: {}", conversion.0);
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
        // println!("UTF-8: {}", conversion.0);
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
        // println!("WIN1252: {}", conversion.0);
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

        // println!("is_reliable: {is_reliable}, bytes_consumed: {bytes_consumed}");

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

    // println!(
    //     "\n\
    //     euc_kr_failed: {euc_kr_failed}, \
    //     euc_kr_characters_decoded_successfully: {euc_kr_characters_decoded_successfully}, \
    //     euc_kr_characters_len: {euc_kr_characters_len}, \
    //     utf8_failed: {utf8_failed}, \
    //     utf8_characters_decoded_successfully: {utf8_characters_decoded_successfully}, \
    //     utf8_characters_len: {utf8_characters_len}, \
    //     win1252_characters_7f_or_above: {win1252_characters_7f_or_above}, \
    //     win1252_total_characters: {win1252_total_characters}, \
    //     \n\
    //     uchardet_guess: {uchardet_guessed_encoding:?}, \
    //     \n\
    //     compact_enc_det_encoding_guess: {compact_enc_det_encoding_guess:?}, \
    //     \n\
    //     encoding_map: {encoding_map:?}, str: {}\n\
    //     ------------------------------------------------------------------------------------",
    //     encoding_rs::WINDOWS_1252.decode(bytes).0
    // );

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
