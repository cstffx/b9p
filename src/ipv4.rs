use std::net::Ipv4Addr;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::error::{Error, ErrorKind};
use nom::IResult;
use nom::multi::separated_list0;

pub fn dot_and_octet(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag(".")(input)?;
    let (input, num) = complete::u8(input)?;

    Ok((input, num))
}

pub fn ipv4(input: &str) -> IResult<&str, Ipv4Addr> {
    let (input, octets) = separated_list0(
        tag("."),
        complete::u8
    )(input)?;

    if octets.len() == 4 {
        Ok((input, Ipv4Addr::from([octets[0], octets[1], octets[2], octets[3]])))
    } else {
        Err(nom::Err::Error(Error {
            input,
            code: ErrorKind::Fail,
        }))
    }
}

#[cfg(test)]
mod test {
    use crate::ipv4::*;
    use std::net::Ipv4Addr;

    macro_rules! build_ipv4_addr {
        ($ip_address:expr) => {
            $ip_address.parse::<Ipv4Addr>().unwrap()
        };
    }

    macro_rules! assert_ipv4_address {
        ($ip_address:literal) => {{
            let ip = build_ipv4_addr!($ip_address);
            assert_eq!(Ok( ("", ip)), ipv4($ip_address))
        }};
    }

    #[test]
    fn test_ipv4() {
        assert_ipv4_address!("0.0.0.0");
        assert_ipv4_address!("172.16.64.12");
        assert_ipv4_address!("255.255.255.255");

        assert!(ipv4("192.158.52,1").is_err());
        assert!(ipv4("256.0.0.1").is_err());
        assert!(ipv4("a.0.0.1").is_err());
    }

    #[test]
    fn test_dot_and_octet() {
        assert_eq!(Ok(("", 0)), dot_and_octet(".0"));
        assert_eq!(Ok(("", 255)), dot_and_octet(".255"));

        assert!(dot_and_octet(".256").is_err());
    }
}