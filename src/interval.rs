use crate::{pitch::{PitchOrder, Pitches}, notes::Notes};
use super::pitch::PitchOctave;
use crate::error::{Error, Result};
use strum::EnumString;


pub struct Interval {
    pub root: PitchOctave,
    pub kind: IntervalType,
    pub pitch_order: PitchOrder,
}

impl Interval {
    pub fn new(root: PitchOctave, kind: IntervalType) -> Self {
        Self {
            root,
            kind,
            pitch_order: PitchOrder::Ascending,
        }
    }
    pub fn new_with_order(root: PitchOctave, kind: IntervalType, pitch_order: PitchOrder) -> Self {
        Self {
            root,
            kind,
            pitch_order,
        }
    }
    fn gen_notes(&self) -> Result<Pitches> {
        let semitone = self.kind.get_semitone_value();
        let mut interval = Pitches(vec![self.root]);

        match self.pitch_order {
            PitchOrder::Ascending => {
                if let Some(tone) = self.root.checked_add(IntervalType::from(semitone)) {
                    interval.0.push(tone);
                } else {
                    return Err(Error::OutofBounds);
                }
            }
            PitchOrder::Descending => {
                if let Some(tone) = self.root.checked_sub(IntervalType::from(semitone)) {
                    interval.0.push(tone);
                } else {
                    return Err(Error::OutofBounds);
                }
            }
        }
        Ok(interval)
    }
}

impl Notes for Interval {
    fn notes(&self) -> Result<Pitches> {
        self.gen_notes()
    }
}

pub enum Quality {
    Perfect,
    Major,
    Minor,
    Augmented,
    Diminished,
}

pub type NumericIntervalsSlice = [u8];

#[derive(Debug, Copy, Clone, PartialEq, EnumString)]
pub enum IntervalType {
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

impl IntervalType {
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

    pub fn get_diatonic_value(&self) -> u8 {
        match self {
            IntervalType::Unison => Self::UNISON_DIATONIC_INC,
            IntervalType::AugmentedSecond
            | IntervalType::DiminishedSecond
            | IntervalType::MinorSecond
            | IntervalType::MajorSecond => Self::SECOND_DIATONIC_INC,
            IntervalType::DiminishedThird
            | IntervalType::AugmentedThird
            | IntervalType::MajorThird
            | IntervalType::MinorThird => Self::THIRD_DIATONIC_INC,
            IntervalType::AugmentedFourth | IntervalType::DiminishedFourth | IntervalType::Fourth => {
                Self::FOURTH_DIATONIC_INC
            }
            IntervalType::AugmentedFifth | IntervalType::DiminishedFifth | IntervalType::Fifth => {
                Self::FIFTH_DIATONIC_INC
            }
            IntervalType::AugmentedSixth
            | IntervalType::DiminishedSixth
            | IntervalType::MinorSixth
            | IntervalType::MajorSixth => Self::SIXTH_DIATONIC_INC,
            IntervalType::DiminishedSeventh
            | IntervalType::MinorSeventh
            | IntervalType::MajorSeventh
            | IntervalType::AugmentedSeventh => Self::SEVENTH_DIATONIC_INC,
            IntervalType::Octave | IntervalType::DiminishedEighth => Self::EIGHTH_DIATONIC_INC,
        }
    }

