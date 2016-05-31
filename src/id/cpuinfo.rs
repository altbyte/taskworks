use std::io;
use std::io::Read;
use std::io::prelude;
use std::str;
use nom::{not_line_ending,line_ending};
use nom::IResult;
use std::fs::{OpenOptions, File};

#[derive(Debug)]
pub struct Cpuinfo<'a> {
    pub field: Option<&'a str>,
    pub value: Option<Vec<&'a str>>,
}

impl<'a> Cpuinfo<'a> {
    fn new() -> Cpuinfo<'a> {
        Cpuinfo {
            field: None,
            value: None,
        }
    }

    pub fn read_cpuinfo() {
        let mut input = OpenOptions::new().read(true).open("/proc/cpuinfo").unwrap();
        let mut rtv = Vec::new();
        input.read_to_end(&mut rtv);
        match Cpuinfo::parse_cpuinfo(rtv.as_slice()) {
            IResult::Done(_,o) => {
                print!("{:?}", o);
                for o in o {
                    println!("{:?}", o.field);
                    println!("{:?}", o.value);
                }
            }
            _                  => panic!("Error")
        }
    }

    fn parse_cpuinfo(input:&[u8]) -> IResult<&[u8],Vec<Cpuinfo>> {
        many0!(input,
               chain!(
                   field: map_res!( is_not!("\t"), str::from_utf8) ~
                   value: many0!(terminated!(map_res!( tag!(": "), str::from_utf8), tag!("\n"))),
                   ||{
                       Cpuinfo {
                           field: Some(field),
                           value: Some(value)
                       }
                   }
                )
            )
    }

}
