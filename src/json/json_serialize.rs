use crate::{JsonPrimitive,JsonType, JsonArray};
pub use crate::JsonElement;
pub use crate::JsonObject;

pub trait Serializable {
    fn serialize(&self) ->Box<dyn JsonElement>;
}

impl Serializable for i32 {
    fn serialize(&self) -> Box<dyn JsonElement> {
        JsonPrimitive::new(JsonType::Int(self.clone()))
    }
}

impl Serializable for usize {
    fn serialize(&self) -> Box<dyn JsonElement> {
        JsonPrimitive::new(JsonType::Int(self.clone() as i32))
    }
}

impl Serializable for f64 {
    fn serialize(&self) -> Box<dyn JsonElement> {
        JsonPrimitive::new(JsonType::Float(self.clone()))
    }
}

impl Serializable for bool {
    fn serialize(&self) -> Box<dyn JsonElement> {
        JsonPrimitive::new(JsonType::Bool(self.clone()))
    }
}

impl Serializable for String {
    fn serialize(&self) -> Box<dyn JsonElement> {
        JsonPrimitive::new(JsonType::String(self.clone()))
    }
}


impl <T: Serializable> Serializable for Vec<T> {
    fn serialize(&self) -> Box<dyn JsonElement> {
        let mut arr = JsonArray::new();
        for item in self{
            arr.push_ele(item.serialize());
        }
        arr
    }
}

impl <T: Serializable> Serializable for [T;1] {
    fn serialize(&self) -> Box<dyn JsonElement> {
        let mut arr = JsonArray::new();
        for item in self{
            arr.push_ele(item.serialize());
        }
        arr
    }
}
