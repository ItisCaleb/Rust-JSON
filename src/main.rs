pub mod json;
use json::JsonParser;
fn main() {
    let x = JsonParser::parse("{\"hi\":[123,\"bruh\"]}".to_string());
    match x {
        Ok(v)=>{
            println!("{}",v.as_object().unwrap().get("hi")
                .unwrap().as_array().unwrap().get(1).unwrap().as_string().unwrap());
        }
        Err(e)=>{
            println!("{}",e);
        }
    }
    
}
