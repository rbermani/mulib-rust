use mulib::chord::Chord;
use mulib::chord::ChordQuality;
use mulib::chord::Inversion;
use mulib::interval::Interval;
use mulib::interval::IntervalType;
use mulib::pitch::PitchOctave;
use mulib::notes::Notes;

use mulib::pitch::PitchOrder;
use mulib::scale::Scale;
use mulib::scale::ScaleType;
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

    //println!("{:?}",root);
    let chord = Chord::new_as_inversion(root, chordtype, Inversion::Root);

    Ok(Some(match chord.notes() {
        Err(e) => {
            match e {
                MuLibErr::InvalidInversion => e.to_string(),
                MuLibErr::OutofBounds => e.to_string(),
                _ => "Unknown".to_string(),
            }
        }
        Ok(pitches) => {
            pitches.0.iter().map(|noteabs| noteabs.to_string()).collect::<Vec<String>>().join(" ")
        },
    }))
}

// Generate scale
pub fn scale_gen<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    let tonic_str: String = args["tonic"].convert()?;
    let scale_str: String = args["scale"].convert()?;
    let tonic = PitchOctave::from_str(tonic_str.as_str())?;
    let scale_type = ScaleType::from_str(scale_str.as_str())?;

    //println!("{:?}",root);
    let scale = Scale::new(tonic, scale_type);

    Ok(Some(match scale.notes() {
        Err(e) => {
            match e {
                MuLibErr::OutofBounds => e.to_string(),
                _ => "Unknown".to_string(),
            }
        }
        Ok(pitches) => {
            pitches.0.iter().map(|noteabs| noteabs.to_string()).collect::<Vec<String>>().join(" ")
        },
    }))
}

// Generate descending scale
pub fn scale_gen_desc<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    let tonic_str: String = args["tonic"].convert()?;
    let scale_str: String = args["scale"].convert()?;
    let tonic = PitchOctave::from_str(tonic_str.as_str())?;
    let scale_type = ScaleType::from_str(scale_str.as_str())?;

    //println!("{:?}",root);
    let scale = Scale::new_with_order(tonic, scale_type, PitchOrder::Descending);

    Ok(Some(match scale.notes() {
        Err(e) => {
            match e {
                MuLibErr::OutofBounds => e.to_string(),
                _ => "Unknown".to_string(),
            }
        }
        Ok(pitches) => {
            pitches.0.iter().map(|noteabs| noteabs.to_string()).collect::<Vec<String>>().join(" ")
        },
    }))
}

// Generate interval
pub fn interval_gen<T>(args: HashMap<String, Value>, _context: &mut T) -> Result<Option<String>> {
    let root_str: String = args["root"].convert()?;
    let interval_str: String = args["interval"].convert()?;
    let order: String = args["order"].convert()?;
    let root = PitchOctave::from_str(root_str.as_str())?;
    let interval_type = IntervalType::from_str(interval_str.as_str())?;
    let pitch_order = PitchOrder::from_str(order.as_str())?;

    let interval = Interval::new_with_order(root, interval_type, pitch_order);

    Ok(Some(match interval.notes() {
        Err(e) => {
            match e {
                MuLibErr::OutofBounds => e.to_string(),
                _ => "Unknown".to_string(),
            }
        }
        Ok(pitches) => {
            pitches.0.iter().map(|noteabs| noteabs.to_string()).collect::<Vec<String>>().join(" ")
        },
    }))
}