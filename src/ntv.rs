use crate::data::Data;

pub fn println(data: Data) -> Result<Option<Data>, ()> {
    println!("{}", data.0);
    Ok(None)
}