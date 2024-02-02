use nom::bytes::complete::tag;
use nom::IResult;

fn boolean_true(i: &str) -> IResult<&str, bool> {
    let (i, b) = nom::combinator::map(nom::branch::alt((
        tag("true"),
        tag("1"),
        tag("yes"))), |_: &str| true)(i)?;

    Ok((i, b))
}

fn boolean_false(i: &str) -> IResult<&str, bool> {
    let (i, b) = nom::combinator::map(nom::branch::alt((
        tag("false"),
        tag("0"),
        tag("no"))), |_: &str| false)(i)?;

    Ok((i, b))
}

pub fn boolean(i: &str) -> IResult<&str, bool> {
    let (i, b) = nom::branch::alt((
        boolean_true,
        boolean_false
    ))(i)?;

    Ok((i, b))
}

#[cfg(test)]
mod tests {
    use crate::boolean::*;

    #[test]
    fn test_boolean() {
        assert_eq!(Ok(("", true)), boolean("true"));
        assert_eq!(Ok(("", true)), boolean("1"));
        assert_eq!(Ok(("", true)), boolean("yes"));

        assert_eq!(Ok(("", false)), boolean("false"));
        assert_eq!(Ok(("", false)), boolean("0"));
        assert_eq!(Ok(("", false)), boolean("no"));
    }
}