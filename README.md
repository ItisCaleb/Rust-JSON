# Rust-JSON
 A homemade JSON library

## Usage

```Rust
extern crate rjson;
use rjson::{JsonParser,Result};
fn main() -> Result<()>{
    let json = "[{
            \"hi\":123
        },{
            \"hi\":456
        },{
            \"hi\":789,
            \"kirito\":true
        }]";
    let result = JsonParser::parse(&json)?;
    let result = r.as_array()?;
    if result.get(2)?.as_object()?.get("kirito").as_bool()?{
        println("Link Start!");
    }
    Ok(())
}

```