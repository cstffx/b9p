use nom::IResult;

use crate::integer::integer;

mod common;
mod ttl_time_unix;

fn seconds(input: &str) -> IResult<&str, u32> {
    integer(input)
}

#[cfg(test)]
mod test {
    use crate::duration::*;
    use crate::integer::{MAX_INTEGER, MIN_INTEGER};

    #[test]
    fn test_seconds() {
        assert_eq!(Ok(("", MIN_INTEGER)), seconds(&MIN_INTEGER.to_string()));
        assert_eq!(Ok(("", MAX_INTEGER)), seconds(&MAX_INTEGER.to_string()));

        assert!(seconds("4294967296").is_err());
    }
}
