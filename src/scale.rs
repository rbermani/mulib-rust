use super::pitch::{Pitch, PitchOctave, Alter};
use crate::{error::{Error, Result}, chord::{Inversion, Voicing}, interval::Interval};
use strum::EnumString;
type ScaleInstance = Vec<PitchOctave>;

pub struct Scale {
    kind: ScaleType,
    notes: Vec<Pitch>,
}

impl Scale {
    fn gen_scale_notes(tonic: Pitch) -> Vec<Pitch> {

        vec![]
    }

    pub fn new(tonic: Pitch, kind: ScaleType) -> Scale {
        Scale {
            kind,
            notes: Self::gen_scale_notes(tonic),
        }
    }
}

impl Scale {
    pub fn get_tonic(&self) -> Pitch {
        self.notes.get(0).unwrap().clone()
    }
    pub fn get_type(&self) -> ScaleType {
        self.kind
    }
    pub fn get_instance(&self) -> Result<ScaleInstance> {
        // let root_form = self.kind.root_chord_interval();
        // let mut chord: ChordInstance = vec![self.root];
        // for note in root_form {
        //     if let Some(chord_tone) = self.root.checked_add(*note) {
        //         chord.push(chord_tone);
        //     } else {
        //         return Err(Error::OutofBounds);
        //     }
        // }
        Err(Error::OutofBounds)
        //Ok(ScaleInstance)
    }
    pub fn get_scale_tones(
        &self,
        _inversion: Inversion,
        _voicing: Voicing,
    ) -> Result<ScaleInstance> {
        //let chordvec = vec![self.root];

        // match voicing {
        //     Voicing::Close => {
        //         if let Some(intervals) = self.get_instance(Inversion::Root) {
        //             for note in intervals {
        //                 if let Some(chord_tone) = self.root.checked_add(*note) {
        //                     chordvec.push(chord_tone);
        //                 } else {
        //                     return Err(Error::OutofBounds);
        //                 }
        //             }
        //         } else {
        //             return Err(Error::InvalidInversion);
        //         }

        //     }
        //     _ => {

        //     }
        // }
        //Ok(chordvec)
        Err(Error::OutofBounds)
    }
}

pub struct ScaleBuilder {
    root: PitchOctave,
    kind: ScaleType,
}

impl ScaleBuilder {
    pub fn build(root: PitchOctave, kind: ScaleType) -> Scale {
        Scale { kind, notes: vec![] }
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

type IntervalsSlice = [Interval];

impl ScaleType {
    const IONIAN_SCALE: [Interval; 7] = [
        Interval::MajorSecond,
        Interval::MajorSecond,
        Interval::MinorSecond,
        Interval::MajorSecond,
        Interval::MajorSecond,
        Interval::MajorSecond,
        Interval::MinorSecond,
    ];
    const AEOLIAN_SCALE: [Alter; 7] = [
        Alter::None,
        Alter::None,
        Alter::None,
        Alter::None,
        Alter::None,
        Alter::None,
        Alter::None,
    ];

    pub fn get_intervals(scale_type: ScaleType) -> &'static IntervalsSlice {
        return &Self::IONIAN_SCALE;
    }
}
