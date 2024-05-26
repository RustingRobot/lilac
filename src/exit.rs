use std::process;

pub fn err_exit(message: &str) -> ! {
    let message = message.replace("\n", "\n      "); // indent new line
    println!("\u{1b}[31;1merror\u{1b}[0m {}", message);
    err(message)
}

pub fn err_list(messages: Vec<String>) -> ! {
    for m in &messages {
        let m = m.replace("\n", "\n      "); // indent new line
        println!("\u{1b}[31;1merror\u{1b}[0m {}", m);
    }
    err(messages.concat())
}

fn err(message: String) -> ! {
    if cfg!(test){
        panic!("{}", message)
    }
    process::exit(1);
}

pub trait Try<T> {
    fn err_try(self, msg: &str) -> T;
}

impl<T, E> Try<T> for Result<T, E> {
    fn err_try(self, msg: &str) -> T {
        match self {
            Ok(t) => t,
            Err(_) => err_exit(msg),
        }
    }
}