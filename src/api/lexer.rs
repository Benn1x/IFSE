pub enum Input {
    Exit,
    Cache,
    CacheSize,
    Help,
    Input(String),
}

impl Input {
    pub fn new(inp: String) -> Self {
        let input: Vec<&str> = inp.split(" ").collect();
        match input[0] {
            ":q" => Self::Exit,
            ":c" => Self::Cache,
            ":cs" => Self::CacheSize,
            ":h" => Self::Help,
            _ => Self::Input(inp),
        }
    }
}
