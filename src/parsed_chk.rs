use crate::{
    chk2::{
        chk_colr::{parse_colr2, ChkColr},
        chk_crgb::{parse_crgb2, ChkCrgb},
        chk_dd2::{parse_dd22, ChkDd2},
        chk_dim::{parse_dim2, ChkDim},
        chk_era::{parse_era2, ChkEra},
        chk_forc::{parse_forc2, ChkForc},
        chk_iown::{parse_iown2, ChkIown},
        chk_isom::{parse_isom2, ChkIsom},
        chk_ive2::{parse_ive22, ChkIve2},
        chk_iver::{parse_iver2, ChkIver},
        chk_mask::{parse_mask2, ChkMask},
        chk_mbrf::{parse_mbrf2, ChkMbrf},
        chk_mrgn::{parse_mrgn2, ChkMrgn},
        chk_mtxm::{parse_mtxm2, ChkMtxm},
        chk_ownr::{parse_ownr2, ChkOwnr},
        chk_ptec::{parse_ptec2, ChkPtec},
        chk_ptex::{parse_ptex2, ChkPtex},
        chk_puni::{parse_puni2, ChkPuni},
        chk_pupx::{parse_pupx2, ChkPupx},
        chk_side::{parse_side2, ChkSide},
        chk_sprp::{parse_sprp2, ChkSprp},
        chk_str::{parse_str2, ChkStr2},
        chk_strx::{parse_strx2, ChkStrx2},
        chk_swnm::{parse_swnm2, ChkSwnm},
        chk_tecs::{parse_tecs2, ChkTecs},
        chk_tecx::{parse_tecx2, ChkTecx},
        chk_thg2::{parse_thg22, ChkThg2},
        chk_tile::{parse_tile2, ChkTile},
        chk_trig::{parse_trig2, ChkTrig},
        chk_type::{parse_type2, ChkType},
        chk_unis::{parse_unis2, ChkUnis},
        chk_unit::{parse_unit2, ChkUnit},
        chk_unix::{parse_unix2, ChkUnix},
        chk_upgr::{parse_upgr2, ChkUpgr},
        chk_upgs::{parse_upgs2, ChkUpgs},
        chk_upgx::{parse_upgx2, ChkUpgx},
        chk_uprp::{parse_uprp2, ChkUprp},
        chk_upus::{parse_upus2, ChkUpus},
        chk_vcod::{parse_vcod2, ChkVcod},
        chk_ver::{parse_ver2, ChkVer},
        chk_wav::{parse_wav2, ChkWav},
    },
    riff::{parse_riff, validate_and_group_riff_chunks},
    ChunkName,
};
use anyhow::Result;
use tracing::instrument;

pub struct ParsedChk<'a> {
    pub colr: Result<ChkColr<'a>>,
    pub crgb: Result<ChkCrgb<'a>>,
    pub dd2: Result<ChkDd2>,
    pub dim: Result<ChkDim<'a>>,
    pub era: Result<ChkEra<'a>>,
    pub forc: Result<ChkForc<'a>>,
    pub iown: Result<ChkIown<'a>>,
    pub isom: Result<ChkIsom>,
    pub ive2: Result<ChkIve2<'a>>,
    pub iver: Result<ChkIver<'a>>,
    pub mask: Result<ChkMask<'a>>,
    pub mbrf: Result<ChkMbrf>,
    pub mrgn: Result<ChkMrgn<'a>>,
    pub mtxm: Result<ChkMtxm>,
    pub ownr: Result<ChkOwnr<'a>>,
    pub ptec: Result<ChkPtec<'a>>,
    pub ptex: Result<ChkPtex<'a>>,
    pub puni: Result<ChkPuni<'a>>,
    pub pupx: Result<ChkPupx<'a>>,
    pub side: Result<ChkSide<'a>>,
    pub sprp: Result<ChkSprp<'a>>,
    pub str: Result<ChkStr2>,
    pub strx: Result<ChkStrx2>,
    pub swnm: Result<ChkSwnm<'a>>,
    pub tecs: Result<ChkTecs<'a>>,
    pub tecx: Result<ChkTecx<'a>>,
    pub thg2: Result<ChkThg2>,
    pub tile: Result<ChkTile>,
    pub trig: Result<ChkTrig>,
    pub type_: Result<ChkType<'a>>,
    pub unis: Result<ChkUnis<'a>>,
    pub unit: Result<ChkUnit>,
    pub unix: Result<ChkUnix<'a>>,
    pub upgr: Result<ChkUpgr<'a>>,
    pub upgs: Result<ChkUpgs<'a>>,
    pub upgx: Result<ChkUpgx<'a>>,
    pub uprp: Result<ChkUprp<'a>>,
    pub upus: Result<ChkUpus<'a>>,
    pub vcod: Result<ChkVcod<'a>>,
    pub ver: Result<ChkVer<'a>>,
    pub wav: Result<ChkWav<'a>>,
}

