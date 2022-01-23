use crate::util::CursorSlicer;
use std::cmp::min;

// Required for all versions. Not required for Melee.
// Validation: Must be a multiple of 2400 bytes.

// This section contains all of the mission briefings shown by the players.

// See "TRIG" section for format, as it is exactly the same except for the conditions. In this section the 16 conditions are still there, they are all null except for the very first condition, which only has a condition byte of 13. See #Mission Briefing Actions List for the action bytes for the mission briefing actions.

// This section can be split. Additional MBRF sections will add more briefing triggers.

#[derive(Debug)]
pub struct ChkMbrfCondition<'a> {
    pub location: &'a u32,
    pub group: &'a u32,
    pub qualified_number: &'a u32,
    pub unit_id: &'a u16,
    pub numeric_comparison_or_switch_state: &'a u8,
    pub condition: &'a u8,
    pub resource_type_or_score_type_or_switch_number: &'a u8,
    pub flags: &'a u8,
    pub mask_flag: &'a u16,
}

fn parse_mbrf_condition<'a>(
    slicer: &mut CursorSlicer<'a>,
) -> Result<ChkMbrfCondition<'a>, anyhow::Error> {
    Ok(ChkMbrfCondition {
        location: slicer.extract_ref()?,
        group: slicer.extract_ref()?,
        qualified_number: slicer.extract_ref()?,
        unit_id: slicer.extract_ref()?,
        numeric_comparison_or_switch_state: slicer.extract_ref()?,
        condition: slicer.extract_ref()?,
        resource_type_or_score_type_or_switch_number: slicer.extract_ref()?,
        flags: slicer.extract_ref()?,
        mask_flag: slicer.extract_ref()?,
    })
}

#[derive(Debug)]
pub struct ChkMbrfAction<'a> {
    pub location: &'a u32,
    pub string_number: &'a u32,
    pub wav_string_number: &'a u32,
    pub seconds_or_milliseconds: &'a u32,
    pub first_or_only_group_or_player_affected: &'a u32,
    pub second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number:
        &'a u32,
    pub unit_type_or_score_type_or_resource_type_or_alliance_status: &'a u16,
    pub action: &'a u8,
    pub number_of_units_or_action_state_or_unit_order_or_number_modifier: &'a u8,
    pub flags: &'a u8,
    pub padding: &'a u8,
    pub mask_flag: &'a u16,
}

fn parse_mbrf_action<'a>(
    slicer: &mut CursorSlicer<'a>,
) -> Result<ChkMbrfAction<'a>, anyhow::Error> {
    Ok(ChkMbrfAction {
        location: slicer.extract_ref()?,
        string_number: slicer.extract_ref()?,
        wav_string_number: slicer.extract_ref()?,
        seconds_or_milliseconds: slicer.extract_ref()?,
        first_or_only_group_or_player_affected: slicer.extract_ref()?,
        second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number: slicer.extract_ref()?,
        unit_type_or_score_type_or_resource_type_or_alliance_status: slicer.extract_ref()?,
        action: slicer.extract_ref()?,
        number_of_units_or_action_state_or_unit_order_or_number_modifier: slicer.extract_ref()?,
        flags: slicer.extract_ref()?,
        padding: slicer.extract_ref()?,
        mask_flag: slicer.extract_ref()?,
    })
}

#[derive(Debug)]
pub struct ChkMbrfIndividual<'a> {
    pub conditions: [ChkMbrfCondition<'a>; 16],
    pub actions: [ChkMbrfAction<'a>; 64],
    pub execution_flags: &'a u32,
    pub executed_for_player: &'a [u8; 27],
    pub current_action: &'a u8,
}

pub(crate) fn parse_mbrf_individual<'a>(
    mut slicer: &mut CursorSlicer<'a>,
) -> Result<ChkMbrfIndividual<'a>, anyhow::Error> {
    let conditions = [
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
        parse_mbrf_condition(&mut slicer)?,
    ];

    let actions = [
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
        parse_mbrf_action(&mut slicer)?,
    ];

    Ok(ChkMbrfIndividual {
        conditions,
        actions,
        execution_flags: slicer.extract_ref()?,
        executed_for_player: slicer.extract_ref()?,
        current_action: slicer.extract_ref()?,
    })
}

#[derive(Debug)]
pub struct ChkMbrf<'a> {
    pub triggers: Vec<ChkMbrfIndividual<'a>>,
}

pub(crate) fn parse_mbrf<'a>(sec: &'a [u8]) -> Result<ChkMbrf<'a>, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);
    let mut triggers = Vec::new();

    anyhow::ensure!(sec.len() % 2400 == 0);
    for _ in 0..sec.len() / 2400 {
        triggers.push(parse_mbrf_individual(&mut slicer)?);
    }

    Ok(ChkMbrf { triggers })
}
