use crate::api::lexer::Command;

pub fn execute(command: &Command) {
    // TODO search with hemming distance for similar commands!!!
    match command.get_command().as_str() {
        ":h" => {
            for arg in command.get_args().iter() {
                match arg {
                    _ => {
                        println!("Unknown Argument for :h");
                        break;
                    }
                }
            }
        }
        _ => {
            println!("Unknown Command!");
        }
    }
}
