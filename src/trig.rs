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

use serde::Serialize;

use std::collections::HashMap;

use crate::{
    chk2::chk_trig::{ChkTrigAction, ChkTrigCondition},
    ChunkName, ParsedChunk,
};

#[derive(Debug, Serialize)]
pub enum ResourceType {
    Unknown(i64),
    Ore,
    Gas,
    OreAndGas,
}

fn parse_resource_type(resource_type: u16) -> ResourceType {
    match resource_type {
        0 => ResourceType::Ore,
        1 => ResourceType::Gas,
        10 => ResourceType::OreAndGas,
        _ => ResourceType::Unknown(resource_type as i64),
    }
}

#[derive(Debug, Serialize)]
pub enum NumericComparison {
    Unknown(i64),
    AtLeast,
    AtMost,
    Exactly,
}

fn parse_numeric_comparison(numeric_comparison: u8) -> NumericComparison {
    match numeric_comparison {
        0 => NumericComparison::AtLeast,
        1 => NumericComparison::AtMost,
        10 => NumericComparison::Exactly,
        _ => NumericComparison::Unknown(numeric_comparison as i64),
    }
}

#[derive(Debug, Serialize)]
pub enum NumberModifier {
    Unknown(i64),
    SetTo,
    Add,
    Subtract,
}

fn parse_number_modifier(modifier: u8) -> NumberModifier {
    match modifier {
        7 => NumberModifier::SetTo,
        8 => NumberModifier::Add,
        9 => NumberModifier::Subtract,
        _ => NumberModifier::Unknown(modifier as i64),
    }
}

#[derive(Debug, Serialize)]
pub enum ScoreType {
    Unknown(i64),
    Total,
    Units,
    Buildings,
    UnitsAndBuildings,
    Kills,
    Razings,
    KillsAndRazings,
    Custom,
}

fn parse_score_type(score_type: u16) -> ScoreType {
    match score_type {
        0 => ScoreType::Total,
        1 => ScoreType::Units,
        2 => ScoreType::Buildings,
        3 => ScoreType::UnitsAndBuildings,
        4 => ScoreType::Kills,
        5 => ScoreType::Razings,
        6 => ScoreType::KillsAndRazings,
        7 => ScoreType::Custom,
        _ => ScoreType::Unknown(score_type as i64),
    }
}

#[derive(Debug, Serialize)]
pub enum SwitchState {
    Unknown(i64),
    Set,
    Cleared,
}

fn parse_switch_state(switch_state: u8) -> SwitchState {
    match switch_state {
        2 => SwitchState::Set,
        3 => SwitchState::Cleared,
        _ => SwitchState::Unknown(switch_state as i64),
    }
}

#[derive(Debug, Serialize)]
pub enum Order {
    Unknown(i64),
    Move,
    Patrol,
    Attack,
}

fn parse_order(order: u8) -> Order {
    match order {
        0 => Order::Move,
        1 => Order::Patrol,
        2 => Order::Attack,
        _ => Order::Unknown(order as i64),
    }
}

#[derive(Debug, Serialize)]
pub enum ActionState {
    Unknown(i64),
    EnabledOrSet,
    DisabledOrClear,
    ToggleOrToggle,
    RandomizeSwitch,
}

fn parse_action_state(action_state: u8) -> ActionState {
    match action_state {
        4 => ActionState::EnabledOrSet,
        5 => ActionState::DisabledOrClear,
        6 => ActionState::ToggleOrToggle,
        11 => ActionState::RandomizeSwitch,
        _ => ActionState::Unknown(action_state as i64),
    }
}

#[derive(Debug, Serialize)]
pub enum AllianceStatus {
    Unknown(i64),
    Enemy,
    Allied,
    AlliedVictory,
}

fn parse_alliance_status(alliance_status: u16) -> AllianceStatus {
    match alliance_status {
        0 => AllianceStatus::Enemy,
        1 => AllianceStatus::Allied,
        2 => AllianceStatus::AlliedVictory,
        _ => AllianceStatus::Unknown(alliance_status as i64),
    }
}

