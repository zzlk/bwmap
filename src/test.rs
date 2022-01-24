use crate::{
    chk::{merge_raw_chunks, parse_merged_chunks, ChunkName, ParsedChunk},
    parse_chk,
};
use rayon::prelude::*;

fn for_all_test_maps<F: Fn(walkdir::DirEntry) + Sync>(func: F) {
    let processed_maps =
        walkdir::WalkDir::new(format!("{}/test_vectors", env!("CARGO_MANIFEST_DIR")))
            .into_iter()
            //.par_bridge()
            .filter_map(Result::ok)
            .filter(
                |e| match e.file_name().to_string_lossy().to_string().as_str() {
                    "[EUD]컴디 파이널.scx" => false,
                    "마인의 폭피 1.scm" => false,
                    _ => {
                        !e.file_type().is_dir()
                            && (e.file_name().to_string_lossy().ends_with(".scx")
                                || e.file_name().to_string_lossy().ends_with(".scm"))
                    }
                },
            )
            .map(|e| {
                func(e);
            })
            .count();

    assert_eq!(processed_maps, 176);
}

#[test]
fn test_constrain_encoding_detection_algorithm() {
    let f = |s: &str| {
        let mut root = env!("CARGO_MANIFEST_DIR").to_owned();
        root.push_str(format!("/test_vectors/{s}").as_str());

        let mpq = std::fs::read(std::path::Path::new(root.as_str())).unwrap();
        let chk = crate::get_chk_from_mpq_in_memory(mpq.as_slice()).unwrap();
        let raw_chunks = crate::parse_chk(chk.as_slice());
        let merged_chunks = crate::merge_rawchunks(raw_chunks.as_slice());
        let chk_dump = crate::get_parsed_chk(&merged_chunks).unwrap();

        assert!(chk_dump.mtxm.len() > 0);
        assert!(chk_dump.map_ver > 0);

        chk_dump.scenario_name
    };

    let test_vectors = [
        ("폭탄피하기[뿌요뿌요].scx", "폭피[뿌요뿌요]"),
        ("»JoNiS»BoUnD».scx", "JØNÎ$  ßøûñÐ(beta)"),
        (
            "D u s t BouND P .scx",
            "\u{0013}\u{0002}Ðúst BóüÑÐ\u{0012}\u{0006}Dust BouND      .",
        ),
        ("poop bound.scx", "\u{4}Poo\u{6}p \u{3}Boun\u{6}d"),
        (
            "Crescent Bound.scx",
            "\u{3}C\u{4}r\u{6}escent \u{7}B\u{6}ound",
        ),
        (
            "[[[[[도라에몽의성큰(빨�__짱 (1).scx",
            "\u{4}도라에몽\u{4} 의\u{4} \u{4}S\u{5}unken ",
        ),
        (
            "Ecstasy EVF.scx",
            "\u{1}마린키우기 \u{7}E\u{6}cstasy \u{3}EVF",
        ),
    ];

    test_vectors
        .into_par_iter()
        .for_each(|(a, b)| assert_eq!(f(a), b));
}

#[test]
fn test_get_chk_from_mpq_filename() {
    crate::test::for_all_test_maps(|e| {
        assert!(
            crate::get_chk_from_mpq_filename(e.path().to_string_lossy().to_string())
                .unwrap()
                .len()
                > 0
        );
    });
}

#[test]
fn test_get_chk_from_mpq_in_memory() {
    crate::test::for_all_test_maps(|e| {
        assert_eq!(
            crate::get_chk_from_mpq_in_memory(std::fs::read(e.path()).unwrap().as_slice()).unwrap(),
            crate::get_chk_from_mpq_filename(e.path().to_string_lossy().to_string()).unwrap()
        );
    });
}