#[instrument(level = "trace", skip_all)]
pub fn parse_chk_full<'a>(chk: &'a [u8]) -> ParsedChk<'a> {
    let riff_chunks = parse_riff(chk);
    let riff_chunks = validate_and_group_riff_chunks(riff_chunks.as_slice());

    #[rustfmt::skip]
    let ret = ParsedChk {
        colr: riff_chunks.get(&ChunkName::COLR).map(|x| parse_colr2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        crgb: riff_chunks.get(&ChunkName::CRGB).map(|x| parse_crgb2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        dd2:  riff_chunks.get(&ChunkName::DD2 ).map(|x| parse_dd22 (x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        dim:  riff_chunks.get(&ChunkName::DIM ).map(|x| parse_dim2 (x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        era:  riff_chunks.get(&ChunkName::ERA ).map(|x| parse_era2 (x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        forc: riff_chunks.get(&ChunkName::FORC).map(|x| parse_forc2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        iown: riff_chunks.get(&ChunkName::IOWN).map(|x| parse_iown2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        isom: riff_chunks.get(&ChunkName::ISOM).map(|x| parse_isom2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        ive2: riff_chunks.get(&ChunkName::IVE2).map(|x| parse_ive22(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        iver: riff_chunks.get(&ChunkName::IVER).map(|x| parse_iver2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        mask: riff_chunks.get(&ChunkName::MASK).map(|x| parse_mask2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        mbrf: riff_chunks.get(&ChunkName::MBRF).map(|x| parse_mbrf2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        mrgn: riff_chunks.get(&ChunkName::MRGN).map(|x| parse_mrgn2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        mtxm: riff_chunks.get(&ChunkName::MTXM).map(|x| parse_mtxm2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        ownr: riff_chunks.get(&ChunkName::OWNR).map(|x| parse_ownr2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        ptec: riff_chunks.get(&ChunkName::PTEC).map(|x| parse_ptec2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        ptex: riff_chunks.get(&ChunkName::PTEx).map(|x| parse_ptex2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        puni: riff_chunks.get(&ChunkName::PUNI).map(|x| parse_puni2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        pupx: riff_chunks.get(&ChunkName::PUPx).map(|x| parse_pupx2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        side: riff_chunks.get(&ChunkName::SIDE).map(|x| parse_side2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        sprp: riff_chunks.get(&ChunkName::SPRP).map(|x| parse_sprp2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        str:  riff_chunks.get(&ChunkName::STR ).map(|x| parse_str2 (x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        strx: riff_chunks.get(&ChunkName::STRx).map(|x| parse_strx2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        swnm: riff_chunks.get(&ChunkName::SWNM).map(|x| parse_swnm2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        tecs: riff_chunks.get(&ChunkName::TECS).map(|x| parse_tecs2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        tecx: riff_chunks.get(&ChunkName::TECx).map(|x| parse_tecx2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        thg2: riff_chunks.get(&ChunkName::THG2).map(|x| parse_thg22(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        tile: riff_chunks.get(&ChunkName::TILE).map(|x| parse_tile2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        trig: riff_chunks.get(&ChunkName::TRIG).map(|x| parse_trig2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        type_:riff_chunks.get(&ChunkName::TYPE).map(|x| parse_type2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        unis: riff_chunks.get(&ChunkName::UNIS).map(|x| parse_unis2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        unit: riff_chunks.get(&ChunkName::UNIT).map(|x| parse_unit2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        unix: riff_chunks.get(&ChunkName::UNIx).map(|x| parse_unix2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        upgr: riff_chunks.get(&ChunkName::UPGR).map(|x| parse_upgr2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        upgs: riff_chunks.get(&ChunkName::UPGS).map(|x| parse_upgs2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        upgx: riff_chunks.get(&ChunkName::UPGx).map(|x| parse_upgx2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        uprp: riff_chunks.get(&ChunkName::UPRP).map(|x| parse_uprp2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        upus: riff_chunks.get(&ChunkName::UPUS).map(|x| parse_upus2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        vcod: riff_chunks.get(&ChunkName::VCOD).map(|x| parse_vcod2(x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        ver:  riff_chunks.get(&ChunkName::VER ).map(|x| parse_ver2 (x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
        wav:  riff_chunks.get(&ChunkName::WAV ).map(|x| parse_wav2 (x.as_slice())).unwrap_or(Err(anyhow::anyhow!("Not Found"))),
    };

    ret
}

#[cfg(test)]
mod test {

    use crate::{parse_chk_full, test::get_all_test_maps};

    #[test]
    fn can_parse_all_maps() {
        for dir_entry in get_all_test_maps() {
            println!("{}", dir_entry.path().to_string_lossy().to_string());
            let chk_data =
                crate::get_chk_from_mpq_filename(dir_entry.path().to_string_lossy().to_string())
                    .unwrap();

            let parsed_chk = parse_chk_full(chk_data.as_slice());

            assert!(parsed_chk.ver.is_ok());
            assert!(parsed_chk.vcod.is_ok());
            assert!(parsed_chk.ownr.is_ok());
            assert!(parsed_chk.era.is_ok());
            assert!(parsed_chk.dim.is_ok());
            assert!(parsed_chk.sprp.is_ok());
            assert!(parsed_chk.forc.is_ok());
            assert!(parsed_chk.side.is_ok());
            assert!(parsed_chk.mtxm.is_ok());
            // assert!(parsed_chk.unit.is_ok());
            // assert!(parsed_chk.mask.is_ok());
            // assert!(parsed_chk.mrgn.is_ok());
        }
    }

    #[test]
    fn specific_test_rise_of_empires() {
        let filename = format!(
            "{}/test_vectors/[6]Rise of Empires v6.07e.scx",
            env!("CARGO_MANIFEST_DIR")
        );

        let chk_data = crate::get_chk_from_mpq_filename(filename).unwrap();
        let parsed_chk = parse_chk_full(chk_data.as_slice());

        assert!(parsed_chk.sprp.is_ok());
    }

    #[test]
    fn specific_test_achievementpyth() {
        let filename = format!(
            "{}/test_vectors/AchievementPyth.scx",
            env!("CARGO_MANIFEST_DIR")
        );

        let chk_data = crate::get_chk_from_mpq_filename(filename).unwrap();
        let parsed_chk = parse_chk_full(chk_data.as_slice());

        assert!(parsed_chk.forc.is_ok());
    }
}