#[derive(Debug, Serialize)]
pub enum UnitType {
    Unknown(i64),
    TerranMarine,
    TerranGhost,
    TerranVulture,
    TerranGoliath,
    GoliathTurret,
    TerranSiegeTankTankMode,
    TankTurretTankMode,
    TerranSCV,
    TerranWraith,
    TerranScienceVessel,
    GuiMontangFirebat,
    TerranDropship,
    TerranBattlecruiser,
    VultureSpiderMine,
    NuclearMissile,
    TerranCivilian,
    SarahKerriganGhost,
    AlanSchezarGoliath,
    AlanSchezarTurret,
    JimRaynorVulture,
    JimRaynorMarine,
    TomKazanskyWraith,
    MagellanScienceVessel,
    EdmundDukeSiegeTank,
    EdmundDukeTurretTankMode,
    EdmundDukeSiegeMode,
    EdmundDukeTurretSiegeMode,
    ArcturusMengskBattlecruiser,
    HyperionBattlecruiser,
    NoradIIBattlecruiser,
    TerranSiegeTankSiegeMode,
    TankTurretSiegeMode,
    Firebat,
    ScannerSweep,
    TerranMedic,
    ZergLarva,
    ZergEgg,
    ZergZergling,
    ZergHydralisk,
    ZergUltralisk,
    ZergBroodling,
    ZergDrone,
    ZergOverlord,
    ZergMutalisk,
    ZergGuardian,
    ZergQueen,
    ZergDefiler,
    ZergScourge,
    TorrarsqueUltralisk,
    MatriarchQueen,
    InfestedTerran,
    InfestedKerrigan,
    UncleanOneDefiler,
    HunterKillerHydralisk,
    DevouringOneZergling,
    KukulzaMutalisk,
    KukulzaGuardian,
    YggdrasillOverlord,
    TerranValkyrie,
    MutaliskOrGuardianCocoon,
    ProtossCorsair,
    ProtossDarkTemplar,
    ZergDevourer,
    ProtossDarkArchon,
    ProtossProbe,
    ProtossZealot,
    ProtossDragoon,
    ProtossHighTemplar,
    ProtossArchon,
    ProtossShuttle,
    ProtossScout,
    ProtossArbiter,
    ProtossCarrier,
    ProtossInterceptor,
    DarkTemplarHero,
    ZeratulDarkTemplar,
    TassadarAndZeratulArchon,
    FenixZealot,
    FenixDragoon,
    TassadarTemplar,
    MojoScout,
    WarbringerReaver,
    GantrithorCarrier,
    ProtossReaver,
    ProtossObserver,
    ProtossScarab,
    DanimothArbiter,
    AldarisTemplar,
    ArtanisScout,
    RhynadonBadlandsCritter,
    BengalaasJungleCritter,
    UnusedWasCargoShip,
    UnusedWasMercenaryGunship,
    ScantidDesertCritter,
    KakaruTwilightCritter,
    RagnasaurAshworldCritter,
    UrsadonIceWorldCritter,
    LurkerEgg,
    Raszagal,
    SamirDuranGhost,
    AlexeiStukovGhost,
    MapRevealer,
    GerardDuGalle,
    ZergLurker,
    InfestedDuran,
    DisruptionWeb,
    TerranCommandCenter,
    TerranComsatStation,
    TerranNuclearSilo,
    TerranSupplyDepot,
    TerranRefinery,
    TerranBarracks,
    TerranAcademy,
    TerranFactory,
    TerranStarport,
    TerranControlTower,
    TerranScienceFacility,
    TerranCovertOps,
    TerranPhysicsLab,
    UnusedWasStarbase,
    TerranMachineShop,
    UnusedWasRepairBay,
    TerranEngineeringBay,
    TerranArmory,
    TerranMissileTurret,
    TerranBunker,
    NoradII,
    IonCannon,
    UrajCrystal,
    KhalisCrystal,
    InfestedCommandCenter,
    ZergHatchery,
    ZergLair,
    ZergHive,
    ZergNydusCanal,
    ZergHydraliskDen,
    ZergDefilerMound,
    ZergGreaterSpire,
    ZergQueensNest,
    ZergEvolutionChamber,
    ZergUltraliskCavern,
    ZergSpire,
    ZergSpawningPool,
    ZergCreepColony,
    ZergSporeColony,
    UnusedZergBuilding,
    ZergSunkenColony,
    ZergOvermindWithShell,
    ZergOvermind,
    ZergExtractor,
    MatureChrysalis,
    ZergCerebrate,
    ZergCerebrateDaggoth,
    UnusedZergBuilding5,
    ProtossNexus,
    ProtossRoboticsFacility,
    ProtossPylon,
    ProtossAssimilator,
    UnusedProtossBuilding1,
    ProtossObservatory,
    ProtossGateway,
    UnusedProtossBuilding2,
    ProtossPhotonCannon,
    ProtossCitadelofAdun,
    ProtossCyberneticsCore,
    ProtossTemplarArchives,
    ProtossForge,
    ProtossStargate,
    StasisCellPrison,
    ProtossFleetBeacon,
    ProtossArbiterTribunal,
    ProtossRoboticsSupportBay,
    ProtossShieldBattery,
    KhaydarinCrystalFormation,
    ProtossTemple,
    XelNagaTemple,
    MineralFieldType1,
    MineralFieldType2,
    MineralFieldType3,
    Cave,
    Cavein,
    Cantina,
    MiningPlatform,
    IndependantCommandCenter,
    IndependantStarport,
    IndependantJumpGate,
    Ruins,
    KyadarinCrystalFormation,
    VespeneGeyser,
    WarpGate,
    PSIDisruptor,
    ZergMarker,
    TerranMarker,
    ProtossMarker,
    ZergBeacon,
    TerranBeacon,
    ProtossBeacon,
    ZergFlagBeacon,
    TerranFlagBeacon,
    ProtossFlagBeacon,
    PowerGenerator,
    OvermindCocoon,
    DarkSwarm,
    FloorMissileTrap,
    FloorHatch,
    LeftUpperLevelDoor,
    RightUpperLevelDoor,
    LeftPitDoor,
    RightPitDoor,
    FloorGunTrap,
    LeftWallMissileTrap,
    LeftWallFlameTrap,
    RightWallMissileTrap,
    RightWallFlameTrap,
    StartLocation,
    Flag,
    YoungChrysalis,
    PsiEmitter,
    DataDisc,
    KhaydarinCrystal,
    MineralClusterType1,
    MineralClusterType2,
    ProtossVespeneGasOrbType1,
    ProtossVespeneGasOrbType2,
    ZergVespeneGasSacType1,
    ZergVespeneGasSacType2,
    TerranVespeneGasTankType1,
    TerranVespeneGasTankType2,
    None,
    AnyUnit,
    Men,
    Buildings,
    Factories,
}

