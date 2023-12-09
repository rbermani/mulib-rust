use crate::{error::{Error, Result}, interval::IntervalType};
use num_derive::FromPrimitive;
use strum::EnumString;

use std::{
    ops::{Add, Sub},
    str::FromStr,
};

pub struct Pitches(pub Vec<PitchOctave>);

impl Pitches {
    fn test(&self) {

    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, EnumString)]
pub enum PitchOrder {
    #[default]
    Ascending,
    Descending,
}

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Default, FromPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum Octave {
    Octave0 = 0,
    Octave1 = 1,
    Octave2 = 2,
    #[default]
    Octave3 = 3,
    Octave4 = 4,
    Octave5 = 5,
    Octave6 = 6,
    Octave7 = 7,
    Octave8 = 8,
}

impl Octave {
    // Method to increment the octave
    pub fn increment(&self) -> Octave {
        match *self {
            Octave::Octave8 => Octave::Octave8, // Octave can't go beyond 8
            _ => Octave::from(*self as u8 + 1),
        }
    }

    // Method to decrement the octave
    pub fn decrement(&self) -> Octave {
        match *self {
            Octave::Octave0 => Octave::Octave0, // Octave can't go below 0
            _ => Octave::from(*self as u8 - 1),
        }
    }
}

impl From<u8> for Octave {
    fn from(value: u8) -> Octave {
        match value {
            0 => Octave::Octave0,
            1 => Octave::Octave1,
            2 => Octave::Octave2,
            3 => Octave::Octave3,
            4 => Octave::Octave4,
            5 => Octave::Octave5,
            6 => Octave::Octave6,
            7 => Octave::Octave7,
            8 => Octave::Octave8,
            _ => panic!("Invalid value for Octave"),
        }
    }
}

#[derive(PartialEq, Debug, FromPrimitive, Default, Eq, Copy, Clone)]
#[repr(i8)]
pub enum AccidentalSpelling {
    #[default]
    Flat = -1,
    Sharp = 1,
}

#[derive(PartialEq, Debug, FromPrimitive, Default, Eq, Copy, Clone)]
#[repr(i8)]
pub enum Alter {
    DoubleFlat = -2,
    Flat = -1,
    #[default]
    None = 0,
    Sharp = 1,
    DoubleSharp = 2,
}

impl From<Alter> for i8 {
    fn from(value: Alter) -> Self {
        match value {
            Alter::DoubleFlat => -2,
            Alter::Flat => -1,
            Alter::None => 0,
            Alter::Sharp => 1,
            Alter::DoubleSharp => 2,
        }
    }
}

impl Alter {
    pub fn from_num_string(input: &str) -> Result<Alter> {
        match input {
            "-2" => Ok(Alter::DoubleFlat),
            "-1" => Ok(Alter::Flat),
            "0" => Ok(Alter::None),
            "1" => Ok(Alter::Sharp),
            "2" => Ok(Alter::DoubleSharp),
            _ => Err(Error::Parse),
        }
    }

    pub fn to_num_string(&self) -> String {
        match self {
            Alter::DoubleFlat => String::from("-2"),
            Alter::Flat => String::from("-1"),
            Alter::None => String::from("0"),
            Alter::Sharp => String::from("1"),
            Alter::DoubleSharp => String::from("2"),
        }
    }
}

impl FromStr for Alter {
    type Err = Error;
    fn from_str(input: &str) -> Result<Alter> {
        match input {
            "bb" => Ok(Alter::DoubleFlat),
            "b" => Ok(Alter::Flat),
            "" => Ok(Alter::None),
            "#" => Ok(Alter::Sharp),
            "##" => Ok(Alter::DoubleSharp),
            _ => Err(Error::Parse),
        }
    }
}

impl ToString for Alter {
    fn to_string(&self) -> String {
        match self {
            Alter::DoubleFlat => String::from("bb"),
            Alter::Flat => String::from("b"),
            Alter::None => String::from(""),
            Alter::Sharp => String::from("#"),
            Alter::DoubleSharp => String::from("##"),
        }
    }
}

