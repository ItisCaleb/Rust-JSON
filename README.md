# Rust-JSON
 A homemade JSON library

## Usage

### Simple JSON Parse

```Rust
extern crate rjson;
use rjson::{JsonParser,Result};
fn main() -> Result<()>{
    let json = "[{
        \"login\":true,
        \"player\":\"Asuna\"
        }]";
    let result = JsonParser::parse(&json)?;
    if result.array()?.get(0)?.object()?.get("login")?.bool()?{
        println!("Link Start!");
    }
    //or you can use indexing
    println!("{}",result[0]["player"].string()?);
    //btw you can also use patter matching
    if let JsonType::String(admin) = result["admin"].get_type(){
        println!("{}",admin);
    }
    Ok(())
}

```

### JSON Serialize


```Rust
use rjson::{Serializable,to_json};
#[derive(Serializable)]
struct SAO{
    admin:String,
    player:Vec<Player>,
}

#[derive(Serializable)]
struct Player{
    name: String,
    id: usize
}
fn main(){
    let sao = SAO{
        admin: "Kayaba Akihiko".to_string(),
        player: vec![
            Player{
                name:"Kirito".to_string(),
                id:0
            },
            Player{
                name:"Asuna".to_string(),
                id:1
            }
        ]
    };
    println!("{}", to_json(class));
}

```