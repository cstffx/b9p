use std::net::Ipv4Addr;

use nom::bytes::complete::tag;
use nom::character::complete;
use nom::IResult;

#[derive(PartialEq, Debug)]
pub struct MaskedIpv4Addr {
    address: Ipv4Addr,
    mask: u8,
}

fn dot_and_u8(i: &str) -> IResult<&str, u8> {
    let (i, _) = tag(".")(i)?;
    let (i, r) = complete::u8(i)?;
    Ok((i, r))
}

fn mask(i: &str) -> IResult<&str, u8>{
    let (i, _) = tag("/")(i)?;
    let (i, r) = complete::u8(i)?;
    Ok((i, r))
}

fn unmasked_ipv4(i: &str) -> IResult<&str, MaskedIpv4Addr> {
    let (i, octet) = complete::u8(i)?;
    let (i, octets) = nom::multi::many_m_n(3, 3, dot_and_u8)(i)?;
    Ok((i, MaskedIpv4Addr {
        address: Ipv4Addr::from([
            octet,
            octets[0],
            octets[1],
            octets[2]
        ]),
        mask: 0,
    }))
}

fn masked_ipv4(i: &str) -> IResult<&str, MaskedIpv4Addr> {
    let (i, r) = unmasked_ipv4(i)?;
    let (i, mask) = mask(i)?;
    Ok((i, MaskedIpv4Addr {
        address: r.address,
        mask
    }))
}

pub fn masked_ipv4_short_1(i: &str) -> IResult<&str, MaskedIpv4Addr> {
    let (i, r) = complete::u8(i)?;
    let (i, mask) = mask(i)?;
    Ok((i, MaskedIpv4Addr {
        address: Ipv4Addr::from([r, 0, 0, 0]),
        mask
    }))
}

pub fn masked_ipv4_short_2(i: &str) -> IResult<&str, MaskedIpv4Addr> {
    let (i, o1) = complete::u8(i)?;
    let (i, o2) = dot_and_u8(i)?;
    let (i, mask) = mask(i)?;
    Ok((i, MaskedIpv4Addr {
        address: Ipv4Addr::from([o1, o2, 0, 0]),
        mask
    }))
}

pub fn masked_ipv4_short_3(i: &str) -> IResult<&str, MaskedIpv4Addr> {
    let (i, o1) = complete::u8(i)?;
    let (i, o2) = dot_and_u8(i)?;
    let (i, o3) = dot_and_u8(i)?;
    let (i, mask) = mask(i)?;
    Ok((i, MaskedIpv4Addr {
        address: Ipv4Addr::from([o1, o2, o3, 0]),
        mask
    }))
}

pub fn ipv4_parse(i: &str) -> IResult<&str, MaskedIpv4Addr> {
    let (i, r) = nom::branch::alt((
        masked_ipv4,
        masked_ipv4_short_1,
        masked_ipv4_short_2,
        masked_ipv4_short_3,
        unmasked_ipv4
    ))(i)?;
    Ok((i, r))
}

mod test {
    use std::net::Ipv4Addr;

    use crate::ipv4::{masked_ipv4, mask, ipv4_parse, MaskedIpv4Addr, unmasked_ipv4, masked_ipv4_short_1};

    #[test]
    fn test_unmasked_ipv4() {
        assert_eq!(Ok(("", MaskedIpv4Addr {
            address: build_ip("192.168.52.1"),
            mask: 0,
        })), unmasked_ipv4("192.168.52.1"));

        assert!(unmasked_ipv4("192.158.52,1").is_err());
        assert!(unmasked_ipv4("256.0.0.1").is_err());
        assert!(unmasked_ipv4("a.0.0.1").is_err());
    }

    #[test]
    fn test_mask() {
        assert_eq!(Ok(("", 24)), mask("/24"));
        assert_eq!(Ok(("", 32)), mask("/32"));
        assert_eq!(Ok(("", 16)), mask("/16"));
    }

    #[test]
    fn test_masked_ipv4() {
        assert_eq!(Ok(("", MaskedIpv4Addr {
            address: build_ip("192.168.52.1"),
            mask: 24,
        })), masked_ipv4("192.168.52.1/24"));

        assert_eq!(Ok(("", MaskedIpv4Addr {
            address: build_ip("192.168.52.1"),
            mask: 24,
        })), ipv4_parse("192.168.52.1/24"));

        assert_eq!(Ok(("", MaskedIpv4Addr {
            address: build_ip("192.0.0.0"),
            mask: 24,
        })), masked_ipv4_short_1("192/24"));
    }

    fn build_ip(str: &str) -> Ipv4Addr {
        str.parse::<Ipv4Addr>().unwrap()
    }
}

