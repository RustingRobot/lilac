use std::{collections::HashMap, path::Path, vec};

use regex::{escape, Regex, RegexBuilder};
use crate::{exit::{err_exit, err_list}, settings};

use super::Span;



#[derive(Debug, PartialEq, Clone)]
pub struct LilacPath{pub path: String}

impl LilacPath {
    const SUB_MARKER: char = ':';
    const MOD_MARKER: char = ';';
    const POSSIBLE_MODS: [&'static str; 2] = ["", "title"];

    pub fn check_path(&self){
        let dir = self.directory();
        let sub = self.subsection();

        if dir.contains(Self::SUB_MARKER) || sub.contains("/") {
            err_exit(&format!("invalid path format!\n{}", self.path));
        }

        if !Path::new(dir).exists(){
            err_exit(&format!("path or file does not exist!\n{}", self.path))
        }

        let mut modifiers = self.modifier();

        if modifiers.is_empty() {return;}
        if modifiers.first().unwrap().parse::<usize>().is_ok(){
            modifiers.remove(0);
        }

        modifiers.iter().for_each(
            |x| {
                
                if !Self::POSSIBLE_MODS.contains(x)
                    { if x.parse::<usize>().is_ok() {
                        err_exit(&format!("numeric modifier can only be the first modifier: {}\n{}",x , self.path))
                    } else {err_exit(&format!("unknown modifier: {}\n{}",x , self.path))}}
            }
        );
    }

    pub fn contains_var(&self) -> bool{
        self.path.contains("{")
    }

    pub fn contains_subsection(&self) -> bool{
        self.subsection().ne("")
    }

    pub fn file_name(&self) -> &str{
        match self.directory().rsplit_once('/') {
            Some((_,name)) => name,
            None => ""
        }
    }

    pub fn directory(&self) -> &str {
        match self.path.split_once(Self::SUB_MARKER) {
            Some((dir, _)) => dir,
            None => match self.path.split_once(Self::MOD_MARKER){
                Some((dir,_)) => dir,
                None => &self.path,
            }
        }
    }

    pub fn subsection(&self) -> &str {
        match self.path.split_once(Self::SUB_MARKER) {
            Some((_, sub)) => match sub.split_once(Self::MOD_MARKER){
                Some((sub,_)) => sub,
                None => sub,
            },
            None => ""
        }
    }

    pub fn modifier(&self) -> Vec<&str> {
        match self.path.split_once(Self::MOD_MARKER) {
            Some((_, modi)) => modi.split(Self::MOD_MARKER).collect(),
            None => vec![]
        }
    }

    pub fn sub_list(&self) -> Vec<&str>{
        let sub = self.subsection();
        if sub == "" {
            vec![]
        } else {
            sub.split(Self::SUB_MARKER).collect()
        }
    }

    pub fn resolve_vars(&mut self, ctx: &HashMap<String, String>){
        for (key, value) in ctx {
            self.path = self.path.replace(&format!("{{{}}}",key), value);
        }
        self.check_path();
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Iterator{pub iterator: String}

#[derive(Debug, PartialEq, Clone)]
pub struct Indent{pub count: usize}

#[derive(Debug, PartialEq, Clone)]
pub enum ErrType{
    EmptyCmd,
    InvalidCmd,
    WrongArgCount
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token{
    Block(Span),
    Command(Span),
    Put(Span, LilacPath, Vec<String>),
    For(Span, LilacPath, Iterator),
    End(Span),
    Run(Span, LilacPath, Vec<String>),
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

    let mut for_counter = 0;

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
                        if cmd_parts.len() < 2 { Token::Error(*span, ErrType::WrongArgCount) } else {
                            let arguments: Vec<String> = cmd_parts[2..cmd_parts.len()].iter().map(|&s| s.into()).collect();
                            Token::Put(*span, LilacPath{ path: cmd_parts[1].to_owned()}, arguments.clone())
                        }
                    }
                    "for" => {
                        if cmd_parts.len() != 4 { Token::Error(*span, ErrType::WrongArgCount) } else {
                            for_counter += 1;
                            Token::For(*span, LilacPath{ path: cmd_parts[1].to_owned()}, Iterator{ iterator: cmd_parts[3].to_owned()})
                        }
                    }
                    "end" => {
                        for_counter -= 1;
                        Token::End(*span)
                    }
                    "run" => { if cmd_parts.len() < 2 { Token::Error(*span, ErrType::WrongArgCount) } else {
                            let arguments: Vec<String> = cmd_parts[2..cmd_parts.len()].iter().map(|&s| s.into()).collect();
                            Token::Run(*span, LilacPath{ path: cmd_parts[1].to_owned()}, arguments.clone())
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
            Token::Put(_, path, _) | Token::For(_, path, _) | Token::Run(_, path, _) => {
                if !path.contains_var(){
                    path.check_path();
                }
            }
            _ => {}
        }
    }

    if for_counter > 0 {
        errors.push("Did not close all loops!".to_owned())
    } else if for_counter < 0 {
        errors.push("Tried to close more loops than were open!".to_owned())
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