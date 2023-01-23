extern crate rjson;
use rjson::{JsonParser, Result, Serializable,to_json};
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


fn main() -> Result<()> {
    //let x = JsonParser::parse("{\"hi\":123}")?;
    let class = SAO{
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
    Ok(())
}
