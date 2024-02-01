use bytes::complete::tag;
use nom::{bytes, IResult};
use nom::character::complete;

mod ipv4;
mod net_prefix;

pub struct Block {
    _name: String,
}

/// Reconoce un valor booleano
/// # Arguments
///
/// * `i`:
///
/// returns: Result<(&str, bool), Err<Error<&str>>>
pub fn boolean(i: &str) -> IResult<&str, bool> {
    let (i, b) = nom::branch::alt((
        boolean_true,
        boolean_false
    ))(i)?;

    Ok((i, b))
}

/// Reconoce un valor booleano verdadero
///
/// # Arguments
///
/// * `i`:
///
/// returns: Result<(&str, bool), Err<Error<&str>>>
pub fn boolean_true(i: &str) -> IResult<&str, bool> {
    let (i, b) = nom::combinator::map(nom::branch::alt((
        tag("true"),
        tag("1"),
        tag("yes"))), |_: &str| true)(i)?;

    Ok((i, b))
}

/// Reconoce un valor booleano falso
///
/// # Arguments
///
/// * `i`:
///
/// returns: Result<(&str, bool), Err<Error<&str>>>
pub fn boolean_false(i: &str) -> IResult<&str, bool> {
    let (i, b) = nom::combinator::map(nom::branch::alt((
        tag("false"),
        tag("0"),
        tag("no"))), |_: &str| false)(i)?;

    Ok((i, b))
}

/// Reconoce un entero de 32 bits.
///
/// # Arguments
///
/// * `i`:
///
/// returns: Result<(&str, u32), Err<Error<&str>>>
pub fn integer(i: &str) -> IResult<&str, u32> {
    let (i, ui) = complete::u32(i)?;
    Ok((i, ui))
}

pub fn block(i:&str) -> IResult<&str, Block>{
    Ok((i, Block {
        _name: String::from("Hola mundo")
    }))
}

#[cfg(test)]
mod tests {
    use crate::{boolean, integer};

    #[test]
    fn test_integer() {
        assert_eq!(Ok(("", 0)), integer("0"));
        assert_eq!(Ok(("", 4294967295)), integer("4294967295"));
        assert!(integer("4294967296").is_err());
        assert!(integer("-1").is_err());
        assert!(integer("-0").is_err());
    }

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
