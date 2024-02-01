use nom::IResult;

mod ipv4;
mod net_prefix;
mod boolean;
mod integer;
mod percentage;
mod duration;

pub struct Block {
    _name: String,
}

pub fn block(i:&str) -> IResult<&str, Block>{
    Ok((i, Block {
        _name: String::from("Hola mundo")
    }))
}

