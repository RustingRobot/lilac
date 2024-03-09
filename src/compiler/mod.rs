pub mod lexer;
pub mod parser;

mod tests{
    use std::env;
    use std::sync::Once;
    use crate::compiler::lexer;

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
        lexer::extract_tokens("[[]]".to_owned());
    }

    #[test]
    #[should_panic(expected = "invalid command")]
    fn invalid_command() {
        init_env();
        lexer::extract_tokens("[[shgloopy path/or/something]]".to_owned());
    }

    #[test]
    #[should_panic(expected = "wrong amount of arguments")]
    fn not_enough_args_command() {
        init_env();
        lexer::extract_tokens("[[put]]".to_owned());
    }

    #[test]
    #[should_panic(expected = "wrong amount of arguments")]
    fn too_many_args_command() {
        init_env();
        lexer::extract_tokens("[[for path as n path]]".to_owned());
    }
}