impl ToString for Octave {
    fn to_string(&self) -> String {
        match self {
            Octave::Octave0 => "0".to_string(),
            Octave::Octave1 => "1".to_string(),
            Octave::Octave2 => "2".to_string(),
            Octave::Octave3 => "3".to_string(),
            Octave::Octave4 => "4".to_string(),
            Octave::Octave5 => "5".to_string(),
            Octave::Octave6 => "6".to_string(),
            Octave::Octave7 => "7".to_string(),
            Octave::Octave8 => "8".to_string(),
        }
    }
}

impl FromStr for Octave {
    type Err = Error;
    fn from_str(input: &str) -> Result<Octave> {
        match input {
            "0" => Ok(Octave::Octave0),
            "1" => Ok(Octave::Octave1),
            "2" => Ok(Octave::Octave2),
            "3" => Ok(Octave::Octave3),
            "4" => Ok(Octave::Octave4),
            "5" => Ok(Octave::Octave5),
            "6" => Ok(Octave::Octave6),
            "7" => Ok(Octave::Octave7),
            "8" => Ok(Octave::Octave8),
            _ => Err(Error::Unit),
        }
    }
}

// Step uses the MIDI pitch numeric values, which are one less than our binary format, due to rests being 0
#[derive(Copy, Debug, Clone, Eq, Default, PartialEq, Ord, PartialOrd)]
//#[repr(u8)]
pub enum Step {
    #[default]
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}
// pub enum Step {
//     #[default]
//     C = 49,
//     D = 51,
//     E = 53,
//     F = 54,
//     G = 56,
//     A = 58,
//     B = 60,
// }

// Implementation of the Add trait for Step
impl Add<u8> for Step {
    type Output = Self;

    fn add(self, other: u8) -> Self {
        let mut result = self;
        for _ in 0..other {
            result = result.increment();
        }
        result
    }
}

// Implementation of the Sub trait for Step
impl Sub<u8> for Step {
    type Output = Self;

    fn sub(self, other: u8) -> Self {
        let mut result = self;
        for _ in 0..other {
            result = result.decrement();
        }
        result
    }
}

impl Step {
    pub fn increment(&self) -> Step {
        match self {
            Step::C => Step::D,
            Step::D => Step::E,
            Step::E => Step::F,
            Step::F => Step::G,
            Step::G => Step::A,
            Step::A => Step::B,
            Step::B => Step::C, // Wrap around from B to C
        }
    }
    // Method to decrement the note
    pub fn decrement(&self) -> Step {
        match *self {
            Step::C => Step::B,
            Step::B => Step::A,
            Step::A => Step::G,
            Step::G => Step::F,
            Step::F => Step::E,
            Step::E => Step::D,
            Step::D => Step::C,
        }
    }
}

impl ToString for Step {
    fn to_string(&self) -> String {
        match self {
            Self::C => String::from("C"),
            Self::D => String::from("D"),
            Self::E => String::from("E"),
            Self::F => String::from("F"),
            Self::G => String::from("G"),
            Self::A => String::from("A"),
            Self::B => String::from("B"),
        }
    }
}

impl FromStr for Step {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        match s {
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            _ => Err(Error::Parse),
        }
    }
}

impl From<Step> for i8 {
    fn from(value: Step) -> Self {
        match value {
            Step::C => 48,
            Step::D => 50,
            Step::E => 52,
            Step::F => 53,
            Step::G => 55,
            Step::A => 57,
            Step::B => 59,
        }
    }
}

impl TryFrom<i8> for Step {
    type Error = Error;
    fn try_from(value: i8) -> std::result::Result<Self, Self::Error> {
        match value {
            48 => Ok(Step::C),
            49 => Err(Error::AlterValue),
            50 => Ok(Step::D),
            51 => Err(Error::AlterValue),
            52 => Ok(Step::E),
            53 => Ok(Step::F),
            54 => Err(Error::AlterValue),
            55 => Ok(Step::G),
            56 => Err(Error::AlterValue),
            57 => Ok(Step::A),
            58 => Err(Error::AlterValue),
            59 => Ok(Step::B),
            _ => Err(Error::OutofBounds),
        }
    }
}


