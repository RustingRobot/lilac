use std::path::Path;

use regex::{escape, Regex, RegexBuilder};
use crate::{exit::{err_exit, err_list}, settings};

use super::{parser, Span};

#[derive(Debug, PartialEq)]
pub struct LilacPath{pub path: String, pub marker: char}

impl LilacPath {
    fn check_path(&self) -> bool{
        let dir = self.directory();
        let sub = self.subsection();

        if dir.contains(self.marker) || sub.contains("/") {
            err_exit(&format!("invalid path format!\n{}", self.path));
        }

        if !Path::new(dir).exists(){
            err_exit(&format!("path or file does not exist!\n{}", self.path))
        }
        true
    }

    fn contains_var(&self) -> bool{
        return self.path.contains("{");
    }

    fn directory(&self) -> &str {
        match self.path.split_once(self.marker) {
            Some((dir, _)) => dir,
            None => &self.path
        }
    }

    fn subsection(&self) -> &str {
        match self.path.split_once(self.marker) {
            Some((_, sub)) => sub,
            None => ""
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Iterator{pub iterator: String}

#[derive(Debug, PartialEq)]
pub struct Indent{pub count: usize}

#[derive(Debug, PartialEq)]
pub enum ErrType{
    EmptyCmd,
    InvalidCmd,
    WrongArgCount
}

#[derive(Debug, PartialEq)]
pub enum Token{
    Block(Span),
    Command(Span),
    Put(Span, LilacPath),
    For(Span, LilacPath, Iterator),
    End(Span),
    Run(Span, LilacPath),
    Subsection(Span, Indent),
    Error(Span, ErrType)
}

pub fn extract_commands(content: &str) -> Vec<Token>{
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
                            Token::Put(*span, LilacPath{ path: cmd_parts[1].to_owned(), marker: settings.subsection_marker})
                        }
                    }
                    "for" => {
                        if cmd_parts.len() != 4 { Token::Error(*span, ErrType::WrongArgCount) } else {
                            Token::For(*span, LilacPath{ path: cmd_parts[1].to_owned(), marker: settings.subsection_marker}, Iterator{ iterator: cmd_parts[3].to_owned()})
                        }
                    }
                    "end" => {
                        Token::End(*span)
                    }
                    "run" => { if cmd_parts.len() != 2 { Token::Error(*span, ErrType::WrongArgCount) } else {
                            Token::Run(*span, LilacPath{ path: cmd_parts[1].to_owned(), marker: settings.subsection_marker})
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

    //error handling:
    let mut errors: Vec<String> = vec![];
    for t in &tokens {
        match t {
            Token::Error(span, err_type) => {
                let error_msg = format!("{}\n{}", match err_type {
                    ErrType::EmptyCmd => "empty command!",
                    ErrType::InvalidCmd => "invalid command!",
                    ErrType::WrongArgCount => "wrong amount of arguments!",
                }, format!("\"{0}\" at {1} to {2}\n",&content[span.start..span.end] , span.start, span.end));
                errors.push(error_msg);
            }
            //check if path exists
            Token::Put(_, path) | Token::For(_, path, _) | Token::Run(_, path) => {
                if !path.contains_var(){
                    path.check_path();
                }
            }
            _ => {}
        }
    }
    if !errors.is_empty() {
        err_list(errors);
    }

    tokens
}

pub fn extract_subsections(content: &str) -> Vec<Token>{
    let mut tokens: Vec<Token> = Vec::new();
    let settings = settings::request_settings();
    let lilac_file = RegexBuilder::new(&format!("^{}{{1,}}.*", escape(&settings.subsection_marker.to_string()))).multi_line(true).build().expect("Regex error?");

    let matches = lilac_file.find_iter(&content);
    let mut last_block = 0;
    for m in matches {
        // don't add block if match is at start of file or directly after the previous match
        if last_block != m.start() {
            tokens.push(Token::Block(Span {start: last_block, end: m.start()}))
        }
        let mut counter = 0;
        for c in content[m.start()..m.end()].chars() {
            if c != settings.subsection_marker{
                break;
            }
            counter += 1;
        }

        tokens.push(Token::Subsection(Span {start: m.start(), end: m.end()}, Indent{count: counter}));
        last_block = m.end();
    }
    // add last block if no match is at the end of the file
    if last_block != content.len() {
        tokens.push(Token::Block(Span {start: last_block, end: content.len()}))
    }

    tokens
}

pub fn visualize_tokens(tokens: Vec<Token>, content: &str){
    for token in tokens{
        println!("{}",
        match token{
            Token::Block(s) => format!("{} {}", bold("Block:"), &content[s.start .. s.end]),
            Token::Command(s) => format!("{} {}", bold("Command:"), &content[s.start .. s.end]),
            Token::Put(s, p) => format!("{} {} {} {}", bold("Put Command:"), &content[s.start .. s.end], bold("Path:"), p.path),
            Token::For(s, p, i) => format!("{} {} {} {} {} {}", bold("For Command:"), &content[s.start .. s.end], bold("Path:"), p.path, bold("Iterator:"), i.iterator),
            Token::End(s) => format!("{} {}", bold("End Command:"), &content[s.start .. s.end]),
            Token::Run(s, p) => format!("{} {} {} {}", bold("Run Command:"), &content[s.start .. s.end], bold("Path:"), p.path),
            Token::Subsection(s, i) => format!("{} {} {} {}", bold("Subsection Command:"), &content[s.start .. s.end], bold("Indent:"), i.count),
            Token::Error(s, e) => format!("{} {} {} {:?}", bold("Error Command:"), &content[s.start .. s.end], bold("Error Type:"), e),
        });
    }
}

fn bold(txt: &str) -> String{
    return format!("\x1B[47m\x1B[30m{}\x1B[0m", txt);
}