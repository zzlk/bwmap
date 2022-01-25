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
            ParsedChunk::PUNI(x) => {
                for i in 0..228 {
                    // Global Availability
                    assert_eq!(
                        x.unit_global_availability[i],
                        if i == 0 { 0 } else { 1 },
                        "{i} {x:?}"
                    );

                    // Player defaults
                    assert_eq!(
                        x.unit_player_uses_defaults[0][i],
                        if i == 0 { 1 } else { 1 },
                        "{i} {x:?}"
                    );
                    assert_eq!(
                        x.unit_player_uses_defaults[1][i],
                        if i == 0 { 0 } else { 1 },
                        "{i} {x:?}"
                    );
                    assert_eq!(
                        x.unit_player_uses_defaults[2][i],
                        if i == 0 { 0 } else { 1 },
                        "{i} {x:?}"
                    );
                    assert_eq!(
                        x.unit_player_uses_defaults[3][i],
                        if i == 0 { 1 } else { 1 },
                        "{i} {x:?}"
                    );

                    // Player availability
                    assert_eq!(
                        x.unit_player_availability[0][i],
                        if i == 0 { 1 } else { 1 },
                        "{i} {x:?}"
                    );
                    assert_eq!(
                        x.unit_player_availability[1][i],
                        if i == 0 { 1 } else { 1 },
                        "{i} {x:?}"
                    );
                    assert_eq!(
                        x.unit_player_availability[2][i],
                        if i == 0 { 0 } else { 1 },
                        "{i} {x:?}"
                    );
                    assert_eq!(
                        x.unit_player_availability[3][i],
                        if i == 0 { 1 } else { 1 },
                        "{i} {x:?}"
                    );
                }
            }
            ParsedChunk::UNIS(x) => {
                assert_eq!(x.armor_points[0], 39, "{x:?}");
                assert_eq!(x.base_weapon_damage[0], 77, "{x:?}");
                assert_eq!(x.build_time[0], 519, "{x:?}");
                assert_eq!(x.config[0], 0, "{x:?}");
                assert_eq!(x.gas_cost[0], 812, "{x:?}");
                assert_eq!(x.hit_points[0], 536849, "{x:?}");
                assert_eq!(x.shield_points[0], 291, "{x:?}");
                assert_eq!(x.string_number[0], 8, "{x:?}");
                assert_eq!(x.upgrade_bonus_weapon_damage[0], 15, "{x:?}");
            }
            ParsedChunk::UPGR(x) => {
                assert_eq!(
                    *x.global_default_maximum_upgrade_level,
                    [3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3],
                    "{x:?}"
                );
                assert_eq!(
                    *x.global_default_starting_upgrade_level,
                    [3, 3, 3, 3, 14, 1, 0, 1, 1, 1, 1, 1],
                    "{x:?}"
                );
                assert_eq!(
                    *x.player_uses_upgrade_defaults,
                    [
                        [
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0
                        ],
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1,
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1
                        ],
                        [
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1
                        ],
                        [
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                        ],
                        [
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                        ],
                        [
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                        ],
                        [
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                        ],
                        [
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                        ],
                        [
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                        ],
                        [
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                        ],
                        [
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                        ],
                        [
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
                            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                        ]
                    ],
                    "{x:?}"
                );
                assert_eq!(
                    *x.starting_upgrade_level,
                    [
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ],
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ],
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ],
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ],
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ],
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ],
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ],
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ],
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ],
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ],
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ],
                        [
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
                        ]
                    ],
                    "{x:?}"
                );
            }
            ParsedChunk::UPGS(x) => {
                assert_eq!(
                    *x.base_gas_cost,
                    [
                        100, 100, 150, 150, 150, 100, 150, 100, 100, 100, 100, 100, 100, 100, 100,
                        200, 299, 100, 200, 150, 100, 150, 200, 150, 200, 150, 150, 100, 200, 150,
                        150, 150, 150, 150, 150, 200, 200, 200, 150, 150, 150, 100, 200, 100, 150,
                        0
                    ],
                    "{x:?}"
                );
                assert_eq!(
                    *x.base_mineral_cost,
                    [
                        100, 100, 150, 150, 150, 100, 150, 100, 100, 100, 100, 100, 100, 100, 100,
                        200, 77, 100, 200, 150, 100, 150, 200, 150, 200, 150, 150, 100, 200, 150,
                        150, 150, 150, 150, 150, 200, 200, 200, 150, 150, 150, 100, 200, 100, 150,
                        0
                    ],
                    "{x:?}"
                );
                assert_eq!(
                    *x.base_time,
                    [
                        3990, 3990, 3990, 3990, 3990, 3990, 3990, 3990, 3990, 3990, 3990, 3990,
                        3990, 3990, 3990, 3990, 19620, 1500, 2490, 2490, 2490, 2490, 2490, 2490,
                        2400, 1995, 1995, 1500, 1500, 1500, 1500, 2490, 2490, 2490, 1995, 2490,
                        2490, 2490, 1995, 1995, 2490, 2490, 2490, 1500, 2490, 0
                    ],
                    "{x:?}"
                );
                assert_eq!(
                    *x.gas_cost_factor,
                    [
                        75, 75, 75, 75, 75, 75, 75, 75, 75, 50, 50, 50, 75, 50, 75, 100, 15, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0
                    ],
                    "{x:?}"
                );
                assert_eq!(
                    *x.mineral_cost_factor,
                    [
                        75, 75, 75, 75, 75, 75, 75, 75, 75, 50, 50, 50, 75, 50, 75, 100, 13, 0, 0,
                        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0
                    ],
                    "{x:?}"
                );
                assert_eq!(
                    *x.time_factor,
                    [
                        480, 480, 480, 480, 480, 480, 480, 480, 480, 480, 480, 480, 480, 480, 480,
                        480, 285, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                        0, 0, 0, 0, 0, 0, 0
                    ],
                    "{x:?}"
                );
                assert_eq!(
                    *x.upgrade_uses_default_setings,
                    [
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1,
                        1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1
                    ],
                    "{x:?}"
                );
            }
            ParsedChunk::DD2(x) => {
                assert_eq!(x.doodads.len(), 1, "{x:?}");
                assert_eq!(
                    x.doodads[0],
                    crate::chk2::chk_dd2::ChkDd2Individual {
                        doodad_number: 0,
                        x: 3968,
                        y: 96,
                        owner: 0,
                        disabled: 0
                    },
                    "{x:?}"
                );
            }
            ParsedChunk::THG2(x) => {
                assert_eq!(x.sprites.len(), 3, "{x:?}");
                assert_eq!(
                    x.sprites[0],
                    crate::chk2::chk_thg2::ChkThg2Individual {
                        sprite_number: 59,
                        x: 3968,
                        y: 96,
                        owner: 0,
                        unused: 0,
                        flags: 4992,
                    },
                    "{x:?}"
                );
                assert_eq!(
                    x.sprites[1],
                    crate::chk2::chk_thg2::ChkThg2Individual {
                        sprite_number: 0,
                        x: 3970,
                        y: 163,
                        owner: 0,
                        unused: 0,
                        flags: 8832,
                    },
                    "{x:?}"
                );
                assert_eq!(
                    x.sprites[2],
                    crate::chk2::chk_thg2::ChkThg2Individual {
                        sprite_number: 287,
                        x: 3872,
                        y: 61,
                        owner: 0,
                        unused: 0,
                        flags: 4736,
                    },
                    "{x:?}"
                );
            }
            ParsedChunk::MASK(x) => {
                for i in 0..x.fog.len() {
                    assert_eq!(x.fog[i], if i == 890 { 254 } else { 255 }, "{x:?} {i}");
                }
            }
            ParsedChunk::MRGN(x) => {
                assert_eq!(x.locations.len(), 64, "{x:?}");

                for i in 0..x.locations.len() {
                    match i {
                        0 => assert_eq!(
                            x.locations[0],
                            crate::chk2::chk_mrgn::ChkMrgnIndividual {
                                left: 3744,
                                top: 32,
                                right: 3808,
                                bottom: 96,
                                name_string_number: 10,
                                elevation_flags: 0
                            },
                            "{x:?}"
                        ),
                        63 => assert_eq!(
                            x.locations[i],
                            crate::chk2::chk_mrgn::ChkMrgnIndividual {
                                left: 0,
                                top: 0,
                                right: 4096,
                                bottom: 4096,
                                name_string_number: 3,
                                elevation_flags: 0
                            },
                            "{x:?}"
                        ),
                        _ => assert_eq!(
                            x.locations[i],
                            crate::chk2::chk_mrgn::ChkMrgnIndividual {
                                left: 0,
                                top: 0,
                                right: 0,
                                bottom: 0,
                                name_string_number: 0,
                                elevation_flags: 0
                            },
                            "{x:?}"
                        ),
                    };
                }
            }
            ParsedChunk::STR(x) => {
                // offset 2050 is offset of null byte
                assert_eq!(*x.number_of_strings, 1024, "{x:?}");
                assert_eq!(x.string_offsets[0], 2051, "{x:?}");
                assert_eq!(x.string_offsets[1], 2077, "{x:?}");
                assert_eq!(x.string_offsets[2], 2105, "{x:?}");
                assert_eq!(x.string_offsets[3], 2114, "{x:?}");
                assert_eq!(x.string_offsets[4], 2127, "{x:?}");
                assert_eq!(x.string_offsets[5], 2140, "{x:?}");
                assert_eq!(x.string_offsets[6], 2148, "{x:?}");
                assert_eq!(x.string_offsets[7], 2156, "{x:?}");
                assert_eq!(x.string_offsets[8], 2168, "{x:?}");
                assert_eq!(x.string_offsets[9], 2198, "{x:?}");
                assert_eq!(x.string_offsets[10], 2050, "{x:?}");

                assert_eq!(x.strings.len(), 2219, "{x:?}");

                assert_eq!(
                    encoding_rs::WINDOWS_1252
                        .decode(
                            &x.strings[(x.string_offsets[0]) as usize
                                ..(x.string_offsets[0] + 26) as usize],
                        )
                        .0
                        .to_owned(),
                    "test case scenario string\0".to_owned(),
                    "{x:?}"
                );

                assert_eq!(
                    encoding_rs::WINDOWS_1252
                        .decode(
                            &x.strings[(x.string_offsets[0] + 26) as usize
                                ..(x.string_offsets[0] + 54) as usize],
                        )
                        .0
                        .to_owned(),
                    "test case scenario string 3\0".to_owned(),
                    "{x:?}"
                );
                assert_eq!(
                    encoding_rs::WINDOWS_1252
                        .decode(
                            &x.strings[(x.string_offsets[0] + 63) as usize
                                ..(x.string_offsets[0] + 76) as usize],
                        )
                        .0
                        .to_owned(),
                    "test force 1\0".to_owned(),
                    "{x:?}"
                );
                assert_eq!(
                    encoding_rs::WINDOWS_1252
                        .decode(
                            &x.strings[(x.string_offsets[0] + 76) as usize
                                ..(x.string_offsets[0] + 89) as usize],
                        )
                        .0
                        .to_owned(),
                    "test force 2\0".to_owned(),
                    "{x:?}"
                );
                assert_eq!(
                    encoding_rs::WINDOWS_1252
                        .decode(
                            &x.strings[(x.string_offsets[0] + 89) as usize
                                ..(x.string_offsets[0] + 97) as usize],
                        )
                        .0
                        .to_owned(),
                    "Force 3\0".to_owned(),
                    "{x:?}"
                );
                assert_eq!(
                    encoding_rs::WINDOWS_1252
                        .decode(
                            &x.strings[(x.string_offsets[0] + 97) as usize
                                ..(x.string_offsets[0] + 105) as usize],
                        )
                        .0
                        .to_owned(),
                    "Force 4\0".to_owned(),
                    "{x:?}"
                );
                assert_eq!(
                    encoding_rs::WINDOWS_1252
                        .decode(
                            &x.strings[(x.string_offsets[0] + 105) as usize
                                ..(x.string_offsets[0] + 117) as usize],
                        )
                        .0
                        .to_owned(),
                    "Test Case 1\0".to_owned(),
                    "{x:?}"
                );
                assert_eq!(
                    encoding_rs::WINDOWS_1252
                        .decode(
                            &x.strings[(x.string_offsets[0] + 117) as usize
                                ..(x.string_offsets[0] + 147) as usize],
                        )
                        .0
                        .to_owned(),
                    "sound\\Zerg\\Drone\\ZDrErr00.WAV\0".to_owned(),
                    "{x:?}"
                );
                assert_eq!(
                    encoding_rs::WINDOWS_1252
                        .decode(
                            &x.strings[(x.string_offsets[0] + 147) as usize
                                ..(x.string_offsets[0] + 168) as usize],
                        )
                        .0
                        .to_owned(),
                    "location test string\0".to_owned(),
                    "{x:?}"
                );
            }
            ParsedChunk::SPRP(x) => {
                assert_eq!(*x.scenario_name_string_number, 1, "{x:?}");
                assert_eq!(*x.description_string_number, 2, "{x:?}");
            }
            ParsedChunk::FORC(x) => {
                assert_eq!(*x.force_name, [4, 5, 6, 7], "{x:?}");
                assert_eq!(*x.force_properties, [15, 9, 15, 15], "{x:?}");
                assert_eq!(*x.player_forces, [0, 1, 2, 3, 0, 0, 0, 0], "{x:?}");
            }
            ParsedChunk::WAV(x) => {
                assert_eq!(x.wav_string_number.len(), 512, "{x:?}");
                assert_eq!(x.wav_string_number[0], 9, "{x:?}");
            }
            ParsedChunk::PTEC(x) => {
                assert_eq!(
                    *x.already_researched,
                    [
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                    ],
                    "{x:?}"
                );
                assert_eq!(
                    *x.global_already_researched_defaults,
                    [0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1],
                    "{x:?}"
                );
                assert_eq!(
                    *x.player_availability,
                    [
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
                    ],
                    "{x:?}"
                );
                assert_eq!(
                    *x.player_uses_default,
                    [
                        [1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                        [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
                    ],
                    "{x:?}"
                );
            }
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