fn parse_unit_type(unit_type: u16) -> UnitType {
    match unit_type {
        0 => UnitType::TerranMarine,
        1 => UnitType::TerranGhost,
        2 => UnitType::TerranVulture,
        3 => UnitType::TerranGoliath,
        4 => UnitType::GoliathTurret,
        5 => UnitType::TerranSiegeTankTankMode,
        6 => UnitType::TankTurretTankMode,
        7 => UnitType::TerranSCV,
        8 => UnitType::TerranWraith,
        9 => UnitType::TerranScienceVessel,
        10 => UnitType::GuiMontangFirebat,
        11 => UnitType::TerranDropship,
        12 => UnitType::TerranBattlecruiser,
        13 => UnitType::VultureSpiderMine,
        14 => UnitType::NuclearMissile,
        15 => UnitType::TerranCivilian,
        16 => UnitType::SarahKerriganGhost,
        17 => UnitType::AlanSchezarGoliath,
        18 => UnitType::AlanSchezarTurret,
        19 => UnitType::JimRaynorVulture,
        20 => UnitType::JimRaynorMarine,
        21 => UnitType::TomKazanskyWraith,
        22 => UnitType::MagellanScienceVessel,
        23 => UnitType::EdmundDukeSiegeTank,
        24 => UnitType::EdmundDukeTurretTankMode,
        25 => UnitType::EdmundDukeSiegeMode,
        26 => UnitType::EdmundDukeTurretSiegeMode,
        27 => UnitType::ArcturusMengskBattlecruiser,
        28 => UnitType::HyperionBattlecruiser,
        29 => UnitType::NoradIIBattlecruiser,
        30 => UnitType::TerranSiegeTankSiegeMode,
        31 => UnitType::TankTurretSiegeMode,
        32 => UnitType::Firebat,
        33 => UnitType::ScannerSweep,
        34 => UnitType::TerranMedic,
        35 => UnitType::ZergLarva,
        36 => UnitType::ZergEgg,
        37 => UnitType::ZergZergling,
        38 => UnitType::ZergHydralisk,
        39 => UnitType::ZergUltralisk,
        40 => UnitType::ZergBroodling,
        41 => UnitType::ZergDrone,
        42 => UnitType::ZergOverlord,
        43 => UnitType::ZergMutalisk,
        44 => UnitType::ZergGuardian,
        45 => UnitType::ZergQueen,
        46 => UnitType::ZergDefiler,
        47 => UnitType::ZergScourge,
        48 => UnitType::TorrarsqueUltralisk,
        49 => UnitType::MatriarchQueen,
        50 => UnitType::InfestedTerran,
        51 => UnitType::InfestedKerrigan,
        52 => UnitType::UncleanOneDefiler,
        53 => UnitType::HunterKillerHydralisk,
        54 => UnitType::DevouringOneZergling,
        55 => UnitType::KukulzaMutalisk,
        56 => UnitType::KukulzaGuardian,
        57 => UnitType::YggdrasillOverlord,
        58 => UnitType::TerranValkyrie,
        59 => UnitType::MutaliskOrGuardianCocoon,
        60 => UnitType::ProtossCorsair,
        61 => UnitType::ProtossDarkTemplar,
        62 => UnitType::ZergDevourer,
        63 => UnitType::ProtossDarkArchon,
        64 => UnitType::ProtossProbe,
        65 => UnitType::ProtossZealot,
        66 => UnitType::ProtossDragoon,
        67 => UnitType::ProtossHighTemplar,
        68 => UnitType::ProtossArchon,
        69 => UnitType::ProtossShuttle,
        70 => UnitType::ProtossScout,
        71 => UnitType::ProtossArbiter,
        72 => UnitType::ProtossCarrier,
        73 => UnitType::ProtossInterceptor,
        74 => UnitType::DarkTemplarHero,
        75 => UnitType::ZeratulDarkTemplar,
        76 => UnitType::TassadarAndZeratulArchon,
        77 => UnitType::FenixZealot,
        78 => UnitType::FenixDragoon,
        79 => UnitType::TassadarTemplar,
        80 => UnitType::MojoScout,
        81 => UnitType::WarbringerReaver,
        82 => UnitType::GantrithorCarrier,
        83 => UnitType::ProtossReaver,
        84 => UnitType::ProtossObserver,
        85 => UnitType::ProtossScarab,
        86 => UnitType::DanimothArbiter,
        87 => UnitType::AldarisTemplar,
        88 => UnitType::ArtanisScout,
        89 => UnitType::RhynadonBadlandsCritter,
        90 => UnitType::BengalaasJungleCritter,
        91 => UnitType::UnusedWasCargoShip,
        92 => UnitType::UnusedWasMercenaryGunship,
        93 => UnitType::ScantidDesertCritter,
        94 => UnitType::KakaruTwilightCritter,
        95 => UnitType::RagnasaurAshworldCritter,
        96 => UnitType::UrsadonIceWorldCritter,
        97 => UnitType::LurkerEgg,
        98 => UnitType::Raszagal,
        99 => UnitType::SamirDuranGhost,
        100 => UnitType::AlexeiStukovGhost,
        101 => UnitType::MapRevealer,
        102 => UnitType::GerardDuGalle,
        103 => UnitType::ZergLurker,
        104 => UnitType::InfestedDuran,
        105 => UnitType::DisruptionWeb,
        106 => UnitType::TerranCommandCenter,
        107 => UnitType::TerranComsatStation,
        108 => UnitType::TerranNuclearSilo,
        109 => UnitType::TerranSupplyDepot,
        110 => UnitType::TerranRefinery,
        111 => UnitType::TerranBarracks,
        112 => UnitType::TerranAcademy,
        113 => UnitType::TerranFactory,
        114 => UnitType::TerranStarport,
        115 => UnitType::TerranControlTower,
        116 => UnitType::TerranScienceFacility,
        117 => UnitType::TerranCovertOps,
        118 => UnitType::TerranPhysicsLab,
        119 => UnitType::UnusedWasStarbase,
        120 => UnitType::TerranMachineShop,
        121 => UnitType::UnusedWasRepairBay,
        122 => UnitType::TerranEngineeringBay,
        123 => UnitType::TerranArmory,
        124 => UnitType::TerranMissileTurret,
        125 => UnitType::TerranBunker,
        126 => UnitType::NoradII,
        127 => UnitType::IonCannon,
        128 => UnitType::UrajCrystal,
        129 => UnitType::KhalisCrystal,
        130 => UnitType::InfestedCommandCenter,
        131 => UnitType::ZergHatchery,
        132 => UnitType::ZergLair,
        133 => UnitType::ZergHive,
        134 => UnitType::ZergNydusCanal,
        135 => UnitType::ZergHydraliskDen,
        136 => UnitType::ZergDefilerMound,
        137 => UnitType::ZergGreaterSpire,
        138 => UnitType::ZergQueensNest,
        139 => UnitType::ZergEvolutionChamber,
        140 => UnitType::ZergUltraliskCavern,
        141 => UnitType::ZergSpire,
        142 => UnitType::ZergSpawningPool,
        143 => UnitType::ZergCreepColony,
        144 => UnitType::ZergSporeColony,
        145 => UnitType::UnusedZergBuilding,
        146 => UnitType::ZergSunkenColony,
        147 => UnitType::ZergOvermindWithShell,
        148 => UnitType::ZergOvermind,
        149 => UnitType::ZergExtractor,
        150 => UnitType::MatureChrysalis,
        151 => UnitType::ZergCerebrate,
        152 => UnitType::ZergCerebrateDaggoth,
        153 => UnitType::UnusedZergBuilding5,
        154 => UnitType::ProtossNexus,
        155 => UnitType::ProtossRoboticsFacility,
        156 => UnitType::ProtossPylon,
        157 => UnitType::ProtossAssimilator,
        158 => UnitType::UnusedProtossBuilding1,
        159 => UnitType::ProtossObservatory,
        160 => UnitType::ProtossGateway,
        161 => UnitType::UnusedProtossBuilding2,
        162 => UnitType::ProtossPhotonCannon,
        163 => UnitType::ProtossCitadelofAdun,
        164 => UnitType::ProtossCyberneticsCore,
        165 => UnitType::ProtossTemplarArchives,
        166 => UnitType::ProtossForge,
        167 => UnitType::ProtossStargate,
        168 => UnitType::StasisCellPrison,
        169 => UnitType::ProtossFleetBeacon,
        170 => UnitType::ProtossArbiterTribunal,
        171 => UnitType::ProtossRoboticsSupportBay,
        172 => UnitType::ProtossShieldBattery,
        173 => UnitType::KhaydarinCrystalFormation,
        174 => UnitType::ProtossTemple,
        175 => UnitType::XelNagaTemple,
        176 => UnitType::MineralFieldType1,
        177 => UnitType::MineralFieldType2,
        178 => UnitType::MineralFieldType3,
        179 => UnitType::Cave,
        180 => UnitType::Cavein,
        181 => UnitType::Cantina,
        182 => UnitType::MiningPlatform,
        183 => UnitType::IndependantCommandCenter,
        184 => UnitType::IndependantStarport,
        185 => UnitType::IndependantJumpGate,
        186 => UnitType::Ruins,
        187 => UnitType::KyadarinCrystalFormation,
        188 => UnitType::VespeneGeyser,
        189 => UnitType::WarpGate,
        190 => UnitType::PSIDisruptor,
        191 => UnitType::ZergMarker,
        192 => UnitType::TerranMarker,
        193 => UnitType::ProtossMarker,
        194 => UnitType::ZergBeacon,
        195 => UnitType::TerranBeacon,
        196 => UnitType::ProtossBeacon,
        197 => UnitType::ZergFlagBeacon,
        198 => UnitType::TerranFlagBeacon,
        199 => UnitType::ProtossFlagBeacon,
        200 => UnitType::PowerGenerator,
        201 => UnitType::OvermindCocoon,
        202 => UnitType::DarkSwarm,
        203 => UnitType::FloorMissileTrap,
        204 => UnitType::FloorHatch,
        205 => UnitType::LeftUpperLevelDoor,
        206 => UnitType::RightUpperLevelDoor,
        207 => UnitType::LeftPitDoor,
        208 => UnitType::RightPitDoor,
        209 => UnitType::FloorGunTrap,
        210 => UnitType::LeftWallMissileTrap,
        211 => UnitType::LeftWallFlameTrap,
        212 => UnitType::RightWallMissileTrap,
        213 => UnitType::RightWallFlameTrap,
        214 => UnitType::StartLocation,
        215 => UnitType::Flag,
        216 => UnitType::YoungChrysalis,
        217 => UnitType::PsiEmitter,
        218 => UnitType::DataDisc,
        219 => UnitType::KhaydarinCrystal,
        220 => UnitType::MineralClusterType1,
        221 => UnitType::MineralClusterType2,
        222 => UnitType::ProtossVespeneGasOrbType1,
        223 => UnitType::ProtossVespeneGasOrbType2,
        224 => UnitType::ZergVespeneGasSacType1,
        225 => UnitType::ZergVespeneGasSacType2,
        226 => UnitType::TerranVespeneGasTankType1,
        227 => UnitType::TerranVespeneGasTankType2,
        228 => UnitType::None,
        229 => UnitType::AnyUnit,
        230 => UnitType::Men,
        231 => UnitType::Buildings,
        232 => UnitType::Factories,
        _ => UnitType::Unknown(unit_type as i64),
    }
}

