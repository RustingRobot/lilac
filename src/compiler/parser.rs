use super::{lexer::Token};

pub struct commandNode{

}

pub struct SubsectionNode<'a>{
    pub content: &'a str,
    pub children: Vec<SubsectionNode<'a>>
}

pub fn build_subsection_tree<'a>(content: String, tokens: Vec<Token>) -> SubsectionNode<'a>{
    todo!()
}

pub fn build_syntax_tree(){
    
}