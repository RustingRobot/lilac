use std::{collections::HashMap, env, fs::{self}, path::Path};

use std::process::Command;

use crate::exit::{err_exit, Try};

use super::{lexer::{self, Iterator, LilacPath, Token}, SubsectionNode, TokenNode};

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

pub fn build_syntax_tree(tokens: &[Token]) -> Vec<TokenNode> {
    build_ast_layer(tokens, 0).0
}

fn build_ast_layer(tokens: &[Token], mut index: usize) -> (Vec<TokenNode>, usize) {
    let mut root = vec![];
    while index < tokens.len() {
        match &tokens[index] {
            Token::For(_, _, _) => {
                let (sub_tree, inc) = build_ast_layer(tokens, index + 1);
                root.push(TokenNode{ content: tokens[index].clone(), children: sub_tree });
                index = inc + 1;
            }
            Token::End(_) => {
                return (root, index);
            }
            _ => {
                root.push(TokenNode{ content: tokens[index].clone(), children: vec![] });
                index += 1;
            }
        }
    }
    (root, index)
}


pub fn parse_syntax_tree(nodes: &Vec<TokenNode>, content: &String, ctx: &HashMap<String, String>) -> String{
    let mut file_contents = String::new();
    for node in nodes {
        let temp_str: String;
        file_contents.push_str(match &node.content {
            Token::Block(s) => &content[s.start .. s.end],
            Token::Put(_, l) => {temp_str = parse_put(l, &ctx); &temp_str},
            Token::For(_, l, i) => {temp_str = parse_for(l, i.clone(), &node.children, &content, ctx.clone()); &temp_str},
            Token::Run(_, l, a) => {temp_str = parse_run(l, &ctx, a); &temp_str},
            _ => err_exit(&format!("invalid token in parsing stage: {:?}", node)),
        })
    };
    file_contents
}

fn parse_for(path: &LilacPath, iterator: Iterator, children: &Vec<TokenNode>, content: &String, mut ctx: HashMap<String, String>) -> String{
    let mut mod_path = path.clone();
    if mod_path.contains_var(){
        mod_path.resolve_vars(&ctx);
    }
    
    let mut build_string = String::new();
    let dir = mod_path.directory();
    // get all files / subsections that need to be iterated over
    let mut loop_elements: Vec<String> = vec![];
    
    if Path::new(&dir).is_file() {
        let file = fs::read_to_string(&dir).err_try(&format!("could not read file {}", dir));
        let tokens = lexer::extract_subsections(&file);
        let tree = build_subsection_tree(&file, tokens, dir);
        let mut children = tree.get_children(mod_path.sub_list(), mod_path.directory());
        //let mut child_paths: Vec<String> = children.iter().map(|s| format!("{}:{}", dir, s)).collect();
        loop_elements.append(&mut children);
    } else if Path::new(&dir).is_dir() {
        let paths = fs::read_dir(dir).err_try(&format!("could not read from path {}", dir));
        for path in paths{
            loop_elements.push(path.unwrap().path().display().to_string())
        }
    }

    //let loop_elements = vec![];
    for element in loop_elements {
        ctx.insert(iterator.iterator.clone(), element);
        build_string.push_str(&parse_syntax_tree(&children, content, &ctx).trim_start());
    }
    build_string.trim_end().to_owned()
}

fn parse_put(path: &LilacPath, ctx: &HashMap<String, String>) -> String{
    let mut mod_path = path.clone();
    if mod_path.contains_var(){
        mod_path.resolve_vars(ctx);
    }

    let file = fs::read_to_string(mod_path.directory()).err_try(&format!("could not read file {}", mod_path.path));
    let get_title = path.modifier().contains(&"title");

    if mod_path.contains_subsection() {
        
/*         if path.modifier().contains(&"title"){
            return match mod_path.sub_list().last(){
                Some(s) => s.to_string(),
                None => err_exit(&format!("oops this should not happen! \n (error in path:{})", mod_path.path))
            }
        } */

        let tokens = lexer::extract_subsections(&file);
        let tree = build_subsection_tree(&file, tokens, &mod_path.path);

        if let Some(m) = path.modifier().first(){
            if m.parse::<usize>().is_ok(){
                return tree.get_content(mod_path.sub_list(), mod_path.directory(), Some(m.parse::<usize>().unwrap()), get_title).to_owned()
            }
        }
        tree.get_content(mod_path.sub_list(), mod_path.directory(), None, get_title).to_owned()
    }else{
        if let Some(m) = path.modifier().first(){
            if m.parse::<usize>().is_ok(){
                let tokens = lexer::extract_subsections(&file);
                let tree = build_subsection_tree(&file, tokens, &mod_path.path);
                return tree.get_content(mod_path.sub_list(), mod_path.directory(), Some(m.parse::<usize>().unwrap()), get_title).to_owned()
            }
        }
        
        if get_title{
            return mod_path.file_name().to_owned()
        }

        file
    }
}

fn parse_run(path: &LilacPath, ctx: &HashMap<String, String>, arguments: &Vec<String>) -> String{
    println!("arguments: {:?}", arguments);

    let mut mod_path = path.clone();
    if mod_path.contains_var(){
        mod_path.resolve_vars(ctx);
    }

    let program;
    match Path::new(&mod_path.path).extension().unwrap().to_str(){
        Some("sh") => {
            program = "sh";
        },
        Some("js") => {
            program = "node";
        },
        _ => err_exit(&format!("file-extension is not supported as an executable ({})", mod_path.path))
    }
    let mut cmd = Command::new(program);
    let cmd = cmd.arg(mod_path.path.clone());

    for arg in arguments {
        cmd.arg(arg);
    }

    String::from_utf8_lossy(&cmd.output().err_try(&format!("could not launch {}", program)).stdout).trim().to_string()
}