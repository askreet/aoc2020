use std::io::{BufRead};
use lazy_static::lazy_static;
use regex::{Regex};
use enum_from_str::ParseEnumVariantError;
use enum_from_str_derive::FromStr;

#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Passport {
    byr: Option<Year>,
    iyr: Option<Year>,
    eyr: Option<Year>,
    hgt: Height,
    hcl: Option<HairColor>,
    ecl: Option<EyeColor>,
    pid: Option<PassportID>,
    cid: Option<String>,
}

impl Passport {
    pub fn is_somewhat_valid(&self) -> bool {
        self.byr.is_some() &&
            self.iyr.is_some() &&
            self.eyr.is_some() &&
            self.hgt != Height::Invalid &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()
    }

    pub fn is_valid(&self) -> bool {
        self.byr.is_some() && self.byr.unwrap().is_between(1920, 2002) &&
            self.iyr.is_some() && self.iyr.unwrap().is_between(2010, 2020) &&
            self.eyr.is_some() && self.eyr.unwrap().is_between(2020, 2030) &&
            self.hgt.is_valid() &&
            self.hcl.is_some() &&
            self.ecl.is_some() &&
            self.pid.is_some()
    }
}

type Year = u16;

trait YearMethods {
    fn parse(input: &str) -> Option<Year>;
    fn is_between(&self, a: u16, b: u16) -> bool;
}

impl YearMethods for Year {
    fn parse(input: &str) -> Option<Year> {
        match input.parse::<u16>() {
            Ok(v) => Some(v),
            Err(_) => None
        }
    }

    fn is_between(&self, a: u16, b: u16) -> bool {
        (a..=b).contains(&self)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
enum Height {
    Invalid,
    In(u8),
    Cm(u8)
}

impl Default for Height {
    fn default() -> Self {
        Height::Invalid
    }
}

impl Height {
    fn parse(input: &str) -> Height {
        if let Some(captures) = regex_captures!(r"(\d+)(cm|in)", input) {
            match captures[1].parse::<u8>() {
                Ok(v) => match &captures[2] {
                    "cm" => Height::Cm(v),
                    "in" => Height::In(v),
                    &_ => Height::Invalid
                },
                Err(_) => Height::Invalid
            }
        } else {
            Height::Invalid
        }
    }

    fn is_valid(&self) -> bool {
        match self {
            Height::Cm(cm) => (150..=193).contains(cm),
            Height::In(ins) => (59..=76).contains(ins),
            Height::Invalid => false,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct HairColor { inner: String }

impl HairColor {
    fn parse(input: &str) -> Option<HairColor> {
        regex_captures!(r"^#[0-9a-f]{6}$", input)
            .map(|c| HairColor { inner: c[0].to_string() })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct PassportID { inner: String }
impl PassportID {
    fn parse(input: &str) -> Option<PassportID> {
        regex_captures!(r"^[0-9]{9}$", input)
            .map(|c| PassportID { inner: c[0].to_string() })
    }
}


#[derive(Clone, FromStr, PartialEq, Eq, Debug)]
#[allow(non_camel_case_types)]
enum EyeColor { amb, blu, brn, gry, grn, hzl, oth }

struct RecordReader<T: BufRead> {
    reader: T,
}

impl<T: BufRead> RecordReader<T> {
    fn new(reader: T) -> RecordReader<T> {
        RecordReader { reader }
    }
}

impl<T: BufRead> Iterator for RecordReader<T> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buf = String::new();

        loop {
            match self.reader.read_line(&mut buf) {
                Ok(bytes) =>
                    match (bytes, buf.is_empty()) {
                        (0, true) => return None, // EOF, no pending record.
                        (0, false) => return Some(buf), // EOF, pending record.
                        (1, _) => return Some(buf), // Blank line = end of record.
                        _ => {}
                    },
                Err(e) => panic!(e),
            }
        }
    }
}

pub fn parse_passports<T: BufRead>(reader: T) -> Vec<Passport> {
    lazy_static! {
        static ref FIELD_RE: Regex = Regex::new(r"(\w{3}):(\S*)").unwrap();
    }

    let record_reader = RecordReader::new(reader);

    record_reader.map(|record| {
        let mut passport = Passport::default();

        for captures in FIELD_RE.captures_iter(record.as_str()) {
            let key: &str = &captures[1];
            let value: &str = &captures[2];

            match key {
                "byr" => passport.byr = Year::parse(value),
                "iyr" => passport.iyr = Year::parse(value),
                "eyr" => passport.eyr = Year::parse(value),
                "hgt" => passport.hgt = Height::parse(value),
                "hcl" => passport.hcl = HairColor::parse(value),
                "ecl" => passport.ecl = value.parse::<EyeColor>().ok(),
                "pid" => passport.pid = PassportID::parse(value),
                "cid" => passport.cid = Some(value.to_string()),
                &_ => {}
            }
        }

        passport
    }).collect()
}

#[test]
fn test_parse_passports() {
    use std::io::Cursor;
    let reader = Cursor::new("\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in");

    let passports = parse_passports(reader);

    assert_eq!(
        Passport {
            byr: Some(1937),
            iyr: Some(2017),
            eyr: Some(2020),
            hgt: Height::Cm(183),
            hcl: Some(HairColor { inner: "#fffffd".to_string() }),
            ecl: Some(EyeColor::gry),
            pid: Some(PassportID { inner: "860033327".to_string() }), // TODO
            cid: Some("147".to_string()), // TODO
        },
        passports[0]
    );

    assert_eq!(4, passports.len());
}

