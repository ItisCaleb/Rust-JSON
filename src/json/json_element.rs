use std::collections::HashMap;

use crate::Serializable;

use super::{JsonError, Result, Token, JsonType};

macro_rules! jerr {
    ($err:expr) => {
        Err(JsonError::new($err.to_string()))
    };
}

///JSON element
///
///array() to get a JSON array
///
///object() to get a JSON object
///
///int() to get i32
///
///float() to get f64
///
///bool() to get bool
pub trait JsonElement {
    /// check element is a JSON array
    fn is_array(&self) -> bool {
        false
    }
    /// check element is a JSON object
    fn is_object(&self) -> bool {
        false
    }
    fn is_primitive(&self) -> bool {
        false
    }
    /// check element is null
    fn is_null(&self) -> bool {
        false
    }
    /// get element as a JSON array
    fn array(&self) -> Result<&JsonArray> {
        jerr!("JSON Element is not an array")
    }
    /// get element as a JSON object
    fn object(&self) -> Result<&JsonObject> {
        jerr!("JSON Element is not an object")
    }
    fn primitive(&self) -> Result<&JsonPrimitive>{
        jerr!("JSON Element is not a primitive")
    }
    fn int(&self) -> Result<i32>{
        jerr!("JSON Element is not a int")
    }

    fn float(&self) -> Result<f64>{
        jerr!("JSON Element is not a float")
    }

    fn string(&self) -> Result<String>{
        jerr!("JSON Element is not a string")
    }

    fn bool(&self) -> Result<bool>{
        jerr!("JSON Element is not a bool")
    }
}

///function as a JSON array
///
///get(index) to get item at index
///
///len() to retrieve length of array
pub struct JsonArray {
    children: HashMap<usize, Box<dyn JsonElement>>,
}

///function as a JSON object
///
///get(key) to get item of key
pub struct JsonObject {
    children: HashMap<String, Box<dyn JsonElement>>,
}

pub struct JsonPrimitive {
    value: JsonType,
}
impl JsonElement for JsonArray {
    fn is_array(&self) -> bool {
        true
    }
    fn array(&self) -> Result<&JsonArray> {
        Ok(self)
    }
}
impl JsonArray {
    pub fn new() -> Box<JsonArray> {
        JsonArray {
            children: HashMap::new(),
        }
        .into()
    }
    pub(crate) fn get_children(&self) -> &HashMap<usize, Box<dyn JsonElement>> {
        &self.children
    }
    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn is_empty(&self) -> bool {
        self.children.len() == 0
    }

    pub fn push<T:Serializable>(&mut self,item: T){
        self.children.insert(self.len(), item.serialize());
    }
    pub fn push_ele(&mut self,item: Box<dyn JsonElement>){
        self.children.insert(self.len(), item);
    }

    pub fn get(&self, index: usize) -> Result<&dyn JsonElement> {
        match self.children.get(&index) {
            Some(v) => Ok(v.as_ref()),
            None => jerr!(format!("value at index {} is undefined", index)),
        }
    }
}

impl JsonElement for JsonObject {
    fn is_object(&self) -> bool {
        true
    }
    fn object(&self) -> Result<&JsonObject> {
        Ok(self)
    }
}
impl JsonObject {
    pub fn new() -> Box<JsonObject> {
        JsonObject {
            children: HashMap::new(),
        }
        .into()
    }
    pub(crate) fn get_children(&self) -> &HashMap<String, Box<dyn JsonElement>> {
        &self.children
    }
    pub fn put<T:Serializable>(&mut self, key: &str,item: T) {
        self.children.insert(key.to_string(), item.serialize());
    }
    pub fn put_ele(&mut self, key: &str,item: Box<dyn JsonElement>){
        self.children.insert(key.to_string(), item);
    }

    pub fn get(&self, key: &str) -> Result<&dyn JsonElement> {
        match self.children.get(&key.to_string()) {
            Some(v) => Ok(v.as_ref()),
            None => jerr!(format!("key \"{}\" is undefined", key)),
        }
    }
}

impl JsonElement for JsonPrimitive {
    fn is_primitive(&self) -> bool {
        true
    }
    fn is_null(&self) -> bool {
        self.value == JsonType::Null
    }
    fn primitive(&self) -> Result<&JsonPrimitive> {
        Ok(self)
    }
    fn int(&self) -> Result<i32>{
        match self.get() {
            JsonType::Int(v)=>Ok(v),
            _=>jerr!("JSON Element is not a int")
        }
    }

    fn float(&self) -> Result<f64>{
        match self.get() {
            JsonType::Float(v)=>Ok(v),
            _=>jerr!("JSON Element is not a float")
        }
    }

    fn string(&self) -> Result<String>{
        match self.get() {
            JsonType::String(v)=>Ok(v),
            _=>jerr!("JSON Element is not a string")
        }
    }

    fn bool(&self) -> Result<bool>{
        match self.get() {
            JsonType::Bool(v)=>Ok(v),
            _=>jerr!("JSON Element is not a bool")
        }
    }
    
}

impl JsonPrimitive {
    pub(crate) fn new(value: JsonType) -> Box<JsonPrimitive> {
        JsonPrimitive { value }.into()
    }
    pub fn get(&self)->JsonType{
        self.value.clone()
    }
}
