# Rust-JSON
 A homemade JSON library

## Usage

### Simple JSON Parse

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