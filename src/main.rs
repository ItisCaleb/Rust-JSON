extern crate rjson;
use rjson::{JsonParser,Result};
fn main() -> Result<()>{
    let x = JsonParser::parse("bd")?;
    println!("{:#?}",x.as_object()?.get("1").unwrap().as_bool());
    Ok(())
    
}