#[test]
fn test_parse_merged_chunks() {
    crate::test::for_all_test_maps(|e| {
        let chk_data =
            crate::get_chk_from_mpq_filename(e.path().to_string_lossy().to_string()).unwrap();

        let raw_chunks = parse_chk(&chk_data);
        let merged_chunks = merge_raw_chunks(&raw_chunks);
        let parsed_chunks = parse_merged_chunks(&merged_chunks);

        assert!(
            parsed_chunks.is_ok(),
            "filename: {}, {:?}",
            e.file_name().to_string_lossy(),
            parsed_chunks
        );

        let parsed_chunks = parsed_chunks.unwrap();

        // println!("{:?}", parsed_chunks);

        assert!(
            parsed_chunks.get(&ChunkName::VCOD).is_some(),
            "filename: {}, {:?}",
            e.file_name().to_string_lossy(),
            parsed_chunks
        );
    });
}

#[test]
fn test_specific_map_files_for_known_values() {
    let path = format!(
        "{}/test_vectors/custom_cases/base.scm",
        env!("CARGO_MANIFEST_DIR")
    );

    let chk = crate::get_chk_from_mpq_filename(path).unwrap();
    let raw_chunks = crate::parse_chk(chk.as_slice());
    let merged_chunks = crate::merge_raw_chunks(raw_chunks.as_slice());

    assert_eq!(merged_chunks.keys().count(), 31);

    assert!(merged_chunks.get(&ChunkName::VER).is_some());
    assert!(merged_chunks.get(&ChunkName::IVER).is_some());
    assert!(merged_chunks.get(&ChunkName::VCOD).is_some());
    assert!(merged_chunks.get(&ChunkName::IOWN).is_some());
    assert!(merged_chunks.get(&ChunkName::OWNR).is_some());
    assert!(merged_chunks.get(&ChunkName::SIDE).is_some());
    assert!(merged_chunks.get(&ChunkName::ERA).is_some());
    assert!(merged_chunks.get(&ChunkName::DIM).is_some());
    assert!(merged_chunks.get(&ChunkName::MTXM).is_some());
    assert!(merged_chunks.get(&ChunkName::TILE).is_some());
    assert!(merged_chunks.get(&ChunkName::ISOM).is_some());
    assert!(merged_chunks.get(&ChunkName::UNIT).is_some());
    assert!(merged_chunks.get(&ChunkName::PUNI).is_some());
    assert!(merged_chunks.get(&ChunkName::UNIS).is_some());
    assert!(merged_chunks.get(&ChunkName::UPGR).is_some());
    assert!(merged_chunks.get(&ChunkName::UPGS).is_some());
    assert!(merged_chunks.get(&ChunkName::DD2).is_some());
    assert!(merged_chunks.get(&ChunkName::THG2).is_some());
    assert!(merged_chunks.get(&ChunkName::MASK).is_some());
    assert!(merged_chunks.get(&ChunkName::MRGN).is_some());
    assert!(merged_chunks.get(&ChunkName::STR).is_some());
    assert!(merged_chunks.get(&ChunkName::SPRP).is_some());
    assert!(merged_chunks.get(&ChunkName::FORC).is_some());
    assert!(merged_chunks.get(&ChunkName::WAV).is_some());
    assert!(merged_chunks.get(&ChunkName::PTEC).is_some());
    assert!(merged_chunks.get(&ChunkName::TECS).is_some());
    assert!(merged_chunks.get(&ChunkName::MBRF).is_some());
    assert!(merged_chunks.get(&ChunkName::TRIG).is_some());
    assert!(merged_chunks.get(&ChunkName::UPRP).is_some());
    assert!(merged_chunks.get(&ChunkName::UPUS).is_some());
    assert!(merged_chunks.get(&ChunkName::SWNM).is_some());

    let parsed_chunks = crate::parse_merged_chunks(&merged_chunks).unwrap();

    assert_eq!(parsed_chunks.keys().count(), 31);

    for (cn, pc) in parsed_chunks.iter() {
        match pc {
            ParsedChunk::VER(_) => assert_eq!(cn, &ChunkName::VER),
            ParsedChunk::IVER(_) => assert_eq!(cn, &ChunkName::IVER),
            ParsedChunk::VCOD(_) => assert_eq!(cn, &ChunkName::VCOD),
            ParsedChunk::IOWN(_) => assert_eq!(cn, &ChunkName::IOWN),
            ParsedChunk::OWNR(_) => assert_eq!(cn, &ChunkName::OWNR),
            ParsedChunk::SIDE(_) => assert_eq!(cn, &ChunkName::SIDE),
            ParsedChunk::ERA(_) => assert_eq!(cn, &ChunkName::ERA),
            ParsedChunk::DIM(_) => assert_eq!(cn, &ChunkName::DIM),
            ParsedChunk::MTXM(_) => assert_eq!(cn, &ChunkName::MTXM),
            ParsedChunk::TILE(_) => assert_eq!(cn, &ChunkName::TILE),
            ParsedChunk::ISOM(_) => assert_eq!(cn, &ChunkName::ISOM),
            ParsedChunk::UNIT(_) => assert_eq!(cn, &ChunkName::UNIT),
            ParsedChunk::PUNI(_) => assert_eq!(cn, &ChunkName::PUNI),
            ParsedChunk::UNIS(_) => assert_eq!(cn, &ChunkName::UNIS),
            ParsedChunk::UPGR(_) => assert_eq!(cn, &ChunkName::UPGR),
            ParsedChunk::UPGS(_) => assert_eq!(cn, &ChunkName::UPGS),
            ParsedChunk::DD2(_) => assert_eq!(cn, &ChunkName::DD2),
            ParsedChunk::THG2(_) => assert_eq!(cn, &ChunkName::THG2),
            ParsedChunk::MASK(_) => assert_eq!(cn, &ChunkName::MASK),
            ParsedChunk::MRGN(_) => assert_eq!(cn, &ChunkName::MRGN),
            ParsedChunk::STR(_) => assert_eq!(cn, &ChunkName::STR),
            ParsedChunk::SPRP(_) => assert_eq!(cn, &ChunkName::SPRP),
            ParsedChunk::FORC(_) => assert_eq!(cn, &ChunkName::FORC),
            ParsedChunk::WAV(_) => assert_eq!(cn, &ChunkName::WAV),
            ParsedChunk::PTEC(_) => assert_eq!(cn, &ChunkName::PTEC),
            ParsedChunk::TECS(_) => assert_eq!(cn, &ChunkName::TECS),
            ParsedChunk::MBRF(_) => assert_eq!(cn, &ChunkName::MBRF),
            ParsedChunk::TRIG(_) => assert_eq!(cn, &ChunkName::TRIG),
            ParsedChunk::UPRP(_) => assert_eq!(cn, &ChunkName::UPRP),
            ParsedChunk::UPUS(_) => assert_eq!(cn, &ChunkName::UPUS),
            ParsedChunk::SWNM(_) => assert_eq!(cn, &ChunkName::SWNM),
            _ => {
                panic!("{cn:?}, {pc:?}");
            }
        }
    }

    for (cn, pc) in parsed_chunks {
        match pc {
            ParsedChunk::VER(x) => {
                assert_eq!(*x.file_format_version, 59);
            }
            ParsedChunk::IVER(x) => {
                assert_eq!(*x.additional_file_format_version, 10);
            }
            ParsedChunk::VCOD(x) => {
                assert_eq!(
                    *x.hash,
                    [1, 4, 5, 6, 2, 1, 5, 2, 0, 3, 7, 7, 5, 4, 6, 3],
                    "{x:?}"
                );
            }
            ParsedChunk::IOWN(x) => {
                assert_eq!(
                    *x.player_owner,
                    [6, 5, 3, 6, 7, 6, 6, 6, 0, 0, 0, 0],
                    "{x:?}"
                );
            }
            ParsedChunk::OWNR(x) => {
                assert_eq!(
                    *x.player_owner,
                    [6, 5, 3, 6, 7, 6, 6, 0, 0, 0, 0, 0],
                    "{x:?}"
                );
            }
            ParsedChunk::SIDE(x) => {
                assert_eq!(
                    *x.player_side,
                    [5, 5, 1, 5, 5, 5, 5, 5, 7, 7, 7, 7],
                    "{x:?}"
                );
            }
            ParsedChunk::ERA(x) => {
                assert_eq!(*x.tileset, 1, "{x:?}");
            }
            ParsedChunk::DIM(x) => {
                assert_eq!(*x.width, 128, "{x:?}");
                assert_eq!(*x.height, 128, "{x:?}");
            }
            ParsedChunk::MTXM(x) => {
                assert_eq!(x.data.len(), 128 * 128, "{x:?}");
                assert_eq!(x.data[(126) + 128 * (1)], 16, "{x:?}"); // 1 pink creep tile
                assert_eq!(x.data[(124) + 128 * (2)], 16385, "{x:?}"); // 2x2 doodad
                assert_eq!(x.data[(124) + 128 * (3)], 16401, "{x:?}"); // 2x2 doodad
                assert_eq!(x.data[(123) + 128 * (2)], 16384, "{x:?}"); // 2x2 doodad
                assert_eq!(x.data[(123) + 128 * (3)], 16400, "{x:?}"); // 2x2 doodad
            }
            ParsedChunk::TILE(x) => {
                assert_eq!(x.data.len(), 128 * 128, "{x:?}");
                assert_eq!(x.data[(126) + 128 * (1)], 16, "{x:?}"); // 1 pink creep tile
                assert_eq!(x.data[(124) + 128 * (2)], 64, "{x:?}"); // 2x2 doodad is not in TILE, so regular space platform
                assert_eq!(x.data[(124) + 128 * (3)], 67, "{x:?}"); // 2x2 doodad is not in TILE, so regular space platform
                assert_eq!(x.data[(123) + 128 * (2)], 81, "{x:?}"); // 2x2 doodad is not in TILE, so regular space platform
                assert_eq!(x.data[(123) + 128 * (3)], 83, "{x:?}"); // 2x2 doodad is not in TILE, so regular space platform
            }
            ParsedChunk::ISOM(x) => {
                x.data.iter().for_each(|&y| assert_eq!(y, 32, "{x:?}"));
            }
            ParsedChunk::UNIT(x) => {
                assert_eq!(x.units.len(), 8, "{x:?}");
                assert_eq!(
                    x.units[0],
                    crate::chk2::chk_unit::ChkUnitIndividual {
                        class_instance: &1,
                        x: &3872,
                        y: &162,
                        unit_id: &0,
                        type_of_relation_to_other_building: &0,
                        properties_that_can_be_applied: &24,
                        properties_that_can_be_changed: &3,
                        owner: &0,
                        hit_points_percent: &74,
                        shield_points_percent: &100,
                        energy_points_percent: &100,
                        resource_amount: &0,
                        number_of_units_in_hangar: &0,
                        unit_state_flags: &24,
                        unused: &0,
                        class_instance_related_to: &0,
                    },
                    "{x:?}"
                );
                assert_eq!(
                    x.units[1],
                    crate::chk2::chk_unit::ChkUnitIndividual {
                        class_instance: &2,
                        x: &3744,
                        y: &80,
                        unit_id: &214,
                        type_of_relation_to_other_building: &0,
                        properties_that_can_be_applied: &24,
                        properties_that_can_be_changed: &3,
                        owner: &2,
                        hit_points_percent: &100,
                        shield_points_percent: &100,
                        energy_points_percent: &100,
                        resource_amount: &0,
                        number_of_units_in_hangar: &0,
                        unit_state_flags: &0,
                        unused: &0,
                        class_instance_related_to: &0,
                    },
                    "{x:?}"
                );
                assert_eq!(
                    x.units[2],
                    crate::chk2::chk_unit::ChkUnitIndividual {
                        class_instance: &3,
                        x: &3680,
                        y: &80,
                        unit_id: &214,
                        type_of_relation_to_other_building: &0,
                        properties_that_can_be_applied: &24,
                        properties_that_can_be_changed: &3,
                        owner: &0,
                        hit_points_percent: &100,
                        shield_points_percent: &100,
                        energy_points_percent: &100,
                        resource_amount: &0,
                        number_of_units_in_hangar: &0,
                        unit_state_flags: &0,
                        unused: &0,
                        class_instance_related_to: &0,
                    },
                    "{x:?}"
                );
                assert_eq!(
                    x.units[3],
                    crate::chk2::chk_unit::ChkUnitIndividual {
                        class_instance: &4,
                        x: &3712,
                        y: &80,
                        unit_id: &214,
                        type_of_relation_to_other_building: &0,
                        properties_that_can_be_applied: &24,
                        properties_that_can_be_changed: &3,
                        owner: &1,
                        hit_points_percent: &100,
                        shield_points_percent: &100,
                        energy_points_percent: &100,
                        resource_amount: &0,
                        number_of_units_in_hangar: &0,
                        unit_state_flags: &0,
                        unused: &0,
                        class_instance_related_to: &0,
                    },
                    "{x:?}"
                );
                assert_eq!(
                    x.units[4],
                    crate::chk2::chk_unit::ChkUnitIndividual {
                        class_instance: &5,
                        x: &3808,
                        y: &80,
                        unit_id: &214,
                        type_of_relation_to_other_building: &0,
                        properties_that_can_be_applied: &24,
                        properties_that_can_be_changed: &3,
                        owner: &3,
                        hit_points_percent: &100,
                        shield_points_percent: &100,
                        energy_points_percent: &100,
                        resource_amount: &0,
                        number_of_units_in_hangar: &0,
                        unit_state_flags: &0,
                        unused: &0,
                        class_instance_related_to: &0,
                    },
                    "{x:?}"
                );
                assert_eq!(
                    x.units[5],
                    crate::chk2::chk_unit::ChkUnitIndividual {
                        class_instance: &6,
                        x: &3840,
                        y: &80,
                        unit_id: &214,
                        type_of_relation_to_other_building: &0,
                        properties_that_can_be_applied: &24,
                        properties_that_can_be_changed: &3,
                        owner: &4,
                        hit_points_percent: &100,
                        shield_points_percent: &100,
                        energy_points_percent: &100,
                        resource_amount: &0,
                        number_of_units_in_hangar: &0,
                        unit_state_flags: &0,
                        unused: &0,
                        class_instance_related_to: &0,
                    },
                    "{x:?}"
                );
                assert_eq!(
                    x.units[6],
                    crate::chk2::chk_unit::ChkUnitIndividual {
                        class_instance: &7,
                        x: &3872,
                        y: &80,
                        unit_id: &214,
                        type_of_relation_to_other_building: &0,
                        properties_that_can_be_applied: &24,
                        properties_that_can_be_changed: &3,
                        owner: &5,
                        hit_points_percent: &100,
                        shield_points_percent: &100,
                        energy_points_percent: &100,
                        resource_amount: &0,
                        number_of_units_in_hangar: &0,
                        unit_state_flags: &0,
                        unused: &0,
                        class_instance_related_to: &0,
                    },
                    "{x:?}"
                );
                assert_eq!(
                    x.units[7],
                    crate::chk2::chk_unit::ChkUnitIndividual {
                        class_instance: &8,
                        x: &3872,
                        y: &80,
                        unit_id: &214,
                        type_of_relation_to_other_building: &0,
                        properties_that_can_be_applied: &24,
                        properties_that_can_be_changed: &3,
                        owner: &6,
                        hit_points_percent: &100,
                        shield_points_percent: &100,
                        energy_points_percent: &100,
                        resource_amount: &0,
                        number_of_units_in_hangar: &0,
                        unit_state_flags: &0,
                        unused: &0,
                        class_instance_related_to: &0,
                    },
                    "{x:?}"
                );
            }
            ParsedChunk::PUNI(_) => assert_eq!(cn, ChunkName::PUNI),
            ParsedChunk::UNIS(_) => assert_eq!(cn, ChunkName::UNIS),
            ParsedChunk::UPGR(_) => assert_eq!(cn, ChunkName::UPGR),
            ParsedChunk::UPGS(_) => assert_eq!(cn, ChunkName::UPGS),
            ParsedChunk::DD2(_) => assert_eq!(cn, ChunkName::DD2),
            ParsedChunk::THG2(_) => assert_eq!(cn, ChunkName::THG2),
            ParsedChunk::MASK(_) => assert_eq!(cn, ChunkName::MASK),
            ParsedChunk::MRGN(_) => assert_eq!(cn, ChunkName::MRGN),
            ParsedChunk::STR(_) => assert_eq!(cn, ChunkName::STR),
            ParsedChunk::SPRP(_) => assert_eq!(cn, ChunkName::SPRP),
            ParsedChunk::FORC(_) => assert_eq!(cn, ChunkName::FORC),
            ParsedChunk::WAV(_) => assert_eq!(cn, ChunkName::WAV),
            ParsedChunk::PTEC(_) => assert_eq!(cn, ChunkName::PTEC),
            ParsedChunk::TECS(_) => assert_eq!(cn, ChunkName::TECS),
            ParsedChunk::MBRF(_) => assert_eq!(cn, ChunkName::MBRF),
            ParsedChunk::TRIG(_) => assert_eq!(cn, ChunkName::TRIG),
            ParsedChunk::UPRP(_) => assert_eq!(cn, ChunkName::UPRP),
            ParsedChunk::UPUS(_) => assert_eq!(cn, ChunkName::UPUS),
            ParsedChunk::SWNM(_) => assert_eq!(cn, ChunkName::SWNM),
            _ => {
                panic!("{cn:?}, {pc:?}");
            }
        }
    }

    // if let ParsedChunk::VER(x) = parsed_chunks.get(&ChunkName::VER).unwrap() {
    //     assert_eq!(*x.file_format_version, 59);
    // } else {
    //     panic!();
    // }

    // if let ParsedChunk::IVER(x) = parsed_chunks.get(&ChunkName::IVER).unwrap() {
    //     assert_eq!(*x.additional_file_format_version, 10);
    // } else {
    //     panic!();
    // }

    // if let ParsedChunk::VCOD(x) = parsed_chunks.get(&ChunkName::VCOD).unwrap() {
    //     assert_eq!(
    //         *x.hash,
    //         [1, 4, 5, 6, 2, 1, 5, 2, 0, 3, 7, 7, 5, 4, 6, 3],
    //         "{x:?}"
    //     );
    // } else {
    //     panic!();
    // }

    // if let ParsedChunk::IOWN(x) = parsed_chunks.get(&ChunkName::IOWN).unwrap() {
    //     assert_eq!(
    //         *x.player_owner,
    //         [1, 4, 5, 6, 2, 1, 5, 2, 0, 3, 7, 1],
    //         "{x:?}"
    //     );
    // } else {
    //     panic!();
    // }

    // if let ParsedChunk::OWNR(x) = parsed_chunks.get(&ChunkName::OWNR).unwrap() {
    //     assert_eq!(
    //         *x.player_owner,
    //         [1, 4, 5, 6, 2, 1, 5, 2, 0, 3, 7, 1],
    //         "{x:?}"
    //     );
    // } else {
    //     panic!();
    // }
}
