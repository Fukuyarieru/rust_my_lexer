use crate::prelude::*;

pub struct Lexer {
    allowed_tokens: Vec<Box<dyn TokenTrait>> // consider dyn compatability
    relations: Vec<Relation>
}

impl Lexer {
    pub fn interpret_to_tokens(str: String) -> Vec<Token> {
        Vec::new()
    }
    pub fn interpret_to_relations(str: String) -> Vec<Relation> {
        Vec::new()
    }
    // if there is a matching token, we return Some(Token), if not we return None
    pub fn match_token(str: String) -> Option<Token> {
        None
    }
}