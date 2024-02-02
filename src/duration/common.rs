use nom::bytes::complete::tag_no_case;
use nom::IResult;
use nom::sequence::tuple;

use crate::integer::integer;

pub fn number_and_char_no_case(char: &str) -> impl Fn(&str) -> IResult<&str, u32> + '_ {
    move |input| {
        let (input, (num, _)) = tuple((integer, tag_no_case(char)))(input)?;
        Ok((input, num))
    }
}

#[cfg(test)]
mod test {
    use crate::duration::common::*;
    use crate::integer::{MAX_INTEGER, MIN_INTEGER};

    #[test]
    fn test_number_and_char_no_case() {
        let min_upper = MIN_INTEGER.to_string() + "X";
        let min_lower = MIN_INTEGER.to_string() + "x";

        let max_upper = MAX_INTEGER.to_string() + "X";
        let max_lower = MAX_INTEGER.to_string() + "x";

        assert_eq!(Ok(("", MIN_INTEGER)), number_and_char_no_case("X")(&min_upper));
        assert_eq!(Ok(("", MIN_INTEGER)), number_and_char_no_case("x")(&min_lower));

        assert_eq!(Ok(("", MAX_INTEGER)), number_and_char_no_case("X")(&max_upper));
        assert_eq!(Ok(("", MAX_INTEGER)), number_and_char_no_case("x")(&max_lower));
    }
}