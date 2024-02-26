use regex::{escape, Regex};
use crate::settings;

#[derive(Debug)]
struct Span{
    start: usize,
    end: usize
}

#[derive(Debug)]
enum Token{
    Block(Span), // start end
    Delimiter(Span),
    Include(Span),
    Path(Span),
    Iterate(Span),
    End(Span),
    Run(Span),
    Subsection(Span)
}

pub fn build_token_tree(content: String){
    let mut tokens: Vec<Token> = Vec::new();
    let settings = settings::request_settings();
    let lilac_file = Regex::new(&format!("{}.*?{}", escape(&settings.start_delimiter), escape(&settings.end_delimiter))).expect("Regex error?");

    let matches = lilac_file.find_iter(&content);

    let mut last_block = 0;
    for m in matches {
        // don't add block if match is at start of file or directly after the previous match
        if last_block != m.start() {
            tokens.push(Token::Block(Span {start: last_block, end: m.start()}))
        }
        tokens.push(Token::Delimiter(Span {start: m.start(), end: m.end()}));
        last_block = m.end();
    }
    // add last block if no match is at the end of the file
    if last_block != content.len() {
        tokens.push(Token::Block(Span {start: last_block, end: content.len()}))
    }

    println!("tokens {:?}", tokens);

    for t in tokens{
        match t {
            Token::Delimiter(span) | Token::Block(span) => {
                let text: String = content.chars().skip(span.start).take(span.end - span.start).collect();
                println!("text block: {:?}", text)
            }
            _ => {}
        }
    }
}