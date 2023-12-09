use crate::interval::{IntervalType, NumericIntervalsSlice};
use crate::pitch::PitchOrder;
use super::pitch::{PitchOctave, Pitches};
use crate::notes::Notes;
use crate::error::{Error, Result};
use strum::EnumString;


pub struct Scale {
    pub tonic: PitchOctave,
    pub kind: ScaleType,
    pub pitch_order: PitchOrder,
}

impl Scale {
    pub fn new(tonic: PitchOctave, kind: ScaleType) -> Self {
        Self {
            tonic,
            kind,
            pitch_order: PitchOrder::Ascending,
        }
    }
    pub fn new_with_order(tonic: PitchOctave, kind: ScaleType, pitch_order: PitchOrder) -> Self {
        Self {
            tonic,
            kind,
            pitch_order,
        }
    }
    fn gen_notes(&self) -> Result<Pitches> {
        let intervals = self.kind.scale_interval();
        let mut scale = Pitches(vec![self.tonic]);
        match self.pitch_order {
            PitchOrder::Ascending => {
                for pitch in intervals {
                    if let Some(scale_tone) = self.tonic.checked_add(IntervalType::from(*pitch)) {
                        scale.0.push(scale_tone);
                    } else {
                        return Err(Error::OutofBounds);
                    }
                }
                // Add the octave of the tonic
                if let Some(scale_tone) = self.tonic.checked_add(IntervalType::Octave) {
                    scale.0.push(scale_tone);
                } else {
                    return Err(Error::OutofBounds);
                }
            }
            PitchOrder::Descending => {
                for pitch in intervals.iter().rev() {
                    if let Some(scale_tone) = self.tonic.checked_sub(IntervalType::from(12 - *pitch)) {
                        scale.0.push(scale_tone);
                    } else {
                        return Err(Error::OutofBounds);
                    }
                }
                // Add the octave of the tonic
                if let Some(scale_tone) = self.tonic.checked_sub(IntervalType::Octave) {
                    scale.0.push(scale_tone);
                } else {
                    return Err(Error::OutofBounds);
                }
            }
        }


        Ok(scale)


    }
}

impl Notes for Scale {
    fn notes(&self) -> Result<Pitches> {
        self.gen_notes()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, EnumString)]
pub enum ScaleType {
    Chromatic,
    Blues,
    Augmented,
    Altered,
    AlteredFlat7,
    Ionian,
    IonianAug,
    Dorian,
    DorianFlat2,
    DorianSharp4,
    Phrygian,
    Lydian,
    LydianMinor,
    Mixolydian,
    Aeolian,
    AeolianSharp7,
    MelodicMinor,
    Locrian,
    LocrianTwo,
    LocrianSix,
    LeadingWholeTone,
    WholeTone,
    PentatonicMinor,
    PentatonicMajor,
}

//type IntervalsSlice = [Interval];

impl ScaleType {
    const IONIAN_SCALE: [u8; 6] = [2, 4, 5, 7, 9, 11];
    const CHROMATIC_SCALE: [u8; 10] = [2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    const AEOLIAN_SCALE: [u8; 6] = [2, 3, 5, 7, 8, 10];


    fn scale_interval(&self) -> &NumericIntervalsSlice {
        match self {
            ScaleType::Ionian => &Self::IONIAN_SCALE,
            ScaleType::Chromatic => &Self::CHROMATIC_SCALE,
            ScaleType::Aeolian => &Self::AEOLIAN_SCALE,
            _ => &Self::IONIAN_SCALE,
        }
    }
    // pub fn get_intervals(scale_type: ScaleType) -> &'static IntervalsSlice {
    //     return &Self::IONIAN_SCALE;
    // }
}
