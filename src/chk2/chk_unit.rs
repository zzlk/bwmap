use crate::util::CursorSlicer;
use serde::Serialize;

// Required for all versions and all game types.
// Validation: Must be a multiple of 36 bytes.

// This section contains all the pre-placed units on the map and their properties. Each unit on the map gets the following structure:

// u32: The unit's class instance (sort of a "serial number")
// U16: X coordinate of unit
// U16: Y coordinate of unit
// u16: Unit ID
// u16: Type of relation to another building (i.e. add-on, nydus link)
// Bit 9 - Nydus Link
// Bit 10 - Addon Link
// u16: Flags of special properties which can be applied to the unit and are valid:
// Bit 0 - Cloak is valid
// Bit 1 - Burrow is valid
// Bit 2 - In transit is valid
// Bit 3 - Hallucinated is valid
// Bit 4 - Invincible is valid
// Bit 5-15 - Unused
// u16: Out of the elements of the unit data, the properties which can be changed by the map maker:
// Bit 0 - Owner player is valid (the unit is not a critter, start location, etc.; not a neutral unit)
// Bit 1 - HP is valid
// Bit 2 - Shields is valid
// Bit 3 - Energy is valid (unit is a wraith, etc.)
// Bit 4 - Resource amount is valid (unit is a mineral patch, vespene geyser, etc.)
// Bit 5 - Amount in hangar is valid (unit is a reaver, carrier, etc.)
// Bit 6-15 - Unused
// u8: Player number of owner (0-based)
// u8: Hit points % (1-100)
// u8: Shield points % (1-100)
// u8: Energy points % (1-100)
// u32: Resource amount
// u16: Number of units in hangar
// u16: Unit state flags
// Bit 0 - Unit is cloaked
// Bit 1 - Unit is burrowed
// Bit 2 - Building is in transit
// Bit 3 - Unit is hallucinated
// Bit 4 - Unit is invincible
// Bit 5-15 - Unused
// u32: Unused
// u32: Class instance of the unit to which this unit is related to (i.e. via an add-on, nydus link, etc.). It is "0" if the unit is not linked to any other unit.
// Notes about UNIT:

// The X/Y coordinates are the center of the sprite of the unit (in pixels).
// Please edit if you could confirm the bit values to be correct/if you know more bit values.
// Default values will apply if bit values are unchecked. Defaults: 100% HP, 100% SP, 100% EP, 0 resources, 0 hangar count.
// This section can be split. Additional UNIT sections will add more units.

#[derive(Debug, Serialize)]
pub struct ChkUnitIndividual<'a> {
    pub class_instance: &'a u32,
    pub x: &'a u16,
    pub y: &'a u16,
    pub unit_id: &'a u16,
    pub type_of_relation_to_other_building: &'a u16,
    pub properties_that_can_be_applied: &'a u16,
    pub properties_that_can_be_changed: &'a u16,
    pub owner: &'a u8,
    pub hit_points_percent: &'a u8,
    pub shield_points_percent: &'a u8,
    pub energy_points_percent: &'a u8,
    pub resource_amount: &'a u32,
    pub number_of_units_in_hangar: &'a u16,
    pub unit_state_flags: &'a u16,
    pub unused: &'a u32,
    pub class_instance_related_to: &'a u32,
}

#[derive(Debug, Serialize)]
pub struct ChkUnit<'a> {
    pub units: Vec<ChkUnitIndividual<'a>>,
}

pub(crate) fn parse_unit(sec: &[u8]) -> Result<ChkUnit, anyhow::Error> {
    let mut slicer = CursorSlicer::new(sec);

    let mut units = Vec::new();

    for _ in 0..(sec.len() / 36) {
        units.push(ChkUnitIndividual {
            class_instance: slicer.extract_ref()?,
            x: slicer.extract_ref()?,
            y: slicer.extract_ref()?,
            unit_id: slicer.extract_ref()?,
            type_of_relation_to_other_building: slicer.extract_ref()?,
            properties_that_can_be_applied: slicer.extract_ref()?,
            properties_that_can_be_changed: slicer.extract_ref()?,
            owner: slicer.extract_ref()?,
            hit_points_percent: slicer.extract_ref()?,
            shield_points_percent: slicer.extract_ref()?,
            energy_points_percent: slicer.extract_ref()?,
            resource_amount: slicer.extract_ref()?,
            number_of_units_in_hangar: slicer.extract_ref()?,
            unit_state_flags: slicer.extract_ref()?,
            unused: slicer.extract_ref()?,
            class_instance_related_to: slicer.extract_ref()?,
        });
    }

    Ok(ChkUnit { units })
}
