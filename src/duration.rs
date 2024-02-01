use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::IResult;

enum TtlUnit {
    W(u32),
    D(u32),
    H(u32),
    M(u32),
    S(u32),
}

fn ttl_unit(input: &str) -> IResult<&str, &str> {
    let (input, unit) = alt((
        tag_no_case("W"),
        tag_no_case("D"),
        tag_no_case("H"),
        tag_no_case("M"),
        tag_no_case("S"),
    ))(input)?;
    Ok((input, unit))
}