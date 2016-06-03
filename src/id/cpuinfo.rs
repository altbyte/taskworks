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
            IResult::Done(a,o) => {
                for o in o {
                    println!("{:?}", o);
                }
            }
            _                  => panic!("Error")
        }
    }

    fn parse_cpuinfo(input:&[u8]) -> IResult<&[u8],Vec<Cpuinfo>> {
        many0!(input,
               chain!(
                   field: map_res!(take_until_either!("\t"), str::from_utf8) ~
                   value: map_res!(take_until_either!("\n"), str::from_utf8) ~ line_ending,
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
