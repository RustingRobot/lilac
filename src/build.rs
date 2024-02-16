use std::{fs, path::Path, process};
use walkdir::WalkDir;


pub fn build(){
    if !Path::new("./_lilac").exists(){
        print!("This path does not contain a _lilac directory!");
        process::exit(1);
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

    for entry in WalkDir::new(".")
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| !e.path().starts_with("./_lilac")) {
        let original_path = entry.path();
        let linked_path = Path::new("./_lilac/build").join(original_path.strip_prefix(".").unwrap());

        if original_path.is_dir() {
            fs::create_dir_all(linked_path).unwrap();
        }else{
            fs::hard_link(original_path, linked_path).unwrap();
        }
    }
}