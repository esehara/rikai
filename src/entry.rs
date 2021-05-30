use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Input {
    pub text: String,
    pub args: Vec<String>,
    pub args_line: String,
}

#[derive(Debug, Serialize)]
pub struct Example {
    pub text: String,
    pub pre: Vec<String>,
    pub pre_oneline: String,
    pub pre_is_oneline: bool,
    pub pre_has: bool,
}

impl Example {
    pub fn new(line: String, pre: Vec<String>) -> Example {
        Example {
            text: line,
            pre_is_oneline: &pre.len() == &1,
            pre_has: &pre.len() != &0,
            pre_oneline: (&pre.concat()).to_string(),
            pre: pre,
        }
    }
}