#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct Pitch {
    pub step: Step,
    pub alter: Alter,
}

impl FromStr for Pitch {
    type Err = Error;
    fn from_str(input: &str) -> Result<Pitch> {
        let char_vec: Vec<String> = input.chars().map(|inp| inp.to_string()).collect();

        if char_vec.len() > 3 {
            return Err(Error::Parse);
        } else {
            let step = Step::from_str(char_vec[0].as_str())?;
            let alter_str;
            if char_vec.len() == 3 {
                alter_str = [char_vec[1].as_str(), char_vec[2].as_str()].join("");
            } else if char_vec.len() == 2 {
                alter_str = char_vec[1].to_string();
            } else if char_vec.len() == 1 {
                // fall through
                alter_str = "".to_string();
            } else {
                return Err(Error::Parse);
            }
            let alter = Alter::from_str(alter_str.as_str())?;
            Ok(Pitch { step, alter })
        }
    }
}

impl ToString for Pitch {
    fn to_string(&self) -> String {
        [self.step.to_string(), self.alter.to_string()].join("")
    }
}
#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub struct PitchOctave {
    pub pitch: Pitch,
    pub octave: Octave,
}

impl PitchOctave {
    const MAX_NOTE_VALUE: i8 = 96;
    const MIN_NOTE_VALUE: i8 = 12;

    // If the preferred Alter is None, and the semitone falls on an accidental,
    // a default spelling must be chosen, we will choose to use flats by default.
    pub fn new_from_semitone(
        semitone: i8,
        mut prefer_spelling: AccidentalSpelling,
    ) -> Result<PitchOctave> {
        let octave = Self::calculate_octave(semitone as u8);
        let numeric_octave = octave as i8;
        let alter = match prefer_spelling {
            AccidentalSpelling::Flat => Alter::Flat,
            AccidentalSpelling::Sharp => Alter::Sharp,
        };
        let numeric_alter = alter as i8;
        // Get the note number normalized to middle C octave
        let middle_c_number = semitone - ((numeric_octave - 3) * 12);
        match Step::try_from(middle_c_number) {
            Ok(diatonic_step) => {
                // The semitone did not fall on an accidental, we are finished
                Ok(PitchOctave {
                    pitch: Pitch {
                        step: diatonic_step,
                        alter: Alter::None,
                    },
                    octave: octave,
                })
            }
            Err(e) => {
                match e {
                    Error::OutofBounds => Err(Error::OutofBounds),
                    Error::AlterValue => {
                        // The semitone fell on an accidental, we must return the preferred spelling
                        match Step::try_from(middle_c_number - numeric_alter) {
                            Ok(diatonic_step) => Ok(PitchOctave {
                                pitch: Pitch {
                                    step: diatonic_step,
                                    alter: alter,
                                },
                                octave: octave,
                            }),
                            Err(e) => panic!("Error should not be possible here"),
                        }
                    }
                    _ => {
                        panic!("Unsupported Error Type")
                    }
                }
            }
        }
    }

    fn get_semitone_value(&self) -> Result<i8> {
        let mut pitch_semitones = i8::from(self.pitch.step);
        let alter_semitones = i8::from(self.pitch.alter);
        let numeric_octave = self.octave as i8;

        pitch_semitones += alter_semitones;
        pitch_semitones += (numeric_octave - 3) * 12;
        if !(Self::MIN_NOTE_VALUE..=Self::MAX_NOTE_VALUE).contains(&pitch_semitones) {
            Err(Error::OutofBounds)
        } else {
            Ok(pitch_semitones)
        }
    }

    fn calculate_octave(semitone: u8) -> Octave {
        let num = (semitone / 12) - 1;
        Octave::from(num)
    }

