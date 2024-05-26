use std::{fs, fs::File, path::Path};
use walkdir::WalkDir;
use regex::Regex;
use regex::escape;
use crate::compiler::lexer;
use crate::compiler::lexer::LilacPath;
use crate::compiler::parser;
use crate::compiler::parser::build_subsection_tree;
use crate::compiler::visualize::visualize_ast;
use crate::compiler::visualize::visualize_tokens;
use crate::compiler::visualize::Visualize;
use crate::exit::err_exit;
use crate::exit::Try;
use crate::settings;


pub fn build(){
    let path: LilacPath = LilacPath {path: "test_subsec.txt:1:name".to_owned(), marker: ':'};
    let dir = path.directory();
    let mut elem: Vec<String> = vec![];
    
    let mut file = fs::read_to_string(&dir).err_try(&format!("could not read file {}", dir));
    let tokens = lexer::extract_subsections(&file);
    let tree = build_subsection_tree(&file, tokens, dir);
    elem.append(&mut tree.get_children(path.sub_list()));
    
    print!("{:?}", elem);
/*     let content = fs::read_to_string("test_text.txt").err_try("Should have been able to read the file");

    let tree = lexer::extract_commands(&content);
    visualize_tokens(&tree, &content);
    let ast = parser::build_syntax_tree(&tree);
    visualize_ast(&ast); */
    return;

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
                    println!("skipping {}", original_path.to_string_lossy());
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
}

fn process_file(path: &Path, content: String){
    print!("in {:?} ", path);
    File::create(path).unwrap();
    lexer::extract_commands(&content);
}