use std::{fs, fs::File, path::Path};
use walkdir::WalkDir;
use regex::Regex;
use regex::escape;
use crate::compiler::lexer;
use crate::compiler::lexer::visualize_tokens;
use crate::exit::err_exit;
use crate::settings;


pub fn build(){
/*     let str = "root content\n=Headet\ntext\n==One\ntest\n==Two\nbest\n===cool\ntexo\n=Level\nsvell\n=Entry";
    let tokens = lexer::extract_subsections(str);
    let tree = parser::build_subsection_tree(str, tokens, "test/file.txt");
    tree.visualize();
    println!("{:?}",tree.contains(&["Headet","Two"]));
    println!("{:?}",tree.get(&[])); */
    let content = "
    <?xml version=1.0 encoding=UTF-8 ?>
    <rss version=2.0>
    
    <channel>
      <title>my RSS feed</title>
      <link>https://example.com</link>
      <description>news and more</description>
      <copyright>Copyright 2024, Me<\\copyright>
    [[for path/to/files as news]]
      <item>
        <title>[[put {news}:title]]</title>
        <link>https://example.com/news/[[put {news}.title]]</link>
        <description>[[put {news}:description]]</description>
        <author>name@email.com</author>
      </item>
    [[end]]
    </channel>
    
    </rss>
    [[run scripts/printSomething.sh]]";
    let tree = lexer::extract_commands(content);
    visualize_tokens(tree, content);
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