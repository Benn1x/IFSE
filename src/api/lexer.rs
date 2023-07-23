pub enum Input {
    Exit,
    Cache,
    CacheSize,
    Empty,
    Input(String),
}

impl Input {
    pub fn new(inp: &String) -> Self {
        let input: Vec<&str> = inp.split_whitespace().collect();
        if input.is_empty() {
            return Self::Empty;
        }
        match input[0] {
            ":q" => Self::Exit,
            ":c" => Self::Cache,
            ":cs" => Self::CacheSize,
            _ => Self::Input(inp.to_owned()),
        }
    }
}

pub struct Command<'a> {
    command: String,
    args: Vec<&'a str>,
}

impl<'a> Command<'a> {
    pub fn new(command_base: Vec<&'a str>) -> Box<Self> {
        Box::new(Self {
            command: String::from(command_base[0]),
            args: command_base[1..].to_vec(),
        })
    }

    pub fn get_command(&self) -> &String {
        &self.command
    }

    pub fn get_args(&self) -> &Vec<&str> {
        &self.args
    }
}
