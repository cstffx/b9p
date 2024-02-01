use std::net::Ipv4Addr;

use nom::{error::ErrorKind, IResult};
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::error::Error;

use crate::literal::ipv4::{dot_and_octet, ipv4};

#[derive(PartialEq, Debug)]
pub struct NetPrefix {
    address: Ipv4Addr,
    prefix: u8,
}

fn u8_less_than_33(input: &str) -> IResult<&str, u8> {
    let (input, num) = complete::u8(input)?;
    if num > 32 {
        Err(nom::Err::Error(Error {
            input,
            code: ErrorKind::TooLarge,
        }))
    } else {
        Ok((input, num))
    }
}

fn prefix(input: &str) -> IResult<&str, u8> {
    let (input, _) = tag("/")(input)?;
    let (input, num) = u8_less_than_33(input)?;

    Ok((input, num))
}

pub fn net_prefix(i: &str) -> IResult<&str, NetPrefix> {
    let (i, r) = nom::branch::alt((
        net_prefix_short_1,
        net_prefix_short_2,
        net_prefix_short_3,
        net_prefix_no_short
    ))(i)?;

    Ok((i, r))
}

fn net_prefix_short_1(i: &str) -> IResult<&str, NetPrefix> {
    let (i, r) = complete::u8(i)?;
    let (i, prefix) = prefix(i)?;

    Ok((i, NetPrefix {
        address: Ipv4Addr::from([r, 0, 0, 0]),
        prefix,
    }))
}

fn net_prefix_short_2(i: &str) -> IResult<&str, NetPrefix> {
    let (i, o1) = complete::u8(i)?;
    let (i, o2) = dot_and_octet(i)?;
    let (i, prefix) = prefix(i)?;

    Ok((i, NetPrefix {
        address: Ipv4Addr::from([o1, o2, 0, 0]),
        prefix,
    }))
}

fn net_prefix_short_3(i: &str) -> IResult<&str, NetPrefix> {
    let (i, o1) = complete::u8(i)?;
    let (i, o2) = dot_and_octet(i)?;
    let (i, o3) = dot_and_octet(i)?;
    let (i, prefix) = prefix(i)?;

    Ok((i, NetPrefix {
        address: Ipv4Addr::from([o1, o2, o3, 0]),
        prefix,
    }))
}

fn net_prefix_no_short(i: &str) -> IResult<&str, NetPrefix> {
    let (i, address) = ipv4(i)?;
    let (i, prefix): (&str, u8) = prefix(i)?;

    Ok((i, NetPrefix { address, prefix }))
}

mod test {
    use crate::literal::net_prefix::{net_prefix, net_prefix_no_short, net_prefix_short_1, net_prefix_short_2, net_prefix_short_3, NetPrefix, prefix, u8_less_than_33};

    #[test]
    fn test_u8_less_than_33() {
        assert_eq!(Ok(("", 0)), prefix("/0"));
        assert_eq!(Ok(("", 32)), u8_less_than_33("32"));
        assert!(u8_less_than_33("33").is_err())
    }

    #[test]
    fn test_net_prefix_short_1() {
        assert_eq!(Ok(("", make_net_prefix("127.0.0.0", 24))), net_prefix_short_1("127/24"));
    }

    #[test]
    fn test_net_prefix_short_2() {
        assert_eq!(Ok(("", make_net_prefix("127.1.0.0", 24))), net_prefix_short_2("127.1/24"));
    }

    #[test]
    fn test_net_prefix_short_3() {
        assert_eq!(Ok(("", make_net_prefix("127.1.1.0", 24))), net_prefix_short_3("127.1.1/24"));
    }

    #[test]
    fn test_net_prefix_no_short() {
        assert_eq!(Ok(("", make_net_prefix("127.1.1.1", 24))), net_prefix_no_short("127.1.1.1/24"));
    }

    #[test]
    fn test_prefix() {
        assert_eq!(Ok(("", 0)), prefix("/0"));
        assert_eq!(Ok(("", 32)), prefix("/32"));
        assert!(prefix("/33").is_err())
    }

    #[test]
    fn test_net_prefix() {
        assert_eq!(Ok(("", make_net_prefix("127.0.0.0", 24))), net_prefix("127/24"));
        assert_eq!(Ok(("", make_net_prefix("127.16.0.0", 24))), net_prefix("127.16/24"));
        assert_eq!(Ok(("", make_net_prefix("127.16.64.0", 24))), net_prefix("127.16.64/24"));
        assert_eq!(Ok(("", make_net_prefix("127.16.64.1", 24))), net_prefix("127.16.64.1/24"));

        assert!(net_prefix("127.0.0.1").is_err())
    }

    fn make_net_prefix(ip: &str, prefix: u8) -> NetPrefix {
        if prefix > 32 {
            panic!("Prefix should be less than 33")
        }
        NetPrefix {
            prefix,
            address: ip.parse().unwrap(),
        }
    }
}