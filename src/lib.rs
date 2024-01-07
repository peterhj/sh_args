#[cfg(feature = "smol_str")]
extern crate smol_str;

#[cfg(feature = "smol_str")]
use smol_str::{SmolStr};

use std::iter::{Iterator, Peekable};

#[cfg(not(feature = "smol_str"))]
pub type ArgString = std::string::String;
#[cfg(feature = "smol_str")]
pub type ArgString = SmolStr;

pub enum Arg {
  Param{idx: usize, val: ArgString},
  Option{dash2: bool, key: String, val: Option<ArgString>},
  EndOptions,
}

pub struct Args<I> where I: Iterator {
  args: Peekable<I>,
  idx:  usize,
}

impl<I: Iterator> From<I> for Args<I> {
  fn from(args: I) -> Args<I> {
    let args = args.peekable();
    let idx = 0;
    Args{args, idx}
  }
}

impl<I: Iterator<Item=String>> Iterator for Args<I> {
  type Item = Arg;

  fn next(&mut self) -> Option<Arg> {
    let arg_s = self.args.next()?;
    let mut arg_chs = arg_s.chars().peekable();
    match arg_chs.peek() {
      Some('-') => {
        let mut arg_chs2 = arg_chs.peekable();
        arg_chs2.next();
        match arg_chs2.peek() {
          Some('-') => {
            if arg_s.len() <= 2 {
              Some(Arg::EndOptions)
            } else {
              // TODO: option val argument.
              let key = arg_s.get(2 .. ).unwrap().into();
              Some(Arg::Option{dash2: true, key, val: None})
            }
          }
          None => {
            // TODO: option val argument.
            let key = arg_s.get(1 .. ).unwrap().into();
            Some(Arg::Option{dash2: false, key, val: None})
          }
        }
      }
      Some(_) => {
        let idx = self.idx;
        self.idx += 1;
        let val = arg_s.into();
        Some(Arg::Param{idx, val})
      }
      None => panic!("bug")
    }
  }
}
