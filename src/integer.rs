use nom::character::complete;
use nom::IResult;

pub fn integer(i: &str) -> IResult<&str, u32> {
    let (i, ui) = complete::u32(i)?;
    Ok((i, ui))
}

#[cfg(test)]
mod tests {
    use crate::integer::integer;

    #[test]
    fn test_integer() {
        assert_eq!(Ok(("", 0)), integer("0"));
        assert_eq!(Ok(("", 4294967295)), integer("4294967295"));
        assert!(integer("4294967296").is_err());
        assert!(integer("-1").is_err());
        assert!(integer("-0").is_err());
    }
}