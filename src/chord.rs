use crate::interval::{IntervalType, NumericIntervalsSlice};
use crate::notes::Notes;
use crate::pitch::{PitchOctave, Pitches};
use crate::error::{Error, Result};
use strum::EnumString;


#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Chord {
    pub root: PitchOctave,
    pub kind: ChordQuality,
    pub inversion: Inversion,
}

impl Chord {
    pub fn new(root: PitchOctave, kind: ChordQuality) -> Self {
        Self {root, kind, inversion: Inversion::default()}
    }
    pub fn new_as_inversion(root: PitchOctave, kind: ChordQuality, inversion: Inversion) -> Self {
        Self {root, kind, inversion}
    }
    fn gen_inversion(&self, inversion: Inversion) -> Result<Pitches> {
        let root_form = self.kind.root_chord_interval();
        match self.inversion {
            Inversion::Root => {
                let mut chord  = Pitches(vec![self.root]);
                for note in root_form {
                    if let Some(chord_tone) = self.root.checked_add(IntervalType::from(*note)) {
                        chord.0.push(chord_tone);
                    } else {
                        return Err(Error::OutofBounds);
                    }
                }
                Ok(chord)
            }
            Inversion::First => {
                let mut chord  = Pitches(vec![]);
                for note in root_form {
                    // The third is the bass note
                    if let Some(chord_tone) = self.root.checked_add(IntervalType::from(*note)) {
                        chord.0.push(chord_tone);
                    } else {
                        return Err(Error::OutofBounds);
                    }
                }
                if let Some(chord_tone) = self.root.checked_add(IntervalType::Octave) {
                    chord.0.push(chord_tone);
                } else {
                    return Err(Error::OutofBounds);
                }
                Ok(chord)
            }
            Inversion::Second => {
                // Get first inversion, and move first note up an octave
                let mut chord = self.gen_inversion(Inversion::First)?;
                let mut first_note = PitchOctave::default();
                if !chord.0.is_empty() {
                    first_note = chord.0.remove(0);
                }
                if let Some(chord_tone) = first_note.checked_add(IntervalType::Octave) {
                    chord.0.push(chord_tone);
                } else {
                    return Err(Error::OutofBounds);
                }
                Ok(chord)
            }
            Inversion::Third => {
                if self.kind.root_chord_interval().len() >= Inversion::MINIMUM_LEN_THIRD_INVERSION {
                    // Get second inversion, and move first note up an octave
                    let mut chord = self.gen_inversion(Inversion::Second)?;
                    let mut first_note = PitchOctave::default();
                    if !chord.0.is_empty() {
                        first_note = chord.0.remove(0);
                    }
                    if let Some(chord_tone) = first_note.checked_add(IntervalType::Octave) {
                        chord.0.push(chord_tone);
                    } else {
                        return Err(Error::OutofBounds);
                    }
                    Ok(chord)
                } else {
                    // This chord type does not support this rank of inversion
                    Err(Error::InvalidInversion)
                }
            }
            Inversion::Fourth => {
                if self.kind.root_chord_interval().len() >= Inversion::MINIMUM_LEN_FOURTH_INVERSION {
                    // Get second inversion, and move first note up an octave
                    let mut chord = self.gen_inversion(Inversion::Second)?;
                    let mut first_note = PitchOctave::default();
                    if !chord.0.is_empty() {
                        first_note = chord.0.remove(0);
                    }
                    if let Some(chord_tone) = first_note.checked_add(IntervalType::Octave) {
                        chord.0.push(chord_tone);
                    } else {
                        return Err(Error::OutofBounds);
                    }
                    Ok(chord)
                } else {
                    // This chord type does not support this rank of inversion
                    Err(Error::InvalidInversion)
                }
            }
        }
    }
}

impl Notes for Chord {
    fn notes(&self) -> Result<Pitches> {
        self.gen_inversion(self.inversion)
    }
}

#[derive(Clone, Copy, Default, Debug, Eq, PartialEq)]
pub enum Inversion {
    #[default]
    Root,
    First,
    Second,
    Third,
    Fourth,
}

impl Inversion {
    const MINIMUM_LEN_THIRD_INVERSION: usize = 3;
    const MINIMUM_LEN_FOURTH_INVERSION: usize = 4;
}

pub enum Voicing {
    Close,
    Open,
    DropTwo,
    DropThree,
    TwoNote,
    ThreeNote,
    ShellThird,
    ShellSeventh,
    Quartal,
    Quintal,
}

pub enum TemporalStyle {
    Block,
    Broken,
    Arpeggio,
    Stride,
    BrokenTenth,
    Montuno,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumString)]
