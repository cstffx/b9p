use nom::bytes::complete::tag;
use nom::{IResult, sequence};
use crate::integer::integer;

pub fn percentage(input: &str) -> IResult<&str, u32> {
    let (input, (num, _)) = sequence::tuple((integer, tag("%")))(input)?;
    Ok((input, num))
}

#[cfg(test)]
mod tests {
    use crate::percentage::*;

    #[test]
    fn test_percentage() {
        assert_eq!(Ok(("", 0)), percentage("0%"));
        assert_eq!(Ok(("", 4294967295)), percentage("4294967295%"));
        assert!(percentage("4294967296%").is_err());
        assert!(percentage("-1%").is_err());
        assert!(percentage("-0%").is_err());
    }
}