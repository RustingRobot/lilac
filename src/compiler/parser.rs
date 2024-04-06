use crate::exit::err_exit;

use super::{lexer::Token, SubsectionNode, TokenNode};

pub fn build_subsection_tree<'a>(content: &'a str, mut tokens: Vec<Token>, file: &str) -> SubsectionNode<'a>{    
    let mut root_node = SubsectionNode{ name: "root", ..Default::default()};
    
    if let Token::Block(s) = tokens[0] {
        root_node.content = &content[s.start..s.end];
        tokens.remove(0);
    }
    // tokens is now correctly aligned with the first element being a Subsection and the following elements
    // switching between Block and Subsection
    tokens.chunks(2).for_each(|x| {
        if let [Token::Subsection(section_s, i), Token::Block(block_s)] = x {
            match root_node.add_node(content, (section_s, i), &content[block_s.start .. block_s.end], i.count - 1) {
                Err(_) => err_exit(&format!("indent error in file: {}\nunder the section: {}", file, &content[(section_s.start + usize::from(i.count))..section_s.end])), Ok(_) => {},
            }
        } else if let [Token::Subsection(s, i)] = x { // tokens was of uneven length and we are at the last element
            match root_node.add_node(content, (s, i), "", i.count - 1) {
                Err(_) => err_exit(&format!("indent error in file: {}\nunder the section: {}", file, &content[(s.start + usize::from(i.count))..s.end])), Ok(_) => {},
            }
        }
    });
    root_node
}

pub fn build_syntax_tree<'a>(content: &'a str, mut tokens: Vec<Token>, file: &str) -> TokenNode{
    todo!()
}

pub fn parse_syntax_tree(rootNode: TokenNode){

}