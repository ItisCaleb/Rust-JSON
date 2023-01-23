use std::collections::HashMap;

use super::{JsonError, Result, Token, TokenType};

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
        jerr!("JSON Object is not an object")
    }
    /// get element as a string
    fn string(&self) -> Result<String> {
        jerr!("JSON Object is not a string")
    }
    /// get element as a i32
    fn int(&self) -> Result<i32> {
        jerr!("JSON Object is not an int")
    }
    /// get element as a f64
    fn float(&self) -> Result<f64> {
        jerr!("JSON Object is not a float")
    }
    /// get element as a bool
    fn bool(&self) -> Result<bool> {
        jerr!("JSON Object is not a bool")
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
    value: Token,
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
    pub(crate) fn new() -> Box<JsonArray> {
        JsonArray {
            children: HashMap::new(),
        }
        .into()
    }
    pub(crate) fn get_children(&mut self) -> &mut HashMap<usize, Box<dyn JsonElement>> {
        &mut self.children
    }
    pub fn len(&self) -> usize {
        self.children.len()
    }

    pub fn is_empty(&self) -> bool {
        self.children.len() == 0
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
    pub(crate) fn new() -> Box<JsonObject> {
        JsonObject {
            children: HashMap::new(),
        }
        .into()
    }
    pub(crate) fn get_children(&mut self) -> &mut HashMap<String, Box<dyn JsonElement>> {
        &mut self.children
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
        self.value.token_type == TokenType::Null
    }
    fn string(&self) -> Result<String> {
        if matches!(self.value.token_type, TokenType::String) {
            Ok(self.value.text.clone())
        } else {
            jerr!("JSON Object is not a string")
        }
    }
    fn int(&self) -> Result<i32> {
        if matches!(self.value.token_type, TokenType::Int) {
            Ok(self.value.text.parse::<i32>().unwrap())
        } else {
            jerr!("JSON Object is not a int")
        }
    }
    fn float(&self) -> Result<f64> {
        if matches!(self.value.token_type, TokenType::Float)
            || matches!(self.value.token_type, TokenType::Int)
        {
            Ok(self.value.text.parse::<f64>().unwrap())
        } else {
            jerr!("JSON Object is not a float")
        }
    }
    fn bool(&self) -> Result<bool> {
        if self.value.text == "true" {
            Ok(true)
        } else if self.value.text == "false" {
            Ok(false)
        } else {
            jerr!("JSON Object is not a bool")
        }
    }
}

impl JsonPrimitive {
    pub(crate) fn new(value: Token) -> Box<JsonPrimitive> {
        JsonPrimitive { value }.into()
    }
}
