use nom::IResult;

mod ipv4;
mod net_prefix;
mod boolean;
mod integer;

pub struct Block {
    _name: String,
}


/// Reconoce un entero de 32 bits.
///
/// # Arguments
///
/// * `i`:
///
/// returns: Result<(&str, u32), Err<Error<&str>>>

pub fn block(i:&str) -> IResult<&str, Block>{
    Ok((i, Block {
        _name: String::from("Hola mundo")
    }))
}

