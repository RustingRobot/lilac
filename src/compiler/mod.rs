pub mod lexer;
pub mod parser;

mod tests{
    use std::env;
    use std::path::Path;
    use crate::compiler::lexer;

    fn switch_dir(){
        if !env::current_dir().unwrap().ends_with("test"){
            env::set_current_dir(env::current_dir().unwrap().join("test")).unwrap();
        }
    }

    #[test]
    #[should_panic(expected = "empty command")]
    fn empty_command() {
        switch_dir();
        lexer::build_token_tree("[[]]".to_owned());
    }

    #[test]
    #[should_panic(expected = "invalid command")]
    fn invalid_command() {
        switch_dir();
        lexer::build_token_tree("[[shgloopy path/or/something]]".to_owned());
    }

    #[test]
    #[should_panic(expected = "wrong amount of arguments")]
    fn not_enough_args_command() {
        switch_dir();
        lexer::build_token_tree("[[put]]".to_owned());
    }

    #[test]
    #[should_panic(expected = "wrong amount of arguments")]
    fn too_many_args_command() {
        switch_dir();
        lexer::build_token_tree("[[for path as n path]]".to_owned());
    }
}