
use crate::{error::{Error, Result}, pitch::PitchOctave};
use crate::pitch::Pitches;


pub trait Notes {
    fn notes(&self) -> Result<Pitches>;
}

