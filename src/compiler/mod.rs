use crate::exit::err_exit;

use self::lexer::{Indent, Token};

pub mod lexer;
pub mod parser;
pub mod visualize;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Span{
    pub start: usize,
    pub end: usize
}

#[derive(Debug, PartialEq)]
pub struct SubsectionNode<'a>{
    pub name: &'a str,
    pub content: &'a str,
    pub children: Vec<SubsectionNode<'a>>
}

impl<'a> SubsectionNode<'a> {
    fn add_node(&mut self, content: &'a str, s_info: (&Span, &Indent), block_text: &'a str, indent: usize) -> Result<(), ()>{
        if indent > 0 {
            if let Some(last) = self.children.last_mut() {
                return last.add_node(content, s_info, block_text, indent - 1);
            } else {
                return Err(());
            }
        } else {
            self.children.push(
                SubsectionNode { 
                name: &content[(s_info.0.start + usize::from(s_info.1.count))..s_info.0.end].trim(), 
                content: block_text, 
                children: vec![] 
            });
            return Ok(())
        }
    }

    pub fn contains(&self, path: &[&str]) -> bool {
        if path.is_empty() {
            true
        } else {
            match self.children.iter().find_map(|c| {if c.name == path[0] {Some(c)} else {None}}) {
                Some(c) => c.contains(&path[1..]),
                None => false,
            }
        }
    }

    pub fn get_content(&self, mut path: Vec<&str>, file: &str, opt_index: Option<usize>, get_name: bool) -> &str {
        if path.is_empty() {
            match opt_index {
                Some(i) =>
                    match self.children.iter().nth(i){
                        Some(n) => if get_name {n.name} else {n.content.trim()},
                        None => err_exit(&format!("index out of bound: {}", i))
                    },
                None =>if get_name {self.name} else {self.content.trim()}
            }
        } else {
            match self.children.iter().find_map(|c| {if c.name == path[0] {Some(c)} else {None}}) {
                Some(c) => c.get_content(path.drain(1..).collect(), file, opt_index, get_name),
                None => err_exit(&format!("subsection does not exist: {:?} in file: {}", path, file)),
            }
        }
    }

    pub fn get_children(&self, mut path: Vec<&str>, file: &str) -> Vec<String> {
        if path.is_empty() {
            self.children.iter().map(|child| format!("{}:{}", file, child.name).to_owned()).collect()
        } else {
            match self.children.iter().find_map(|c| {if c.name == path[0] {Some(c)} else {None}}) {
                Some(c) => c.get_children(path.drain(1..).collect(), &format!("{}:{}",file,c.name)),
                None => err_exit(&format!("subsection does not exist: {:?} in file: {}", path, file)),
            }
        }
    }

    fn visualize_layer(&self, level: usize) {
        println!(r"{}> {}: {}", "-".repeat(level), self.name, self.content.replace("\n", "\\n"));
        self.children.iter().for_each(|c| c.visualize_layer(level + 1));
    }
}

impl<'a> Default for SubsectionNode<'a> {
    fn default() -> SubsectionNode<'a> {
        SubsectionNode {
            name: "",
            content: "",
            children: vec![]
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct TokenNode{
    pub content: Token,
    pub children: Vec<TokenNode>
}

mod tests{
    use std::env;
    use std::sync::Once;
    use crate::{compiler::{lexer::{self, Indent, LilacPath, Token::*}, Span}, settings};

    static INIT: Once = Once::new();

    fn init_env(){
        INIT.call_once(|| {
            env::set_current_dir(env::current_dir().unwrap().join("test")).unwrap();
        });
    }

    #[test]
    #[should_panic(expected = "empty command")]
    fn empty_command() {
        init_env();
        lexer::extract_commands("[[]]");
    }

    #[test]
    #[should_panic(expected = "invalid command")]
    fn invalid_command() {
        init_env();
        lexer::extract_commands("[[shgloopy path/or/something]]");
    }

    #[test]
    #[should_panic(expected = "wrong amount of arguments")]
    fn not_enough_args_command() {
        init_env();
        lexer::extract_commands("[[put]]");
    }

    #[test]
    #[should_panic(expected = "wrong amount of arguments")]
    fn too_many_args_command() {
        init_env();
        lexer::extract_commands("[[for path as n path]]");
    }

    #[test]
    fn common_commands() {
        init_env();
        let settings = settings::request_settings();
        let tokens = lexer::extract_commands("\
            put block here:[[put path/to/file.txt]]\
            for block here:[[for path/to/files as loop]]\
            end block here:[[end]]\
            run block here:[[run scripts/printSomething.sh]]\
            put block here:[[put path/to/file.txt:subsection]]\
            put block here:[[put path/to/file.txt:sub:subsection]]");
        assert_eq!(tokens, 
        vec![
            Block(Span { start: 0, end: 15 }), 
            Put(Span { start: 15, end: 39 }, LilacPath { path: "path/to/file.txt".into() }), 
            Block(Span { start: 39, end: 54 }), 
            For(Span { start: 54, end: 83 }, LilacPath { path: "path/to/files".into() }, lexer::Iterator { iterator: "loop".into() }), 
            Block(Span { start: 83, end: 98 }), 
            End(Span { start: 98, end: 105 }), 
            Block(Span { start: 105, end: 120 }), 
            Run(Span { start: 120, end: 153 }, LilacPath { path: "scripts/printSomething.sh".into() }, Vec::new()), 
            Block(Span { start: 153, end: 168 }), 
            Put(Span { start: 168, end: 203 }, LilacPath { path: "path/to/file.txt:subsection".into() }), 
            Block(Span { start: 203, end: 218 }), 
            Put(Span { start: 218, end: 257 }, LilacPath { path: "path/to/file.txt:sub:subsection".into() })]);
    }

    #[test]
    #[should_panic(expected = "invalid path")]
    fn invalid_path_1() {
        init_env();
        lexer::extract_commands("[[put path/to:subsection/file]]");
    }

    #[test]
    #[should_panic(expected = "invalid path")]
    fn invalid_path_2() {
        init_env();
        lexer::extract_commands("[[put :path/to/file]]");
    }

    #[test]
    #[should_panic(expected = "path or file does not exist")]
    fn path_not_found() {
        init_env();
        lexer::extract_commands("[[put this/file/does/not/exist]]");
    }

    #[test]
    #[should_panic(expected = "subsection does not exist")]
    fn subsection_not_found() {
        init_env();
        lexer::extract_commands("[[put path/to/file.txt:does_not_exist]]");
    }

    #[test]
    fn common_subsections(){
        init_env();
        let tokens = lexer::extract_subsections("top level\n:one\nheader\n::one/one\ncontent\n:two\nheader\n::two/one\n:::two/one/one");
        assert_eq!(tokens, 
            vec![
                Block(Span { start: 0, end: 10 }), 
                Subsection(Span { start: 10, end: 14 }, Indent { count: 1 }), 
                Block(Span { start: 14, end: 22 }), 
                Subsection(Span { start: 22, end: 31 }, Indent { count: 2 }), 
                Block(Span { start: 31, end: 40 }), 
                Subsection(Span { start: 40, end: 44 }, Indent { count: 1 }), 
                Block(Span { start: 44, end: 52 }), 
                Subsection(Span { start: 52, end: 61 }, Indent { count: 2 }), 
                Block(Span { start: 61, end: 62 }), 
                Subsection(Span { start: 62, end: 76 }, Indent { count: 3 })]);
    }
}