#[derive(Debug, Deserialize)]
pub struct Rule {
    pub input: String,
    pub output: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rules: Vec<Rule>,
}