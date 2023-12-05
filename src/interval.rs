
pub enum Quality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

pub type NumericIntervalsSlice = [u8];

pub enum Interval {
    Unison,
    DiminishedSecond,
    MinorSecond,
    MajorSecond,
    DiminishedThird,
    AugmentedSecond,
    MinorThird,
    MajorThird,
    DiminishedFourth,
    AugmentedThird,
    Fourth,
    AugmentedFourth,
    DiminishedFifth,
    Fifth,
    DiminishedSixth,
    AugmentedFifth,
    MinorSixth,
    MajorSixth,
    DiminishedSeventh,
    AugmentedSixth,
    MinorSeventh,
    MajorSeventh,
    DiminishedEighth,
    AugmentedSeventh,
    Octave,
}

impl Interval {
    pub const UNISON_DIATONIC_INC: u8 = 0;
    pub const SECOND_DIATONIC_INC: u8 = 1; // C,D or E,F would be 1 diatonic increments
    pub const THIRD_DIATONIC_INC: u8 = 2; // E,G or F,A would be 2 diatonic increments
    pub const FOURTH_DIATONIC_INC: u8 = 3;
    pub const FIFTH_DIATONIC_INC: u8 = 4;
    pub const SIXTH_DIATONIC_INC: u8 = 5;
    pub const SEVENTH_DIATONIC_INC: u8 = 6;
    pub const EIGHTH_DIATONIC_INC: u8 = 7;

    const UNISON_INTERVAL_SEMITONES: u8 = 0;
    const MINSECOND_INTERVAL_SEMITONES: u8 = 1;
    const MAJSECOND_INTERVAL_SEMITONES: u8 = 2;
    const MINTHIRD_INTERVAL_SEMITONES: u8 = 3;
    const MAJTHIRD_INTERVAL_SEMITONES: u8 = 4;
    const PERFECT_FOURTH_INTERVAL_SEMITONES: u8 = 5;
    const AUGMENTED_FOURTH_INTERVAL_SEMITONES: u8 = 6;
    const PERFECT_FIFTH_INTERVAL_SEMITONES: u8 = 7;
    const MINOR_SIXTH_INTERVAL_SEMITONES: u8 = 8;
    const MAJOR_SIXTH_INTERVAL_SEMITONES: u8 = 9;
    const MINOR_SEVENTH_INTERVAL_SEMITONES: u8 = 10;
    const MAJOR_SEVENTH_INTERVAL_SEMITONES: u8 = 11;
    const OCTAVE_INTERVAL_SEMITONES: u8 = 12;

    pub fn get_diatonic_increment(&self) -> u8 {
        match self {
            Interval::Unison => Self::UNISON_DIATONIC_INC,
            Interval::AugmentedSecond
            | Interval::DiminishedSecond
            | Interval::MinorSecond
            | Interval::MajorSecond => Self::SECOND_DIATONIC_INC,
            Interval::DiminishedThird
            | Interval::AugmentedThird
            | Interval::MajorThird
            | Interval::MinorThird => Self::THIRD_DIATONIC_INC,
            Interval::AugmentedFourth | Interval::DiminishedFourth | Interval::Fourth => {
                Self::FOURTH_DIATONIC_INC
            }
            Interval::AugmentedFifth | Interval::DiminishedFifth | Interval::Fifth => {
                Self::FIFTH_DIATONIC_INC
            }
            Interval::AugmentedSixth
            | Interval::DiminishedSixth
            | Interval::MinorSixth
            | Interval::MajorSixth => Self::SIXTH_DIATONIC_INC,
            Interval::DiminishedSeventh
            | Interval::MinorSeventh
            | Interval::MajorSeventh
            | Interval::AugmentedSeventh => Self::SEVENTH_DIATONIC_INC,
            Interval::Octave | Interval::DiminishedEighth => Self::EIGHTH_DIATONIC_INC,
        }
    }