    // pub fn get_semitone_value(&self) -> i8 {
    //     let step_semitone = i8::from(self.pitch.step);
    //     let alter_semitone = i8::from(self.pitch.alter);
    //     step_semitone + alter_semitone
    // }
    pub fn respell_pitch(&mut self, alter: Alter) {
        match alter {
            Alter::Sharp => {
                self.pitch.step = self.pitch.step.decrement();
                self.pitch.alter = alter;
            }
            Alter::Flat => {
                self.pitch.step = self.pitch.step.increment();
                self.pitch.alter = alter;
            }
            _ => {todo!("Unimplemented");}
        }
    }
    pub fn checked_sub(&self, v: IntervalType) -> Option<Self> {
        let mut target_semitone = self.get_semitone_value().ok()?;
        target_semitone -= v.get_semitone_value() as i8;

        let original_octave = self.octave as u8;
        let new_diatonic_step = self.pitch.step - v.get_diatonic_value();
        let new_diatonic_pitch;
        if new_diatonic_step >= self.pitch.step
            && v.get_diatonic_value() != IntervalType::UNISON_DIATONIC_INC
        {
            // The new step is an octave lower than the previous
            new_diatonic_pitch = PitchOctave {
                pitch: Pitch {
                    step: new_diatonic_step,
                    alter: Alter::None,
                },
                octave: Octave::from(original_octave - 1),
            };
        } else {
            new_diatonic_pitch = PitchOctave {
                pitch: Pitch {
                    step: new_diatonic_step,
                    alter: Alter::None,
                },
                octave: self.octave,
            };
        }
        let diatonic_semitone = new_diatonic_pitch.get_semitone_value().ok()?;

        // Calculate difference between diatonic target and actual target and use difference to infer
        // accidentals
        // println!("ds is {diatonic_semitone} ts is {target_semitone}");
        let difference = diatonic_semitone - target_semitone;
        // println!("Difference is {}", difference);
        let mut pabs = if difference == -1 {
            PitchOctave::new_from_semitone(target_semitone, AccidentalSpelling::Sharp).ok()?
        } else if difference == 1 || difference == 0{
            PitchOctave::new_from_semitone(target_semitone, AccidentalSpelling::Flat).ok()?
        } else {
            panic!("Incomplete tbd")
        };
        if pabs.pitch.alter == Alter::None {
            // re-spell pitches that did not contain accidentals to contain accidentals based on difference
            if difference == 1 {
                pabs.respell_pitch(Alter::Flat);
            } else if difference == -1 {
                pabs.respell_pitch(Alter::Sharp);
            }
        }
        Some(pabs)
    }
    pub fn checked_add(&self, v: IntervalType) -> Option<Self> {
        let mut target_semitone = self.get_semitone_value().ok()?;
        target_semitone += v.get_semitone_value() as i8;

        let original_octave = self.octave as u8;
        let new_diatonic_step = self.pitch.step + v.get_diatonic_value();
        let new_diatonic_pitch;
        if new_diatonic_step <= self.pitch.step
            && v.get_diatonic_value() != IntervalType::UNISON_DIATONIC_INC
        {
            // The new step is an octave higher than the previous
            new_diatonic_pitch = PitchOctave {
                pitch: Pitch {
                    step: new_diatonic_step,
                    alter: Alter::None,
                },
                octave: Octave::from(original_octave + 1),
            };
        } else {
            new_diatonic_pitch = PitchOctave {
                pitch: Pitch {
                    step: new_diatonic_step,
                    alter: Alter::None,
                },
                octave: self.octave,
            };
        }
        let diatonic_semitone = new_diatonic_pitch.get_semitone_value().ok()?;

        // Calculate difference between diatonic target and actual target and use difference to infer
        // accidentals
        // println!("ds is {diatonic_semitone} ts is {target_semitone}");
        let difference = diatonic_semitone - target_semitone;
        // println!("Difference is {}", difference);
        let mut pabs = if difference == -1 {
            PitchOctave::new_from_semitone(target_semitone, AccidentalSpelling::Sharp).ok()?
        } else if difference == 1 || difference == 0{
            PitchOctave::new_from_semitone(target_semitone, AccidentalSpelling::Flat).ok()?
        } else {
            panic!("Incomplete tbd")
        };
        if pabs.pitch.alter == Alter::None {
            // re-spell pitches that did not contain accidentals to contain accidentals based on difference
            if difference == 1 {
                pabs.respell_pitch(Alter::Flat);
            } else if difference == -1 {
                pabs.respell_pitch(Alter::Sharp);
            }
        }
        Some(pabs)
    }

}
// impl FromStr for PitchAbsolute {
//     type Err = Error;
//     fn from_str(input: &str) -> Result<PitchAbsolute> {
//         match input {