pub enum ChordQuality {
    Major,
    Minor,
    Diminished,
    Sus4,
    Dom7Sus4,
    Major7,
    MinorMajor7,
    Minor7,
    Dom7,
    Diminished7,
    Major7Sharp5,
    Dom7Sharp5,
    Augmented,
    Minor7Flat5,
    Major7Flat5,
    Add9,
    Major9,
    Dom9,
    MinorAdd9,
    MinorMajor7Add9,
    Minor9,
    Major7Sharp11,
    Major9Sharp11,
    Dom7Sharp11,
    Dom9Sharp11,
    Dom13,
    Dom9Thirteenth,
    Dom7Flat9,
    Dom7Flat13,
    Dom7Flat13Flat9,
    Dom11Flat13Flat9,
    Dom7Sharp9,
    Minor7Add11,
    Minor11,
}

impl ChordQuality {
    const MAJOR_TRIAD: [u8; 2] = [4, 7];
    const MINOR_TRIAD: [u8; 2] = [3, 7];
    const DIMINISHED: [u8; 2] = [3, 6];
    const SUS4: [u8; 2] = [5, 7];
    const MAJOR_7: [u8; 3] = [4, 7, 11];
    const MINOR_MAJOR_7: [u8; 3] = [3, 7, 11];
    const MINOR_7: [u8; 3] = [3, 7, 10];
    const DOM_7: [u8; 3] = [4, 7, 10];
    const DOM7_SUS4: [u8; 3] = [5, 7, 10];
    const DIMINISHED_7: [u8; 3] = [3, 6, 9];
    const MAJOR_7_SHARP_5: [u8; 3] = [4, 8, 11];
    const DOM_7_SHARP_5: [u8; 3] = [4, 8, 10];
    const AUGMENTED: [u8; 2] = [4, 8];
    const MINOR_7_FLAT_5: [u8; 3] = [3, 6, 10];
    const MAJOR_7_FLAT_5: [u8; 3] = [4, 6, 10];
    const ADD_9: [u8; 3] = [4, 7, 2];
    const MAJOR_9: [u8; 4] = [4, 7, 11, 2];
    const DOM_9: [u8; 4] = [4, 7, 10, 2];
    const MINOR_ADD_9: [u8; 3] = [3, 7, 2];
    const MINOR_MAJOR_7_ADD_9: [u8; 4] = [3, 7, 11, 2];
    const MINOR_9: [u8; 4] = [3, 7, 10, 2];
    const MAJOR_7_SHARP_11: [u8; 4] = [4, 7, 11, 6];
    const MAJOR_9_SHARP_11: [u8; 5] = [4, 7, 11, 2, 6];
    const DOM_7_SHARP_11: [u8; 4] = [4, 7, 10, 6];
    const DOM_9_SHARP_11: [u8; 5] = [4, 7, 10, 2, 6];
    const DOM_13: [u8; 4] = [4, 7, 10, 9];
    const DOM_9_THIRTEENTH: [u8; 5] = [4, 7, 10, 2, 9];
    const DOM_7_FLAT_9: [u8; 4] = [4, 7, 10, 1];
    const DOM_7_FLAT_13: [u8; 4] = [4, 7, 10, 8];
    const DOM_7_FLAT_13_FLAT_9: [u8; 5] = [4, 7, 10, 1, 8];
    const DOM_11_FLAT_13_FLAT_9: [u8; 6] = [4, 7, 10, 1, 5, 8];
    const DOM_7_SHARP_9: [u8; 4] = [4, 7, 10, 3];
    const MINOR_7_ADD_11: [u8; 4] = [3, 7, 10, 5];
    const MINOR_11: [u8; 5] = [3, 7, 10, 2, 5];

    // pub fn get_inversion(&self, inversion: Inversion) -> Option<&[u8]> {
    //     match inversion {
    //         Inversion::Root => Some(self.intervals()),
    //         Inversion::First => todo!(),
    //         Inversion::Second => todo!(),
    //         Inversion::Third => todo!(),
    //     }
    // }

