use std::env::Args;
use crate::ParseError::*;

#[derive(Debug)]
enum ParseError {
    TooFewArgs,
    TooManyArgs,
    InvalidInteger(String),
}

#[derive(Debug)]
struct Frame {
    width: u32,
    height: u32,
}

struct ParseArgs(Args);

impl ParseArgs {
    fn new() -> ParseArgs {
        ParseArgs(std::env::args())
    }

    fn require_arg(&mut self) -> Result<String, ParseError> {
        match self.0.next() {
            None => return Err(TooFewArgs),
            Some(v) => Ok(v),
        }
    }

    fn require_no_args(&mut self) -> Result<(), ParseError> {
        match self.0.next() {
            None => Ok(()),
            Some(_) => return Err(TooManyArgs),
        }
    }
}

fn parse_args() -> Result<Frame, ParseError> {

    let mut args = ParseArgs::new();
    args.require_arg()?;

    let width_str = args.require_arg()?;
    let height_str = args.require_arg()?;

    args.require_no_args()?;

    let width = parse_u32(width_str)?;
    let height = parse_u32(height_str)?;

    Ok(
        Frame {
            width,
            height,
        }
    )
}
fn parse_u32(v: String) -> Result<u32, ParseError> {
    match v.parse() {
        Err(_) => return Err(InvalidInteger(v)),
        Ok(v) => Ok(v),
    }
}

fn main() {
    println!("{:?}", parse_args());
}
