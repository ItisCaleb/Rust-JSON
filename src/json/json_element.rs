use std::collections::HashMap;

use super::{Token, TokenType};

pub trait JsonElement {
    fn is_array(&self)->bool{false}
    fn is_object(&self)->bool{false}
    fn is_primitive(&self)->bool{false}
    fn as_array(&self)->Result<&JsonArray,String>{
        Err("JSON Element is not an array".to_string())
    }
    fn as_object(&self)->Result<&JsonObject,String>{
        Err("JSON Object is not an object".to_string())
    }
    fn as_string(&self)->Result<String,String>{
        Err("JSON Object is not a string".to_string())
    }
    fn as_int(&self)->Result<i32,String>{
        Err("JSON Object is not an int".to_string())
    }
    fn as_float(&self)->Result<f64,String>{
        Err("JSON Object is not a float".to_string())
    }
    fn as_bool(&self)->Result<bool,String>{
        Err("JSON Object is not a bool".to_string())
    }
    
}

pub struct JsonArray{
    children:HashMap<i32,Box<dyn JsonElement>>
}
pub struct JsonObject{
    children:HashMap<String,Box<dyn JsonElement>>
}
pub struct JsonPrimitive{
    value:Token
}
impl JsonElement for JsonArray {
    fn is_array(&self)->bool {true}
    fn as_array(&self)->Result<&JsonArray,String>{
        Ok(self)
    }
}
impl JsonArray {
    pub(crate) fn new()->Box<JsonArray>{
        JsonArray{
            children:HashMap::new()
        }.into()
    }
    pub(crate) fn get_children(&mut self)->&mut HashMap<i32,Box<dyn JsonElement>>{
        &mut self.children
    }
    pub fn get(&self,index:i32)->Option<&Box<dyn JsonElement>>{
        self.children.get(&index)
    }
}

impl JsonElement for JsonObject {
    fn is_object(&self)->bool {true}
    fn as_object(&self)->Result<&JsonObject,String>{
        Ok(self)
    }
}
impl JsonObject {
    pub(crate) fn new()->Box<JsonObject>{
        JsonObject{
            children:HashMap::new()
        }.into()
    }
    pub(crate) fn get_children(&mut self)->&mut HashMap<String,Box<dyn JsonElement>>{
        &mut self.children
    }
    pub fn get(&self,key:&str)->Option<&Box<dyn JsonElement>>{
        self.children.get(&key.to_string())
    }
}

impl JsonElement for JsonPrimitive {
    fn is_primitive(&self)->bool {true}
    fn as_string(&self)->Result<String,String> {
        if matches!(self.value.token_type,TokenType::String){
            Ok(self.value.text.clone())
        }else{
            Err("JSON Object is not a string".to_string())
        }
        
    }
    fn as_int(&self)->Result<i32,String> {
        if matches!(self.value.token_type,TokenType::Number){
            match self.value.text.parse::<i32>() {
                Ok(v)=>Ok(v),
                _=>Err("JSON Object is not a int".to_string())
            }
            
        }else{
            Err("JSON Object is not a number".to_string())
        }
    }
    fn as_float(&self)->Result<f64,String> {
        if matches!(self.value.token_type,TokenType::Number){
            match self.value.text.parse::<f64>() {
                Ok(v)=>Ok(v),
                _=>Err("JSON Object is not a int".to_string())
            }
        }else{
            Err("JSON Object is not a number".to_string())
        }
    }
    fn as_bool(&self)->Result<bool,String> {
        if self.value.text == "true"{
            Ok(true)
        }else if self.value.text == "false"{
            Ok(false)
        }else {
            Err("JSON Object is not a bool".to_string())
        }
    }
}

impl JsonPrimitive {
    pub(crate) fn new(value: Token)->Box<JsonPrimitive>{
        JsonPrimitive{
            value
        }.into()
    }
}


