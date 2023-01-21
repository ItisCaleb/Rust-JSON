use std::collections::VecDeque;


use super::{JsonElement, JsonArray, JsonObject,
            JsonPrimitive,Lexer,Token,TokenType,
            Result,JsonError};
pub struct JsonParser<'a>{
    tokens: &'a mut VecDeque<Token>,
    diagnostic: Vec<String>
}


impl JsonParser<'_>{ 
    pub fn parse(input: &str)-> Result<Box<dyn JsonElement>>{
        let mut lexer = Lexer::new(input.to_string());
        let (tokens ,diagnostic)= lexer.lex();
        if diagnostic.len()>0{
            return Err(JsonError::new(diagnostic.first().unwrap().to_string()))
        }
        let mut parser: JsonParser = JsonParser{
            tokens,
            diagnostic: vec![]
        };
        
        let json = parser.decide_parse();
        parser.tmatch(TokenType::EOF);
        if parser.diagnostic.len()>0{
            return Err(JsonError::new(parser.diagnostic.first().unwrap().to_string()));
        }
        Ok(json)
    }
    fn tmatch(&mut self,ttype:TokenType)->Token{
        let token = self.next();
        if token.token_type==ttype{
            token
        }else {
            let pos = token.position;
            if token.token_type == TokenType::EOF{
                self.diagnostic.push(format!("Unexpected end of JSON"));
            }else {
                self.diagnostic.push(format!("Unexpected token {} at position {}",
                token.text,pos));
            }
            self.tokens.push_front(token);
            Token::new(ttype,"",pos)
        }
    }
    fn decide_parse(&mut self)-> Box<dyn JsonElement>{
        let token = self.peek();
        match token{
            TokenType::LCurlyBracket=>{
                self.parse_object()
            },
            TokenType::LBracket=>{
                self.parse_array()
            },
            TokenType::Int|TokenType::Float|TokenType::String=>{
                self.parse_primitive()
            },
            _=>{
                self.parse_primitive()
            }
        }
    }
    fn parse_array(&mut self)-> Box<JsonArray>{
        self.tmatch(TokenType::LBracket);
        let mut arr = JsonArray::new();
        if self.cmp_type(TokenType::RBracket){
            self.tmatch(TokenType::RBracket);
            return arr;
        }
        let mut i=0;
        arr.get_children().insert(i, self.decide_parse());
        while !self.cmp_type(TokenType::EOF)
            && !self.cmp_type(TokenType::RBracket) {
            i+=1;
            self.tmatch(TokenType::Comma);
            arr.get_children().insert(i, self.decide_parse());
        }
        self.tmatch(TokenType::RBracket);
        arr
    }
    fn parse_object(&mut self)-> Box<JsonObject>{
        self.tmatch(TokenType::LCurlyBracket);
        let mut object = JsonObject::new();
        if self.cmp_type(TokenType::RCurlyBracket){
            self.tmatch(TokenType::RCurlyBracket);
            return object;
        }
        let (key, field) = self.parse_key_field();
        object.get_children().insert(key, field);
        while  !self.cmp_type(TokenType::EOF)
            && !self.cmp_type(TokenType::RCurlyBracket) {
            self.tmatch(TokenType::Comma);
            let (key, field) = self.parse_key_field(); 
            object.get_children().insert(key, field);
        }
        self.tmatch(TokenType::RCurlyBracket);
        object
    }
    fn parse_key_field(&mut self)->(String,Box<dyn JsonElement>){
        let key = self.tmatch(TokenType::String);
        self.tmatch(TokenType::Colon);
        let field = self.decide_parse();
        (key.text,field)
    }

    fn parse_primitive(&mut self)-> Box<JsonPrimitive>{
        JsonPrimitive::new(self.next())
    }

    fn cmp_type(&self, ttype:TokenType)->bool{
        self.peek()==ttype
    }
    
    fn peek(&self)-> TokenType{
        match self.tokens.front() {
            Some(t)=>t.token_type,
            None=>TokenType::Error
        }
    }

    fn next(&mut self)-> Token{
        match self.tokens.pop_front() {
            Some(t)=>t,
            None=> Token::new(TokenType::Error, "", 0)
        }
    }
}