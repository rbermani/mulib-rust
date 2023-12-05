
use crate::{error::{Error, Result}, pitch::PitchOctave};

pub type Pitches = Vec<PitchOctave>;

pub trait Notes {
    fn notes(&self) -> Result<Pitches>;
}

