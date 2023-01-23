extern crate rjson;
use rjson::{JsonParser,Result};
fn main() -> Result<()>{
    let x = JsonParser::parse("{\"hi\":123}")?;
    println!("{:#?}",x.object()?.get("hi")?.int()?);
    Ok(())
    
}
