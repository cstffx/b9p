use nom::combinator::opt;
use nom::error::{Error, ErrorKind};
use nom::IResult;
use nom::sequence::tuple;

use crate::duration::common::number_and_char_no_case;

const SECONDS_IN_A_WEEK: u32 = 604_800;
const SECONDS_IN_A_DAY: u32 = 86_400;
const SECONDS_IN_AN_HOUR: u32 = 3_600;
const SECONDS_IN_A_MINUTE: u32 = 60;
const SECONDS_IN_A_SECOND: u32 = 1;

#[derive(Debug)]
pub struct TtlTimeUnix {
    weeks: u32,
    days: u32,
    hours: u32,
    minutes: u32,
    seconds: u32,
}

impl TtlTimeUnix {
    fn duration(&self) -> u32 {
        (self.weeks * SECONDS_IN_A_WEEK) +
            (self.days * SECONDS_IN_A_DAY) +
            (self.hours * SECONDS_IN_AN_HOUR) +
            (self.minutes * SECONDS_IN_A_MINUTE) +
            (self.seconds * SECONDS_IN_A_SECOND)
    }

    fn has_time(&self) -> bool {
        self.duration() > 0
    }

    fn from_option_tuple(tup: (Option<u32>, Option<u32>, Option<u32>, Option<u32>, Option<u32>)) -> TtlTimeUnix {
        TtlTimeUnix {
            weeks: tup.0.unwrap_or(0),
            days: tup.1.unwrap_or(0),
            hours: tup.2.unwrap_or(0),
            minutes: tup.3.unwrap_or(0),
            seconds: tup.4.unwrap_or(0),
        }
    }
}

impl PartialEq for TtlTimeUnix {
    fn eq(&self, other: &Self) -> bool {
        self.duration() == other.duration()
    }
}

impl PartialEq<u32> for TtlTimeUnix {
    fn eq(&self, other: &u32) -> bool {
        self.duration() == *other
    }
}

pub fn ttl_unit(input: &str) -> IResult<&str, TtlTimeUnix> {
    let (input, options) = tuple((
        opt(number_and_char_no_case("w")),
        opt(number_and_char_no_case("d")),
        opt(number_and_char_no_case("h")),
        opt(number_and_char_no_case("m")),
        opt(number_and_char_no_case("s")),
    ))(input)?;

    let ttl = TtlTimeUnix::from_option_tuple(options);
    if ttl.has_time() {
        return Ok((input, ttl));
    }

    Err(nom::Err::Error(Error {
        input,
        code: ErrorKind::Fail,
    }))
}

#[cfg(test)]
mod test {
    use crate::duration::ttl_time_unix::*;

    #[test]
    fn test_ttl_unit() {
        let input = "1w2d3h4m5s";
        let expected = TtlTimeUnix {
            weeks: 1,
            days: 2,
            hours: 3,
            minutes: 4,
            seconds: 5,
        };

        let result = ttl_unit(input);
        match result {
            Ok((_, ttl)) => assert_eq!(ttl, expected),
            Err(_) => panic!("Error parsing input"),
        }
    }
}