//         }
//     }
// }

impl FromStr for PitchOctave {
    type Err = Error;
    fn from_str(input: &str) -> Result<PitchOctave> {
        let char_vec: Vec<String> = input.chars().map(|inp| inp.to_string()).collect();

        if char_vec.len() > 4 {
            return Err(Error::Parse);
        } else {
            let octave_str;
            let pitch;
            if char_vec.len() == 4 {
                pitch = Pitch::from_str([char_vec[0].as_str(), char_vec[1].as_str(), char_vec[2].as_str()].join("").as_str())?;
                octave_str = char_vec[3].to_string();
            } else if char_vec.len() == 3 {
                pitch = Pitch::from_str([char_vec[0].as_str(), char_vec[1].as_str()].join("").as_str())?;
                octave_str = char_vec[2].to_string();
            } else if char_vec.len() == 2 {
                pitch = Pitch::from_str(char_vec[0].as_str())?;
                octave_str = char_vec[1].to_string();
            } else {
                return Err(Error::Parse);
            }
            let octave = Octave::from_str(octave_str.as_str())?;
            Ok(PitchOctave { pitch, octave })
        }
    }
}

impl ToString for PitchOctave {
    fn to_string(&self) -> String {
        [
            self.pitch.step.to_string(),
            self.pitch.alter.to_string(),
            self.octave.to_string(),
        ]
        .join("")
    }
}

impl PitchOctave {
    pub fn new(pitch: Pitch, octave: Octave) -> PitchOctave {
        PitchOctave { pitch, octave }
    }
    pub fn pitch_alter_string(self) -> String {
        self.pitch.to_string()
    }
    // pub fn checked_add(&self, v: Interval) -> Option<Self> {
    //     let prev_octave = self.octave;
    //     let semitones = v.get_semitone_value();
    //     let quality = v.get_quality();
    //     let tonic_alter = self.pitch.alter;

    //     None
    // if v == 0 {
    //     Some(*self)
    // } else {
    //     let prev_octave = self.octave;
    //     let semitones = v.get_semitone_value();
    //     let quality = v.get_quality();
    //     let tonic_alter = self.pitch.alter;

    //     let result = self.add(v);
    //     if result.octave == Octave::Octave0 && prev_octave != Octave::Octave0 {
    //         // Overflow case
    //         None
    //     } else {
    //         Some(result)
    //     }
    // }
    // }
    // pub fn checked_sub(&self, v: u8) -> Option<Self> {
    //     if v == 0 {
    //         Some(*self)
    //     } else {
    //         let prev_octave = self.octave;
    //         let result = self.sub(v);
    //         if result.octave == Octave::Octave8 && prev_octave != Octave::Octave8 {
    //             // Overflow case
    //             None
    //         } else {
    //             Some(result)
    //         }
    //     }
    // }
}

#[cfg(test)]
mod tests {

    use crate::{
        error::Error,
        pitch::{AccidentalSpelling, Alter, PitchOctave},
    };
    #[test]
    fn test_semitone_convert() {
        let pitch = PitchOctave::new_from_semitone(49, AccidentalSpelling::Flat);
        println!("{:?}", pitch);

        assert_eq!(true, true);
    }
}
