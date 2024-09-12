use std::collections::HashMap;
use std::io::Write;
use std::{fs, fs::File, path::Path};
use walkdir::WalkDir;
use regex::Regex;
use regex::escape;
use crate::compiler::lexer;
use crate::compiler::parser::build_syntax_tree;
use crate::compiler::parser::parse_syntax_tree;
use crate::exit::err_exit;
use crate::exit::Try;
use crate::settings;


pub fn build(){
    if !Path::new("./_lilac").exists(){
        err_exit("This path does not contain a _lilac directory!");
    }

    // delete all files, that we want to refresh. we could reserve them and only
    // generate new hard links for new files but we can't see if files were moved
    // and an old link that has the same name as the new file should be updated
    // regenerating hard links isn't expensive anyways and I'm planning to add
    // partial rebuilding on directories
    if Path::new("./_lilac/build").exists() {
        fs::remove_dir_all("./_lilac/build").unwrap();
        fs::create_dir("./_lilac/build").unwrap();
    }


    let settings = settings::request_settings();
    let lilac_file = Regex::new(&format!("{}.*?{}", escape(&settings.start_delimiter), escape(&settings.end_delimiter))).expect("Regex error?");

    for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.path().starts_with("./_lilac")) {
        let original_path = entry.path();
        let linked_path = Path::new("./_lilac/build").join(original_path.strip_prefix(".").unwrap());

        if original_path.is_dir() {
            // recreate file structure
            fs::create_dir_all(linked_path).unwrap();
        }else{
            let file_content = match fs::read_to_string(original_path) {
                Err(_) => {
                    println!("\u{1b}[33;1mwarning\u{1b}[0m skipping file: {}", original_path.to_string_lossy());
                    "".to_owned()
                },
                Ok(r) => r
            };
            // only create new file if it is going to be processed by lilac
            if lilac_file.is_match(&file_content){
                process_file(&linked_path, file_content);
            }else{
                // else just create a hard link
                fs::hard_link(original_path, linked_path).unwrap();
            }
        }
    }

    println!("\u{1b}[32;1mdone!\u{1b}[0m");
}

fn process_file(path: &Path, content: String){
    println!("\u{1b}[34;1minfo\u{1b}[0m processing file: {}", path.to_string_lossy());
    let mut f = File::create(path).unwrap();

    let tokens = lexer::extract_commands(&content);
    let tree = build_syntax_tree(&tokens);
    let new_contents = parse_syntax_tree(&tree, &content, &HashMap::new());
    f.write_all(new_contents.as_bytes()).err_try(&format!("could not write to file: {}", path.to_string_lossy()));
}