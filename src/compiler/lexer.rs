use regex::{escape, Regex};
use crate::settings;

#[derive(Debug, Clone, Copy)]
struct Span{
    start: usize,
    end: usize
}

#[derive(Debug)]
struct Path{path: String}
#[derive(Debug)]
struct Iterator{iterator: String}
#[derive(Debug)]
enum ErrType{
    EmptyCmd,
    InvalidCmd,
    WrongArgCount
}

#[derive(Debug)]
enum Token{
    Block(Span),
    Command(Span),
    Put(Span, Path),
    For(Span, Path, Iterator),
    End(Span),
    Run(Span, Path),
    Subsection(Span),
    Error(Span, ErrType)
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
        tokens.push(Token::Command(Span {start: m.start(), end: m.end()}));
        last_block = m.end();
    }
    // add last block if no match is at the end of the file
    if last_block != content.len() {
        tokens.push(Token::Block(Span {start: last_block, end: content.len()}))
    }

    //turn command tokens into more specific tokens
    for t in &mut tokens {
        match t {
            Token::Command(span) => {
                let cmd_start = span.start + settings.start_delimiter.chars().count();
                let cmd_end = span.end - settings.end_delimiter.chars().count();

                let cmd_parts: Vec<&str> = content[cmd_start..cmd_end].split(' ').filter(|s| !(*s).is_empty()).collect();

                if cmd_parts.is_empty() {
                    *t = Token::Error(*span, ErrType::EmptyCmd);
                    continue;
                }

                *t = match cmd_parts[0].to_lowercase().as_str() {
                    "put" => {
                        if cmd_parts.len() != 2 { Token::Error(*span, ErrType::WrongArgCount) } else {
                            Token::Put(*span, Path{ path: cmd_parts[1].to_owned()})
                        }
                    }
                    "for" => {
                        if cmd_parts.len() != 4 { Token::Error(*span, ErrType::WrongArgCount) } else {
                            Token::For(*span, Path{ path: cmd_parts[1].to_owned()}, Iterator{ iterator: cmd_parts[3].to_owned()})
                        }
                    }
                    "end" => {
                        Token::End(*span)
                    }
                    "run" => { if cmd_parts.len() != 2 { Token::Error(*span, ErrType::WrongArgCount) } else {
                            Token::Run(*span, Path{ path: cmd_parts[1].to_owned()})
                        }
                    }
                    _ => {
                        Token::Error(*span, ErrType::InvalidCmd)
                    }
                };
            }
            _ => {}
        }
    }

    

    //println!("tokens {:?}", tokens);

    for t in tokens{
        match t {
            Token::Command(span) | Token::Block(span) | Token::Subsection(span) | Token::Put(span, _) | Token::For(span, _, _) | Token::End(span) | Token::Run(span, _) | Token::Error(span, _) => {
                let text: String = content.chars().skip(span.start).take(span.end - span.start).collect();
                print!("text block: {:?} ", text);
                println!("token: {:?}", t);
            }
            _ => {}
        }
    }
}