    pub fn get_quality(&self) -> Quality {
        match self {
            Interval::Unison | Interval::Fourth | Interval::Fifth | Interval::Octave => {
                Quality::Perfect
            }
            Interval::DiminishedSecond
            | Interval::DiminishedThird
            | Interval::DiminishedFourth
            | Interval::DiminishedFifth
            | Interval::DiminishedSixth
            | Interval::DiminishedSeventh
            | Interval::DiminishedEighth => Quality::Diminished,
            Interval::MajorSecond
            | Interval::MajorThird
            | Interval::MajorSixth
            | Interval::MajorSeventh => Quality::Major,
            Interval::AugmentedFifth
            | Interval::AugmentedThird
            | Interval::AugmentedSixth
            | Interval::AugmentedSecond
            | Interval::AugmentedFourth
            | Interval::AugmentedSeventh => Quality::Augmented,
            Interval::MinorSecond
            | Interval::MinorThird
            | Interval::MinorSixth
            | Interval::MinorSeventh => Quality::Minor,
        }
    }
    pub fn get_semitone_value(&self) -> u8 {
        match self {
            Interval::Unison => Self::UNISON_INTERVAL_SEMITONES,
            Interval::DiminishedSecond => Self::UNISON_INTERVAL_SEMITONES,
            Interval::MinorSecond => Self::MINSECOND_INTERVAL_SEMITONES,
            Interval::MajorSecond => Self::MAJSECOND_INTERVAL_SEMITONES,
            Interval::DiminishedThird => Self::MAJSECOND_INTERVAL_SEMITONES,
            Interval::AugmentedSecond => Self::MINTHIRD_INTERVAL_SEMITONES,
            Interval::AugmentedThird => Self::PERFECT_FOURTH_INTERVAL_SEMITONES,
            Interval::MinorThird => Self::MINTHIRD_INTERVAL_SEMITONES,
            Interval::MajorThird => Self::MAJTHIRD_INTERVAL_SEMITONES,
            Interval::DiminishedFourth => Self::MAJTHIRD_INTERVAL_SEMITONES,
            Interval::Fourth => Self::PERFECT_FOURTH_INTERVAL_SEMITONES,
            Interval::AugmentedFourth => Self::AUGMENTED_FOURTH_INTERVAL_SEMITONES,
            Interval::DiminishedFifth => Self::AUGMENTED_FOURTH_INTERVAL_SEMITONES,
            Interval::Fifth => Self::PERFECT_FIFTH_INTERVAL_SEMITONES,
            Interval::DiminishedSixth => Self::PERFECT_FIFTH_INTERVAL_SEMITONES,
            Interval::AugmentedFifth => Self::MINOR_SIXTH_INTERVAL_SEMITONES,
            Interval::MinorSixth => Self::MINOR_SIXTH_INTERVAL_SEMITONES,
            Interval::MajorSixth => Self::MAJOR_SIXTH_INTERVAL_SEMITONES,
            Interval::AugmentedSixth => Self::MAJOR_SIXTH_INTERVAL_SEMITONES,
            Interval::DiminishedSeventh => Self::MAJOR_SIXTH_INTERVAL_SEMITONES,
            Interval::MinorSeventh => Self::MINOR_SEVENTH_INTERVAL_SEMITONES,
            Interval::MajorSeventh => Self::MAJOR_SEVENTH_INTERVAL_SEMITONES,
            Interval::DiminishedEighth => Self::MAJOR_SEVENTH_INTERVAL_SEMITONES,
            Interval::AugmentedSeventh => Self::OCTAVE_INTERVAL_SEMITONES,
            Interval::Octave => Self::OCTAVE_INTERVAL_SEMITONES,
        }
    }

}

impl From<u8> for Interval {
    fn from(value: u8) -> Self {
        match (value) {
            Self::UNISON_INTERVAL_SEMITONES => Interval::Unison,
            Self::MINSECOND_INTERVAL_SEMITONES => Interval::MinorSecond,
            Self::MAJSECOND_INTERVAL_SEMITONES => Interval::MajorSecond,
            Self::MINTHIRD_INTERVAL_SEMITONES => Interval::MinorThird,
            Self::MAJTHIRD_INTERVAL_SEMITONES => Interval::MajorThird,
            Self::PERFECT_FOURTH_INTERVAL_SEMITONES => Interval::Fourth,
            Self::AUGMENTED_FOURTH_INTERVAL_SEMITONES => Interval::AugmentedFourth,
            Self::PERFECT_FIFTH_INTERVAL_SEMITONES => Interval::Fifth,
            Self::MINOR_SIXTH_INTERVAL_SEMITONES => Interval::MinorSixth,
            Self::MAJOR_SIXTH_INTERVAL_SEMITONES => Interval::MajorSixth,
            Self::MINOR_SEVENTH_INTERVAL_SEMITONES => Interval::MinorSeventh,
            Self::MAJOR_SEVENTH_INTERVAL_SEMITONES => Interval::MajorSeventh,
            Self::OCTAVE_INTERVAL_SEMITONES => Interval::Octave,
            _ => panic!("Unsupported interval"),
        }
    }
}