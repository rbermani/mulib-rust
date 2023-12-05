use mulib::chord::ChordQuality;
use mulib::pitch::PitchOctave;
//use crate::error::{Error, Result};
use repl_rs::Convert;
use repl_rs::Value;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;
use crate::error::{Result, Error};
use mulib::error::Error as MuLibErr;

#[derive(Default)]
pub struct Context {
    list: VecDeque<String>,
}

// Append name to list
pub fn append(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>> {
    let name: String = args["name"].convert()?;
    context.list.push_back(name);
    let list: Vec<String> = context.list.clone().into();

    Ok(Some(list.join(", ")))
}

// Prepend name to list
pub fn prepend(args: HashMap<String, Value>, context: &mut Context) -> Result<Option<String>> {
    let name: String = args["name"].convert()?;
    context.list.push_front(name);
    let list: Vec<String> = context.list.clone().into();

    Ok(Some(list.join(", ")))
}

// Add two numbers.
pub fn add<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    let first: i32 = args["first"].convert()?;
    let second: i32 = args["second"].convert()?;

    Ok(Some((first + second).to_string()))
}

// Generate chord
pub fn chord_gen<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    let root_str: String = args["root"].convert()?;
    let chord_str: String = args["chord"].convert()?;
    let root = PitchOctave::from_str(root_str.as_str())?;
    let chordtype = ChordQuality::from_str(chord_str.as_str())?;
    Err(Error::MuLib(MuLibErr::AlterValue))
    //println!("{:?}",root);
    // let chord = ChordBuilder::build(root, chordtype);

    // Ok(Some(match chord.get_instance(Inversion::Root) {
    //     Err(e) => {
    //         match e {
    //             Error::InvalidInversion => e.to_string(),
    //             Error::OutofBounds => e.to_string(),
    //             _ => "Unknown".to_string(),
    //         }
    //     }
    //     Ok(chord) => {
    //         chord.iter().map(|noteabs| noteabs.to_string()).collect::<Vec<String>>().join(" ")
    //     },
    // }))
}
