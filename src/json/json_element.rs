use std::collections::HashMap;

use super::{Token, TokenType,Result,JsonError};

macro_rules! jerr {
    ($err:expr) => {
        Err(JsonError::new($err.to_string()))
    };
}

pub trait JsonElement {
    fn is_array(&self)->bool{false}
    fn is_object(&self)->bool{false}
    fn is_primitive(&self)->bool{false}
    fn is_null(&self)->bool{false}
    fn as_array(&self)->Result<&JsonArray>{
        jerr!("JSON Element is not an array")
    }
    fn as_object(&self)->Result<&JsonObject>{
        jerr!("JSON Object is not an object")
    }
    fn as_string(&self)->Result<String>{
        jerr!("JSON Object is not a string")
    }
    fn as_int(&self)->Result<i32>{
        jerr!("JSON Object is not an int")
    }
    fn as_float(&self)->Result<f64>{
        jerr!("JSON Object is not a float")
    }
    fn as_bool(&self)->Result<bool>{
        jerr!("JSON Object is not a bool")
    }
    
}

pub struct JsonArray{
    children:HashMap<usize,Box<dyn JsonElement>>
}
pub struct JsonObject{
    children:HashMap<String,Box<dyn JsonElement>>
}
pub struct JsonPrimitive{
    value:Token
}
impl JsonElement for JsonArray {
    fn is_array(&self)->bool {true}
    fn as_array(&self)->Result<&JsonArray>{
        Ok(self)
    }
}
impl JsonArray {
    pub(crate) fn new()->Box<JsonArray>{
        JsonArray{
            children:HashMap::new()
        }.into()
    }
    pub(crate) fn get_children(&mut self)->&mut HashMap<usize,Box<dyn JsonElement>>{
        &mut self.children
    }

    pub fn get(&self,index:usize)->Result<&Box<dyn JsonElement>>{
        match self.children.get(&index) {
            Some(v)=>Ok(v),
            None=>jerr!(format!("value at index {} is undefined",index))
        }
        
    }
}

impl JsonElement for JsonObject {
    fn is_object(&self)->bool {true}
    fn as_object(&self)->Result<&JsonObject>{
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
    pub fn get(&self,key:&str)->Result<&Box<dyn JsonElement>>{
        match self.children.get(&key.to_string()) {
            Some(v)=>Ok(v),
            None=>jerr!(format!("key \"{}\" is undefined",key))
        }
    }
}

impl JsonElement for JsonPrimitive {
    fn is_primitive(&self)->bool {true}
    fn is_null(&self)->bool {
        self.value.token_type == TokenType::Null
    }
    fn as_string(&self)->Result<String> {
        if matches!(self.value.token_type,TokenType::String){
            Ok(self.value.text.clone())
        }else{
            jerr!("JSON Object is not a string")
        }
        
    }
    fn as_int(&self)->Result<i32> {
        if matches!(self.value.token_type,TokenType::Int){
            Ok(self.value.text.parse::<i32>().unwrap())
        }else{
            jerr!("JSON Object is not a int")
        }
    }
    fn as_float(&self)->Result<f64> {
        if matches!(self.value.token_type,TokenType::Float)||
            matches!(self.value.token_type,TokenType::Int){
            Ok(self.value.text.parse::<f64>().unwrap())
        }else{
            jerr!("JSON Object is not a float")
        }
    }
    fn as_bool(&self)->Result<bool> {
        if self.value.text == "true"{
            Ok(true)
        }else if self.value.text == "false"{
            Ok(false)
        }else {
            jerr!("JSON Object is not a bool")
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