    pub fn get_quality(&self) -> Quality {
        match self {
            IntervalType::Unison | IntervalType::Fourth | IntervalType::Fifth | IntervalType::Octave => {
                Quality::Perfect
            }
            IntervalType::DiminishedSecond
            | IntervalType::DiminishedThird
            | IntervalType::DiminishedFourth
            | IntervalType::DiminishedFifth
            | IntervalType::DiminishedSixth
            | IntervalType::DiminishedSeventh
            | IntervalType::DiminishedEighth => Quality::Diminished,
            IntervalType::MajorSecond
            | IntervalType::MajorThird
            | IntervalType::MajorSixth
            | IntervalType::MajorSeventh => Quality::Major,
            IntervalType::AugmentedFifth
            | IntervalType::AugmentedThird
            | IntervalType::AugmentedSixth
            | IntervalType::AugmentedSecond
            | IntervalType::AugmentedFourth
            | IntervalType::AugmentedSeventh => Quality::Augmented,
            IntervalType::MinorSecond
            | IntervalType::MinorThird
            | IntervalType::MinorSixth
            | IntervalType::MinorSeventh => Quality::Minor,
        }
    }
    pub fn get_semitone_value(&self) -> u8 {
        match self {
            IntervalType::Unison => Self::UNISON_INTERVAL_SEMITONES,
            IntervalType::DiminishedSecond => Self::UNISON_INTERVAL_SEMITONES,
            IntervalType::MinorSecond => Self::MINSECOND_INTERVAL_SEMITONES,
            IntervalType::MajorSecond => Self::MAJSECOND_INTERVAL_SEMITONES,
            IntervalType::DiminishedThird => Self::MAJSECOND_INTERVAL_SEMITONES,
            IntervalType::AugmentedSecond => Self::MINTHIRD_INTERVAL_SEMITONES,
            IntervalType::AugmentedThird => Self::PERFECT_FOURTH_INTERVAL_SEMITONES,
            IntervalType::MinorThird => Self::MINTHIRD_INTERVAL_SEMITONES,
            IntervalType::MajorThird => Self::MAJTHIRD_INTERVAL_SEMITONES,
            IntervalType::DiminishedFourth => Self::MAJTHIRD_INTERVAL_SEMITONES,
            IntervalType::Fourth => Self::PERFECT_FOURTH_INTERVAL_SEMITONES,
            IntervalType::AugmentedFourth => Self::AUGMENTED_FOURTH_INTERVAL_SEMITONES,
            IntervalType::DiminishedFifth => Self::AUGMENTED_FOURTH_INTERVAL_SEMITONES,
            IntervalType::Fifth => Self::PERFECT_FIFTH_INTERVAL_SEMITONES,
            IntervalType::DiminishedSixth => Self::PERFECT_FIFTH_INTERVAL_SEMITONES,
            IntervalType::AugmentedFifth => Self::MINOR_SIXTH_INTERVAL_SEMITONES,
            IntervalType::MinorSixth => Self::MINOR_SIXTH_INTERVAL_SEMITONES,
            IntervalType::MajorSixth => Self::MAJOR_SIXTH_INTERVAL_SEMITONES,
            IntervalType::AugmentedSixth => Self::MAJOR_SIXTH_INTERVAL_SEMITONES,
            IntervalType::DiminishedSeventh => Self::MAJOR_SIXTH_INTERVAL_SEMITONES,
            IntervalType::MinorSeventh => Self::MINOR_SEVENTH_INTERVAL_SEMITONES,
            IntervalType::MajorSeventh => Self::MAJOR_SEVENTH_INTERVAL_SEMITONES,
            IntervalType::DiminishedEighth => Self::MAJOR_SEVENTH_INTERVAL_SEMITONES,
            IntervalType::AugmentedSeventh => Self::OCTAVE_INTERVAL_SEMITONES,
            IntervalType::Octave => Self::OCTAVE_INTERVAL_SEMITONES,
        }
    }

}

impl From<u8> for IntervalType {
    fn from(value: u8) -> Self {
        match value {
            Self::UNISON_INTERVAL_SEMITONES => IntervalType::Unison,
            Self::MINSECOND_INTERVAL_SEMITONES => IntervalType::MinorSecond,
            Self::MAJSECOND_INTERVAL_SEMITONES => IntervalType::MajorSecond,
            Self::MINTHIRD_INTERVAL_SEMITONES => IntervalType::MinorThird,
            Self::MAJTHIRD_INTERVAL_SEMITONES => IntervalType::MajorThird,
            Self::PERFECT_FOURTH_INTERVAL_SEMITONES => IntervalType::Fourth,
            Self::AUGMENTED_FOURTH_INTERVAL_SEMITONES => IntervalType::AugmentedFourth,
            Self::PERFECT_FIFTH_INTERVAL_SEMITONES => IntervalType::Fifth,
            Self::MINOR_SIXTH_INTERVAL_SEMITONES => IntervalType::MinorSixth,
            Self::MAJOR_SIXTH_INTERVAL_SEMITONES => IntervalType::MajorSixth,
            Self::MINOR_SEVENTH_INTERVAL_SEMITONES => IntervalType::MinorSeventh,
            Self::MAJOR_SEVENTH_INTERVAL_SEMITONES => IntervalType::MajorSeventh,
            Self::OCTAVE_INTERVAL_SEMITONES => IntervalType::Octave,
            _ => panic!("Unsupported interval"),
        }
    }
}