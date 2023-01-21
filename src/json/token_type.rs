#[derive(Debug)]
#[derive(Clone,Copy)]
#[derive(PartialEq)]
pub(crate) enum TokenType{
    LCurlyBracket,RCurlyBracket,LBracket,
    RBracket,String,Colon,Comma,Int,Float,Error,EOF,
    Bool,Null
}