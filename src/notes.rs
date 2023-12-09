
use crate::error::Result;
use crate::pitch::Pitches;


pub trait Notes {
    fn notes(&self) -> Result<Pitches>;
}