    /// Returns the normalized numeric intervals of the chord in close voicing root inversion as a slice
    fn root_chord_interval(&self) -> &NumericIntervalsSlice {
        match self {
            ChordQuality::Major => &Self::MAJOR_TRIAD,
            ChordQuality::Minor => &Self::MINOR_TRIAD,
            ChordQuality::Diminished => &Self::DIMINISHED,
            ChordQuality::Sus4 => &Self::SUS4,
            ChordQuality::Dom7Sus4 => &Self::DOM7_SUS4,
            ChordQuality::Major7 => &Self::MAJOR_7,
            ChordQuality::MinorMajor7 => &Self::MINOR_MAJOR_7,
            ChordQuality::Minor7 => &Self::MINOR_7,
            ChordQuality::Dom7 => &Self::DOM_7,
            ChordQuality::Diminished7 => &Self::DIMINISHED_7,
            ChordQuality::Major7Sharp5 => &Self::MAJOR_7_SHARP_5,
            ChordQuality::Dom7Sharp5 => &Self::DOM_7_SHARP_5,
            ChordQuality::Augmented => &Self::AUGMENTED,
            ChordQuality::Minor7Flat5 => &Self::MINOR_7_FLAT_5,
            ChordQuality::Major7Flat5 => &Self::MAJOR_7_FLAT_5,
            ChordQuality::Add9 => &Self::ADD_9,
            ChordQuality::Major9 => &Self::MAJOR_9,
            ChordQuality::Dom9 => &Self::DOM_9,
            ChordQuality::MinorAdd9 => &Self::MINOR_ADD_9,
            ChordQuality::MinorMajor7Add9 => &Self::MINOR_MAJOR_7_ADD_9,
            ChordQuality::Minor9 => &Self::MINOR_9,
            ChordQuality::Major7Sharp11 => &Self::MAJOR_7_SHARP_11,
            ChordQuality::Major9Sharp11 => &Self::MAJOR_9_SHARP_11,
            ChordQuality::Dom7Sharp11 => &Self::DOM_7_SHARP_11,
            ChordQuality::Dom9Sharp11 => &Self::DOM_9_SHARP_11,
            ChordQuality::Dom13 => &Self::DOM_13,
            ChordQuality::Dom9Thirteenth => &Self::DOM_9_THIRTEENTH,
            ChordQuality::Dom7Flat9 => &Self::DOM_7_FLAT_9,
            ChordQuality::Dom7Flat13 => &Self::DOM_7_FLAT_13,
            ChordQuality::Dom7Flat13Flat9 => &Self::DOM_7_FLAT_13_FLAT_9,
            ChordQuality::Dom11Flat13Flat9 => &Self::DOM_11_FLAT_13_FLAT_9,
            ChordQuality::Dom7Sharp9 => &Self::DOM_7_SHARP_9,
            ChordQuality::Minor7Add11 => &Self::MINOR_7_ADD_11,
            ChordQuality::Minor11 => &Self::MINOR_11,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_chord_gen() {
    //     let chord = ChordBuilder::build(
    //         PitchAbsolute {
    //             step: Step::C,
    //             alter: Alter::None,
    //             octave: Octave::Octave3,
    //         },
    //         ChordType::Major7,
    //     );

    //     let root_inversion = chord
    //         .get_instance(Inversion::Root)
    //         .expect("Failed to generate chord tones");
    //     let first_inversion = chord
    //         .get_instance(Inversion::First)
    //         .expect("Failed to generate chord tones");
    //     let second_inversion = chord
    //         .get_instance(Inversion::Second)
    //         .expect("Failed to generate chord tones");
    //     let third_inversion = chord
    //         .get_instance(Inversion::Third)
    //         .expect("Failed to generate chord tones");

    //     assert_eq!(
    //         root_inversion,
    //         vec![
    //             PitchAbsolute {
    //                 step: Step::C,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave3
    //             },
    //             PitchAbsolute {
    //                 step: Step::E,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave3
    //             },
    //             PitchAbsolute {
    //                 step: Step::G,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave3
    //             },
    //             PitchAbsolute {
    //                 step: Step::B,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave3
    //             }
    //         ]
    //     );

    //     assert_eq!(
    //         first_inversion,
    //         vec![
    //             PitchAbsolute {
    //                 step: Step::E,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave3
    //             },
    //             PitchAbsolute {
    //                 step: Step::G,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave3
    //             },
    //             PitchAbsolute {
    //                 step: Step::B,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave3
    //             },
    //             PitchAbsolute {
    //                 step: Step::C,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave4
    //             }
    //         ]
    //     );
    //     assert_eq!(
    //         second_inversion,
    //         vec![
    //             PitchAbsolute {
    //                 step: Step::G,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave3
    //             },
    //             PitchAbsolute {
    //                 step: Step::B,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave3
    //             },
    //             PitchAbsolute {
    //                 step: Step::C,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave4
    //             },
    //             PitchAbsolute {
    //                 step: Step::E,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave4
    //             },
    //         ]
    //     );
    //     assert_eq!(
    //         third_inversion,
    //         vec![
    //             PitchAbsolute {
    //                 step: Step::B,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave3
    //             },
    //             PitchAbsolute {
    //                 step: Step::C,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave4
    //             },
    //             PitchAbsolute {
    //                 step: Step::E,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave4
    //             },
    //             PitchAbsolute {
    //                 step: Step::G,
    //                 alter: Alter::None,
    //                 octave: Octave::Octave4
    //             },
    //         ]
    //     );
    // }
}