#[derive(Debug, Serialize)]
pub enum Group {
    Unknown(i64),
    Player1,
    Player2,
    Player3,
    Player4,
    Player5,
    Player6,
    Player7,
    Player8,
    Player9,
    Player10,
    Player11,
    Player12,
    None,
    CurrentPlayer,
    Foes,
    Allies,
    NeutralPlayers,
    AllPlayers,
    Force1,
    Force2,
    Force3,
    Force4,
    Unused1,
    Unused2,
    Unused3,
    Unused4,
    NonAlliedVictoryPlayers,
}

fn parse_group(group: u32) -> Group {
    match group {
        0 => Group::Player1,
        1 => Group::Player2,
        2 => Group::Player3,
        3 => Group::Player4,
        4 => Group::Player5,
        5 => Group::Player6,
        6 => Group::Player7,
        7 => Group::Player8,
        8 => Group::Player9,
        9 => Group::Player10,
        10 => Group::Player11,
        11 => Group::Player12,
        12 => Group::None,
        13 => Group::CurrentPlayer,
        14 => Group::Foes,
        15 => Group::Allies,
        16 => Group::NeutralPlayers,
        17 => Group::AllPlayers,
        18 => Group::Force1,
        19 => Group::Force2,
        20 => Group::Force3,
        21 => Group::Force4,
        22 => Group::Unused1,
        23 => Group::Unused2,
        24 => Group::Unused3,
        25 => Group::Unused4,
        26 => Group::NonAlliedVictoryPlayers,
        _ => Group::Unknown(group as i64),
    }
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Condition {
    Unknown {
        id: i64,
        raw: ChkTrigCondition,
    },
    // 0
    NoCondition,
    // 1
    CountdownTimer {
        comparison: NumericComparison,
        number: i64,
    },
    // 2
    Command {
        player: Group,
        comparison: NumericComparison,
        unit_type: UnitType,
        number: i64,
    },
    // 3
    Bring {
        player: Group,
        comparison: NumericComparison,
        unit_type: UnitType,
        location: i64,
        number: i64,
    },
    // 4
    Accumulate {
        player: Group,
        comparison: NumericComparison,
        unit_type: UnitType,
        number: i64,
        resource_type: ResourceType,
    },
    // 5
    Kill {
        player: Group,
        comparison: NumericComparison,
        unit_type: UnitType,
        number: i64,
    },
    // 6
    CommandsTheMost {
        unit_type: UnitType,
    },
    // 7
    CommandsTheMostAt {
        unit_type: UnitType,
        location: i64,
    },
    // 8
    MostKills {
        unit_type: UnitType,
    },
    // 9
    HighestScore {
        score_type: ScoreType,
    },
    // 10
    MostResources {
        resource_type: ResourceType,
    },
    // 11
    Switch {
        switch: i64,
        switch_state: SwitchState,
    },
    // 12
    ElapsedTime {
        comparison: NumericComparison,
        number: i64,
    },
    // 13
    DataIsAMissionBriefing,
    // 14
    Opponents {
        player: Group,
        comparison: NumericComparison,
        number: i64,
    },
    // 15
    Deaths {
        player: Group,
        comparison: NumericComparison,
        unit_type: UnitType,
        number: i64,
    },
    // 16
    CommandsTheLeast {
        unit_type: UnitType,
    },
    // 17
    CommandsTheLeastAt {
        unit_type: UnitType,
        location: i64,
    },
    // 18
    LeastKills {
        unit_type: UnitType,
    },
    // 19
    LowestScore {
        score_type: ScoreType,
    },
    // 20
    LeastResources {
        resource_type: ResourceType,
    },
    // 21
    Score {
        player: Group,
        comparison: NumericComparison,
        score_type: ScoreType,
        number: i64,
    },
    // 22
    Always,
    // 23
    Never,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum Action {
    Unknown {
        id: i64,
        raw: ChkTrigAction,
    },
    // 0
    NoAction,
    // 1
    Victory,
    // 2
    Defeat,
    // 3
    PreserveTrigger,
    // 4
    Wait {
        time: i64,
    },
    // 5
    PauseGame,
    // 6
    UnpauseGame,
    // 7
    Transmission {
        text: i64,
        unit_type: UnitType,
        location: i64,
        time: i64,
        modifier: NumberModifier,
        wave: i64,
        wave_time: i64,
    },
    // 8
    PlayWav {
        wave: i64,
        wave_time: i64,
    },
    // 9
    DisplayTextMessage {
        text: i64,
    },
    // 10
    CenterView {
        location: i64,
    },
    // 11
    CreateUnitWithProperties {
        player: Group,
        unit_type: UnitType,
        number: i64,
        location: i64,
        unit_prop: i64,
    },
    // 12
    SetMissionObjectives {
        text: i64,
    },
    // 13
    SetSwitch {
        switch: i64,
        switch_action: ActionState,
    },
    // 14
    SetCountdownTimer {
        time: i64,
        modifier: NumberModifier,
    },
    // 15
    RunAIScript {
        script: i64,
    },
    // 16
    RunAIScriptAtLocation {
        script: i64,
        location: i64,
    },
    // 17
    LeaderBoardControl {
        text: i64,
        unit_type: UnitType,
    },
    // 18
    LeaderBoardControlAtLocation {
        text: i64,
        unit_type: UnitType,
        location: i64,
    },
    // 19
    LeaderBoardResources {
        text: i64,
        resource_type: ResourceType,
    },
    // 20
    LeaderBoardKills {
        text: i64,
        unit_type: UnitType,
    },
    // 21
    LeaderBoardPoints {
        text: i64,
        score_type: ScoreType,
    },
    // 22
    KillAllUnits {
        player: Group,
        unit_type: UnitType,
    },
    // 23
    KillUnitAtLocation {
        player: Group,
        unit_type: UnitType,
        number: i64,
        location: i64,
    },
    // 24
    RemoveAllUnits {
        player: Group,
        unit_type: UnitType,
    },
    // 25
    RemoveUnitAtLocation {
        player: Group,
        unit_type: UnitType,
        number: i64,
        location: i64,
    },
    // 26
    SetResources {
        player: Group,
        number: i64,
        modifier: NumberModifier,
        resource_type: ResourceType,
    },
    // 27
    SetScore {
        player: Group,
        number: i64,
        modifier: NumberModifier,
        score_type: ScoreType,
    },
    // 28
    MinimapPing {
        location: i64,
    },
    // 29
    TalkingPortrait {
        unit_type: UnitType,
        time: i64,
    },
    // 30
    MuteUnitSpeech,
    // 31
    UnmuteUnitSpeech,
    // 32
    LeaderBoardComputerPlayers {
        action: ActionState,
    },
    // 33
    LeaderBoardGoalControl {
        text: i64,
        unit_type: UnitType,
    },
    // 34
    LeaderBoardGoalControlAtLocation {
        text: i64,
        unit_type: UnitType,
        location: i64,
    },
    // 35
    LeaderBoardGoalResources {
        text: i64,
        resource_type: ResourceType,
    },
    // 36
    LeaderBoardGoalKills {
        text: i64,
        unit_type: UnitType,
    },
    // 37
    LeaderBoardGoalPoints {
        text: i64,
        score_type: ScoreType,
    },
    // 38
    MoveLocation {
        player: Group,
        unit_type: UnitType,
        source_location: i64,
        destination_location: i64,
    },
    // 39
    MoveUnit {
        player: Group,
        unit_type: UnitType,
        number: i64,
        source_location: i64,
        destination_location: i64,
    },
    // 40
    LeaderboardGreed {
        number: i64,
    },
    // 41
    SetNextScenario {
        text: i64,
    },
    // 42
    SetDoodadState {
        player: Group,
        unit_type: UnitType,
        location: i64,
        state: ActionState,
    },
    // 43
    SetInvincibility {
        player: Group,
        unit_type: UnitType,
        location: i64,
        state: ActionState,
    },
    // 44
    CreateUnit {
        player: Group,
        unit_type: UnitType,
        number: i64,
        location: i64,
    },
    // 45
    SetDeaths {
        player: Group,
        unit_type: UnitType,
        number: i64,
        modifier: NumberModifier,
    },
    // 46
    Order {
        player: Group,
        unit_type: UnitType,
        source_location: i64,
        destination_location: i64,
        order: Order,
    },
    // 47
    Comment {
        text: i64,
    },
    // 48
    GiveUnitsToPlayer {
        source_player: Group,
        destination_player: Group,
        unit_type: UnitType,
        number: i64,
        location: i64,
    },
    // 49
    ModifyUnitHitPoints {
        player: Group,
        unit_type: UnitType,
        number: i64,
        mod_ammount: i64,
        location: i64,
    },
    // 50
    ModifyUnitEnergy {
        player: Group,
        unit_type: UnitType,
        number: i64,
        mod_ammount: i64,
        location: i64,
    },
    // 51
    ModifyUnitShieldPoints {
        player: Group,
        unit_type: UnitType,
        number: i64,
        mod_ammount: i64,
        location: i64,
    },
    // 52
    ModifyUnitResource {
        player: Group,
        unit_type: UnitType,
        number: i64,
        mod_ammount: i64,
        location: i64,
    },
    // 53
    ModifyUnitHangarCount {
        player: Group,
        unit_type: UnitType,
        number: i64,
        mod_ammount: i64,
        location: i64,
    },
    // 54
    PauseTimer,
    // 55
    UnpauseTimer,
    // 56
    Draw,
    // 57
    SetAllianceStatus {
        player: Group,
        alliance_status: AllianceStatus,
    },
    // 58
    DisableDebugMode,
    // 59
    EnableDebugMode,
}

#[derive(Debug, Serialize)]
pub struct Trigger {
    pub conditions: Vec<Condition>,
    pub actions: Vec<Action>,
    pub execution_flags: u32,
    pub activated_for_players: [u8; 27],
    pub index_of_current_action: u8,
}

pub fn parse_triggers(map: &HashMap<ChunkName, ParsedChunk>) -> Vec<Trigger> {
    let mut ret = Vec::new();

    if let Some(ParsedChunk::TRIG(trig)) = map.get(&ChunkName::TRIG) {
        for trigger in trig.triggers {
            let mut conditions = Vec::new();
            for condition in trigger.conditions {
                match condition.condition {
                    0 => {
                        //conditions.push(Condition::NoCondition)
                    }
                    1 => {
                        conditions.push(Condition::CountdownTimer {
                            comparison: parse_numeric_comparison(
                                condition.numeric_comparison_or_switch_state,
                            ),
                            number: condition.qualified_number as i64,
                        });
                    }
                    2 => {
                        conditions.push(Condition::Command {
                            player: parse_group(condition.group),
                            comparison: parse_numeric_comparison(
                                condition.numeric_comparison_or_switch_state,
                            ),
                            unit_type: parse_unit_type(condition.unit_id),
                            number: condition.qualified_number as i64,
                        });
                    }
                    3 => {
                        conditions.push(Condition::Bring {
                            player: parse_group(condition.group),
                            comparison: parse_numeric_comparison(
                                condition.numeric_comparison_or_switch_state,
                            ),
                            unit_type: parse_unit_type(condition.unit_id),
                            location: condition.location as i64,
                            number: condition.qualified_number as i64,
                        });
                    }
                    4 => {
                        conditions.push(Condition::Accumulate {
                            player: parse_group(condition.group),
                            comparison: parse_numeric_comparison(
                                condition.numeric_comparison_or_switch_state,
                            ),
                            unit_type: parse_unit_type(condition.unit_id),
                            number: condition.qualified_number as i64,
                            resource_type: parse_resource_type(
                                condition.resource_type_or_score_type_or_switch_number as u16,
                            ),
                        });
                    }
                    5 => {
                        conditions.push(Condition::Kill {
                            player: parse_group(condition.group),
                            comparison: parse_numeric_comparison(
                                condition.numeric_comparison_or_switch_state,
                            ),
                            unit_type: parse_unit_type(condition.unit_id),
                            number: condition.qualified_number as i64,
                        });
                    }
                    6 => {
                        conditions.push(Condition::CommandsTheMost {
                            unit_type: parse_unit_type(condition.unit_id),
                        });
                    }
                    7 => {
                        conditions.push(Condition::CommandsTheMostAt {
                            unit_type: parse_unit_type(condition.unit_id),
                            location: condition.location as i64,
                        });
                    }
                    8 => {
                        conditions.push(Condition::MostKills {
                            unit_type: parse_unit_type(condition.unit_id),
                        });
                    }
                    9 => {
                        conditions.push(Condition::HighestScore {
                            score_type: parse_score_type(
                                condition.resource_type_or_score_type_or_switch_number as u16,
                            ),
                        });
                    }
                    10 => {
                        conditions.push(Condition::MostResources {
                            resource_type: parse_resource_type(
                                condition.resource_type_or_score_type_or_switch_number as u16,
                            ),
                        });
                    }
                    11 => {
                        conditions.push(Condition::Switch {
                            switch: condition.resource_type_or_score_type_or_switch_number as i64,
                            switch_state: parse_switch_state(
                                condition.numeric_comparison_or_switch_state,
                            ),
                        });
                    }
                    12 => {
                        conditions.push(Condition::ElapsedTime {
                            comparison: parse_numeric_comparison(
                                condition.numeric_comparison_or_switch_state,
                            ),
                            number: condition.qualified_number as i64,
                        });
                    }
                    13 => {
                        conditions.push(Condition::DataIsAMissionBriefing);
                    }
                    14 => {
                        conditions.push(Condition::Opponents {
                            player: parse_group(condition.group),
                            comparison: parse_numeric_comparison(
                                condition.numeric_comparison_or_switch_state,
                            ),
                            number: condition.qualified_number as i64,
                        });
                    }
                    15 => {
                        conditions.push(Condition::Deaths {
                            player: parse_group(condition.group),
                            comparison: parse_numeric_comparison(
                                condition.numeric_comparison_or_switch_state,
                            ),
                            unit_type: parse_unit_type(condition.unit_id),
                            number: condition.qualified_number as i64,
                        });
                    }
                    16 => {
                        conditions.push(Condition::CommandsTheLeast {
                            unit_type: parse_unit_type(condition.unit_id),
                        });
                    }
                    17 => {
                        conditions.push(Condition::CommandsTheLeastAt {
                            unit_type: parse_unit_type(condition.unit_id),
                            location: condition.location as i64,
                        });
                    }
                    18 => {
                        conditions.push(Condition::LeastKills {
                            unit_type: parse_unit_type(condition.unit_id),
                        });
                    }
                    19 => {
                        conditions.push(Condition::LowestScore {
                            score_type: parse_score_type(
                                condition.resource_type_or_score_type_or_switch_number as u16,
                            ),
                        });
                    }
                    20 => {
                        conditions.push(Condition::LeastResources {
                            resource_type: parse_resource_type(
                                condition.resource_type_or_score_type_or_switch_number as u16,
                            ),
                        });
                    }
                    21 => {
                        conditions.push(Condition::Score {
                            player: parse_group(condition.group),
                            comparison: parse_numeric_comparison(
                                condition.numeric_comparison_or_switch_state,
                            ),
                            score_type: parse_score_type(
                                condition.resource_type_or_score_type_or_switch_number as u16,
                            ),
                            number: condition.qualified_number as i64,
                        });
                    }
                    22 => {
                        conditions.push(Condition::Always);
                    }
                    23 => {
                        conditions.push(Condition::Never);
                    }
                    _ => {
                        conditions.push(Condition::Unknown {
                            id: condition.condition as i64,
                            raw: condition,
                        });
                    }
                }
            }

            let mut actions = Vec::new();

            for action in trigger.actions {
                match action.action {
                    0 => {}
                    1 => {
                        actions.push(Action::Victory);
                    }
                    2 => {
                        actions.push(Action::Defeat);
                    }
                    3 => {
                        actions.push(Action::PreserveTrigger);
                    }
                    4 => {
                        actions.push(Action::Wait {
                            time: action.seconds_or_milliseconds as i64,
                        });
                    }
                    5 => {
                        actions.push(Action::PauseGame);
                    }
                    6 => {
                        actions.push(Action::UnpauseGame);
                    }
                    7 => {
                        actions.push(Action::Transmission {
                            text: action.string_number as i64,
                            unit_type: parse_unit_type(action
                                .unit_type_or_score_type_or_resource_type_or_alliance_status),
                            location: action.location as i64,
                            time: action.seconds_or_milliseconds as i64,
                            modifier: parse_number_modifier(
                                action.number_of_units_or_action_state_or_unit_order_or_number_modifier,
                            ),
                            wave: action.wav_string_number as i64,
                            // TODO: not sure what this is supposed to be.
                            wave_time: 0,
                        });
                    }
                    8 => {
                        actions.push(Action::PlayWav {
                            wave: action.wav_string_number as i64,
                            // TODO: not sure what this is supposed to be.
                            wave_time: 0,
                        });
                    }
                    9 => {
                        actions.push(Action::DisplayTextMessage {
                            text: action.string_number as i64,
                        });
                    }
                    10 => {
                        actions.push(Action::CenterView {
                            location: action.location as i64,
                        });
                    }
                    11 => {
                        actions.push(Action::CreateUnitWithProperties {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(action
                                .unit_type_or_score_type_or_resource_type_or_alliance_status),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            location: action.location as i64,
                            unit_prop: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                        });
                    }
                    12 => {
                        actions.push(Action::SetMissionObjectives {
                            text: action.string_number as i64,
                        });
                    }
                    13 => {
                        actions.push(Action::SetSwitch {
                            switch: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            switch_action: parse_action_state(action.number_of_units_or_action_state_or_unit_order_or_number_modifier),
                        });
                    }
                    14 => {
                        actions.push(Action::SetCountdownTimer {
                        time: action.seconds_or_milliseconds as i64,
                        modifier: parse_number_modifier(
                            action.number_of_units_or_action_state_or_unit_order_or_number_modifier,
                        ),
                    });
                    }
                    15 => {
                        actions.push(Action::RunAIScript {
                        script: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                    });
                    }
                    16 => {
                        actions.push(Action::RunAIScriptAtLocation {
                            script: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            location: action.location as i64,
                        });
                    }
                    17 => {
                        actions.push(Action::LeaderBoardControl {
                            text: action.string_number as i64,
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    18 => {
                        actions.push(Action::LeaderBoardControlAtLocation {
                            text: action.string_number as i64,
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                            location: action.location as i64,
                        });
                    }
                    19 => {
                        actions.push(Action::LeaderBoardResources {
                            text: action.string_number as i64,
                            resource_type: parse_resource_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    20 => {
                        actions.push(Action::LeaderBoardKills {
                            text: action.string_number as i64,
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    21 => {
                        actions.push(Action::LeaderBoardPoints {
                            text: action.string_number as i64,
                            score_type: parse_score_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    22 => {
                        actions.push(Action::KillAllUnits {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    23 => {
                        actions.push(Action::KillUnitAtLocation {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            location: action.location as i64,
                        });
                    }
                    24 => {
                        actions.push(Action::RemoveAllUnits {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    25 => {
                        actions.push(Action::RemoveUnitAtLocation {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            location: action.location as i64,
                        });
                    }
                    26 => {
                        actions.push(Action::SetResources {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            modifier: parse_number_modifier(action.number_of_units_or_action_state_or_unit_order_or_number_modifier),
                            resource_type: parse_resource_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    27 => {
                        actions.push(Action::SetScore {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            modifier: parse_number_modifier(action.number_of_units_or_action_state_or_unit_order_or_number_modifier),
                            score_type: parse_score_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    28 => {
                        actions.push(Action::MinimapPing {
                            location: action.location as i64,
                        });
                    }
                    29 => {
                        actions.push(Action::TalkingPortrait {
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                            time: action.seconds_or_milliseconds as i64,
                        });
                    }
                    30 => {
                        actions.push(Action::MuteUnitSpeech);
                    }
                    31 => {
                        actions.push(Action::UnmuteUnitSpeech);
                    }
                    32 => {
                        actions.push(Action::LeaderBoardComputerPlayers {
                            action: parse_action_state(action.number_of_units_or_action_state_or_unit_order_or_number_modifier),
                        });
                    }
                    33 => {
                        actions.push(Action::LeaderBoardGoalControl {
                            text: action.string_number as i64,
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    34 => {
                        actions.push(Action::LeaderBoardGoalControlAtLocation {
                            text: action.string_number as i64,
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                            location: action.location as i64,
                        });
                    }
                    35 => {
                        actions.push(Action::LeaderBoardGoalResources {
                            text: action.string_number as i64,
                            resource_type: parse_resource_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    36 => {
                        actions.push(Action::LeaderBoardGoalKills {
                            text: action.string_number as i64,
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    37 => {
                        actions.push(Action::LeaderBoardGoalPoints {
                            text: action.string_number as i64,
                            score_type: parse_score_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    38 => {
                        actions.push(Action::MoveLocation {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                            source_location: action.location as i64,
                            destination_location: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                        });
                    }
                    39 => {
                        actions.push(Action::MoveUnit {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                            source_location: action.location as i64,
                            destination_location: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                        });
                    }
                    40 => {
                        actions.push(Action::LeaderboardGreed {
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                        });
                    }
                    41 => {
                        actions.push(Action::SetNextScenario {
                            text: action.string_number as i64,
                        });
                    }
                    42 => {
                        actions.push(Action::SetDoodadState {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                            location: action.location as i64,
                            state: parse_action_state(action.number_of_units_or_action_state_or_unit_order_or_number_modifier), // TODO: split actionstate enum into doodad state + switch state + computer-player state.
                        });
                    }
                    43 => {
                        actions.push(Action::SetInvincibility {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                            location: action.location as i64,
                            state: parse_action_state(action.number_of_units_or_action_state_or_unit_order_or_number_modifier), // TODO: split actionstate enum into doodad state + switch state + computer-player state.
                        });
                    }
                    44 => {
                        actions.push(Action::CreateUnit {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            location: action.location as i64,
                        });
                    }
                    45 => {
                        actions.push(Action::SetDeaths {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(action
                                .unit_type_or_score_type_or_resource_type_or_alliance_status),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            modifier: parse_number_modifier(action.number_of_units_or_action_state_or_unit_order_or_number_modifier),
                        });
                    }
                    46 => {
                        actions.push(Action::Order {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(action
                                .unit_type_or_score_type_or_resource_type_or_alliance_status),
                            source_location: action.location as i64,
                            destination_location: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            order: parse_order(action.number_of_units_or_action_state_or_unit_order_or_number_modifier)
                        });
                    }
                    47 => {
                        actions.push(Action::Comment {
                            text: action.string_number as i64,
                        });
                    }
                    48 => {
                        actions.push(Action::GiveUnitsToPlayer {
                            source_player: parse_group(action.first_or_only_group_or_player_affected),
                            destination_player: parse_group(action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number),
                            unit_type: parse_unit_type(action
                                .unit_type_or_score_type_or_resource_type_or_alliance_status),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            location: action.location as i64,
                        });
                    }
                    49 => {
                        actions.push(Action::ModifyUnitHitPoints {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(action
                                .unit_type_or_score_type_or_resource_type_or_alliance_status),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            mod_ammount: 0, // TODO: what is this.
                            location: action.location as i64,
                        });
                    }
                    50 => {
                        actions.push(Action::ModifyUnitEnergy {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(action
                                .unit_type_or_score_type_or_resource_type_or_alliance_status),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            mod_ammount: 0, // TODO: what is this.
                            location: action.location as i64,
                        });
                    }
                    51 => {
                        actions.push(Action::ModifyUnitShieldPoints {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(action
                                .unit_type_or_score_type_or_resource_type_or_alliance_status),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            mod_ammount: 0, // TODO: what is this.
                            location: action.location as i64,
                        });
                    }
                    52 => {
                        actions.push(Action::ModifyUnitResource { // TOOD: ModifyUnitResourceAmount
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(action
                                .unit_type_or_score_type_or_resource_type_or_alliance_status),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            mod_ammount: 0, // TODO: what is this.
                            location: action.location as i64,
                        });
                    }
                    53 => {
                        actions.push(Action::ModifyUnitHangarCount {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            unit_type: parse_unit_type(action
                                .unit_type_or_score_type_or_resource_type_or_alliance_status),
                            number: action.second_group_affected_or_secondary_location_or_cuwp_number_or_number_or_ai_script_or_switch_number as i64,
                            mod_ammount: 0, // TODO: what is this.
                            location: action.location as i64,
                        });
                    }
                    54 => {
                        actions.push(Action::PauseTimer);
                    }
                    55 => {
                        actions.push(Action::UnpauseTimer);
                    }
                    56 => {
                        actions.push(Action::Draw);
                    }
                    57 => {
                        actions.push(Action::SetAllianceStatus {
                            player: parse_group(action.first_or_only_group_or_player_affected),
                            alliance_status: parse_alliance_status(
                                action.unit_type_or_score_type_or_resource_type_or_alliance_status,
                            ),
                        });
                    }
                    58 => {
                        actions.push(Action::DisableDebugMode);
                    }
                    59 => {
                        actions.push(Action::EnableDebugMode);
                    }
                    _ => {
                        actions.push(Action::Unknown {
                            id: action.action as i64,
                            raw: action,
                        });
                    }
                }
            }

            ret.push(Trigger {
                conditions,
                actions,
                execution_flags: trigger.execution_flags,
                activated_for_players: trigger.executed_for_player,
                index_of_current_action: trigger.current_action,
            });
        }
    }

    ret
}
