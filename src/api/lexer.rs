pub enum Input<'a> {
    Exit,
    Cache,
    CacheSize,
    Command(Box<Command<'a>>),
    Input(String),
}

impl<'a> Input<'a> {
    pub fn new(inp: &'a String) -> Self {
        let input: Vec<&str> = inp.split_whitespace().collect();
        match input[0] {
            ":q" => Self::Exit,
            ":c" => Self::Cache,
            ":cs" => Self::CacheSize,
            str if str.starts_with(":") => Self::Command(Command::new(input)),
            _ => Self::Input(inp.clone()),
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
