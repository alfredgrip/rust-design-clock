use std::collections::HashSet;

pub const CLOCK_TEXT_DIM: (u32, u32) = (10, 11);

lazy_static! {
    pub static ref CLOCK_TEXT: Vec<Vec<char>> = 
        vec![
            vec!['K','L','O','C','K','A','N','T','Ä','R','K'],
            vec!['F','E','M','Y','I','S','T','I','O','N','I'],
            vec!['K','V','A','R','T','Q','I','E','N','Z','O'],
            vec!['T','J','U','G','O','L','I','V','I','P','M'],
            vec!['Ö','V','E','R','K','A','M','H','A','L','V'],
            vec!['E','T','T','U','S','V','L','X','T','V','Å'],
            vec!['T','R','E','M','Y','K','Y','F','Y','R','A'],
            vec!['F','E','M','S','F','L','O','R','S','E','X'],
            vec!['S','J','U','Å','T','T','A','I','N','I','O'],
            vec!['T','I','O','E','L','V','A','T','O','L','V']
        ];
    /*
    String::from("KLOCKANTÄRK
    FEMYISTIONI
    KVARTQIENZO
    TJUGOLIVIPM
    ÖVERKAMHALV
    ETTUSVLXTVÅ
    TREMYKYFYRA
    FEMSFLORSEX
    SJUÅTTAINIO
    TIOELVATOLV");
    */
    pub static ref TIME_IS_IDXS: HashSet<(u32, u32)> = HashSet::from([(0, 0), (0, 1), (0, 2), (0, 3), (0, 4), (0, 5), (0, 6), (0, 8), (0, 9)]);
    static ref PAST_IDXS: HashSet<(u32, u32)> = HashSet::from([(4, 0), (4, 1), (4, 2), (4, 3)]);
    static ref TO_IDXS: HashSet<(u32, u32)> = HashSet::from([(3, 6)]);
}

pub enum MinuteStates {
    OClock,
    FivePast,
    TenPast,
    QuarterPast,
    TwentyPast,
    TwentyFivePast,  // fem i halv
    HalfPast,
    TwentyFiveTo,    // fem över halv
    TwentyTo,
    QuarterTo,
    TenTo,
    FiveTo,
}

impl MinuteStates {
    pub fn from_minutes(minutes: u32) -> MinuteStates {
        // round downwards to nearest 5 minutes
        let rounded_minutes: u32 = minutes - (minutes % 5);
        match rounded_minutes {
            0 => MinuteStates::OClock,
            5 => MinuteStates::FivePast,
            10 => MinuteStates::TenPast,
            15 => MinuteStates::QuarterPast,
            20 => MinuteStates::TwentyPast,
            25 => MinuteStates::TwentyFivePast,
            30 => MinuteStates::HalfPast,
            35 => MinuteStates::TwentyFiveTo,
            40 => MinuteStates::TwentyTo,
            45 => MinuteStates::QuarterTo,
            50 => MinuteStates::TenTo,
            55 => MinuteStates::FiveTo,
            _ => panic!("Invalid minutes: {}", minutes),
        }
    }

    pub fn state_to_idx(&self) -> HashSet<(u32, u32)> {
        match self {
            MinuteStates::OClock => HashSet::new(),
            MinuteStates::FivePast => HashSet::from([(1, 0), (1, 1), (1, 2)]).union(&PAST_IDXS).cloned().collect(),
            MinuteStates::TenPast => HashSet::from([(1, 6), (1, 7), (1, 8)]).union(&PAST_IDXS).cloned().collect(),
            MinuteStates::QuarterPast => HashSet::from([(2, 0), (2, 1), (2, 2), (2, 3), (2, 4)]).union(&PAST_IDXS).cloned().collect(),
            MinuteStates::TwentyPast => HashSet::from([(3, 0), (3, 1), (3, 2), (3, 3), (3, 4)]).union(&PAST_IDXS).cloned().collect(),
            MinuteStates::TwentyFivePast => MinuteStates::state_to_idx(&MinuteStates::FiveTo).union(&MinuteStates::state_to_idx(&MinuteStates::HalfPast)).cloned().collect(),
            MinuteStates::HalfPast => HashSet::from([(4, 7), (4, 8), (4, 9), (4, 10)]),
            MinuteStates::TwentyFiveTo => MinuteStates::state_to_idx(&MinuteStates::FivePast).union(&MinuteStates::state_to_idx(&MinuteStates::HalfPast)).cloned().collect(),
            MinuteStates::TwentyTo => HashSet::from([(3, 0), (3, 1), (3, 2), (3, 3), (3, 4)]).union(&TO_IDXS).cloned().collect(),
            MinuteStates::QuarterTo => HashSet::from([(2, 0), (2, 1), (2, 2), (2, 3), (2, 4)]).union(&TO_IDXS).cloned().collect(),
            MinuteStates::TenTo => HashSet::from([(1, 6), (1, 7), (1, 8)]).union(&TO_IDXS).cloned().collect(),
            MinuteStates::FiveTo => HashSet::from([(1, 0), (1, 1), (1, 2)]).union(&TO_IDXS).cloned().collect(),
        }
    }
}

pub enum HourStates {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Eleven,
    Twelve
}

impl HourStates {
    pub fn from_hours(hours: u32) -> HourStates {
        match hours {
            1 => HourStates::One,
            2 => HourStates::Two,
            3 => HourStates::Three,
            4 => HourStates::Four,
            5 => HourStates::Five,
            6 => HourStates::Six,
            7 => HourStates::Seven,
            8 => HourStates::Eight,
            9 => HourStates::Nine,
            10 => HourStates::Ten,
            11 => HourStates::Eleven,
            12 => HourStates::Twelve,
            _ => panic!("Invalid hours: {}", hours),
        }
    }

    pub fn hour_idxs(&self) -> HashSet<(u32, u32)> {
        match self {
            HourStates::One => HashSet::from([(5, 0), (5, 1), (5, 2)]),
            HourStates::Two => HashSet::from([(5, 8), (5, 9), (5, 10)]),
            HourStates::Three => HashSet::from([(6, 0), (6, 1), (6, 2)]),
            HourStates::Four => HashSet::from([(6, 7), (6, 8), (6, 9), (6, 10)]),
            HourStates::Five => HashSet::from([(7, 0), (7, 1), (7, 2)]),
            HourStates::Six => HashSet::from([(7, 8), (7, 9), (7, 10)]),
            HourStates::Seven => HashSet::from([(8, 0), (8, 1), (8, 2)]),
            HourStates::Eight => HashSet::from([(8, 3), (8, 4), (8, 5), (8, 6)]),
            HourStates::Nine => HashSet::from([(8, 8), (8, 9), (8, 10)]),
            HourStates::Ten => HashSet::from([(9, 0), (9, 1), (9, 2)]),
            HourStates::Eleven => HashSet::from([(9, 3), (9, 4), (9, 5), (9, 6)]),
            HourStates::Twelve => HashSet::from([(9, 7), (9, 8), (9, 9), (9, 10)]),
        }
    }
}