use super::{lexer::Token, SubsectionNode, TokenNode};

pub trait Visualize {
    fn visualize(&self);
}

impl<'a> Visualize for SubsectionNode<'a> {
    fn visualize(&self){
        self.visualize_layer(0);
    }
}

pub fn visualize_tokens(tokens: &Vec<Token>, content: &str){
    for token in tokens{
        println!("{}",
        match token{
            Token::Block(s) => format!("{} {}\n{}", bold("Block:"), bold(&format!("[{} - {}]", s.start, s.end)), &content[s.start .. s.end]),
            Token::Command(s) => format!("{} {}", bold("Command:"), &content[s.start .. s.end]),
            Token::Put(s, p, a) => format!("{} {} {} {}  {} {:?}", bold("Put Command:"), &content[s.start .. s.end], bold("Path:"), p.path, bold("Parameters:"), a),
            Token::For(s, p, i) => format!("{} {} {} {} {} {}", bold("For Command:"), &content[s.start .. s.end], bold("Path:"), p.path, bold("Iterator:"), i.iterator),
            Token::End(s) => format!("{} {}", bold("End Command:"), &content[s.start .. s.end]),
            Token::Run(s, p, a) => format!("{} {} {} {} {} {:?}", bold("Run Command:"), &content[s.start .. s.end], bold("Path:"), p.path, bold("Arguments:"), a),
            Token::Subsection(s, i) => format!("{} {} {} {}", bold("Subsection Command:"), &content[s.start .. s.end], bold("Indent:"), i.count),
            Token::Error(s, e) => format!("{} {} {} {:?}", bold("Error Command:"), &content[s.start .. s.end], bold("Error Type:"), e),
        });
    }
}

pub fn visualize_ast(tokens: &Vec<TokenNode>){
    visualize_ast_layer(tokens, 0);
}

fn visualize_ast_layer(tokens: &Vec<TokenNode>, level: usize) {
    for token in tokens {
        println!(r"{}> {:?}", "-".repeat(level), token.content);
        visualize_ast_layer(&token.children, level + 1);
    }
}

fn bold(txt: &str) -> String{
    return format!("\x1B[47m\x1B[30m{}\x1B[0m", txt);
}