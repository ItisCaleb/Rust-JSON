use std::collections::BTreeMap;

use crate::{Serializable,JsonObject, JsonElement, JsonArray, JsonType};

pub fn to_json<T:Serializable>(object: T)->String{
    let ele = object.serialize();
    choose_make(&ele,0)
}
fn choose_make(ele: &Box<dyn JsonElement>,layer:usize)->String{
    match ele.get_type(){
        JsonType::Object=>make_object(ele.object().unwrap(),layer),
        JsonType::Array=>make_array(ele.array().unwrap(),layer),
        JsonType::String(str)=>format!("\"{}\"",str),
        JsonType::Bool(b)=>b.to_string(),
        JsonType::Float(f)=>f.to_string(),
        JsonType::Int(i)=>i.to_string(),
        JsonType::Null => "null".to_string(),
    }
}

fn make_object(ele: &JsonObject,layer: usize)->String{
    let sorted: BTreeMap<&String,&Box<dyn JsonElement>> = ele
        .get_children()
        .iter()
        .map(|(k,v)|(k,v))
        .collect();
    
    let obj:String = sorted.iter()
    .enumerate()
    .map(|(i,(key,item))|{
        if i == 0 {
            format!("  {}\"{}\": {}",
                "  ".repeat(layer),
                key,
                choose_make(item,layer+1))
        }else{
            format!(",\n  {}\"{}\": {}",
                "  ".repeat(layer),
                key,
                choose_make(item,layer+1))
        }
    }).collect();
    format!("{{\n{}\n{}}}",obj,"  ".repeat(layer))
}

fn make_array(ele: &JsonArray,layer:usize)->String{
    let arr:String = ele.get_children()
        .iter()
        .enumerate()
        .map(|(i,item)|{
            if i == 0 {
                format!("{}",choose_make(item,layer))
            }else{
                format!(",{}",choose_make(item,layer))
            }
        }).collect();
    format!("[{}]",arr)
}
