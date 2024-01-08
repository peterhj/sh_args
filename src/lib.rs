#[cfg(feature = "smol_str")]
extern crate smol_str;

#[cfg(feature = "smol_str")]
use smol_str::{SmolStr};

use std::iter::{Iterator, Peekable};

#[cfg(not(feature = "smol_str"))]
pub type ArgString = std::string::String;
#[cfg(feature = "smol_str")]
pub type ArgString = SmolStr;

#[derive(Clone, Debug)]
pub enum Arg {
  Command{cmd: ArgString},
  Option{dashes: u8, key: String, val: Option<ArgString>},
  EndOptions,
  Param{idx: usize, val: ArgString},
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
              let arg_s = arg_s.get(2 .. ).unwrap();
              let (key, val) = if let Some((key, val)) = arg_s.split_once("=") {
                (key.into(), Some(val.into()))
              } else {
                // FIXME: peek next argument.
                (arg_s.into(), None)
              };
              Some(Arg::Option{dashes: 2, key, val: None})
            }
          }
          // FIXME: empty single-dash option.
          _ => {
            let arg_s = arg_s.get(1 .. ).unwrap();
            let (key, val) = if let Some((key, val)) = arg_s.split_once("=") {
              (key.into(), Some(val.into()))
            } else {
              // FIXME: peek next argument.
              (arg_s.into(), None)
            };
            Some(Arg::Option{dashes: 1, key, val: None})
          }
        }
      }
      Some(_) => {
        let idx = self.idx;
        self.idx += 1;
        let val = arg_s.into();
        if idx == 0 {
          Some(Arg::Command{cmd: val})
        } else {
          Some(Arg::Param{idx, val})
        }
      }
      None => panic!("bug")
    }
  }
}
