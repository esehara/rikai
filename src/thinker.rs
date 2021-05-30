#[derive(Debug, Clone)]
pub enum ParameterKind {
    Only,
    NoIdea,
}

#[derive(Debug)]
pub enum InputLine {
    One,
    Two,
    NoIdea,
}

#[derive(Debug, Clone)]
pub struct Parameter {
    pub name: String,
    pub kind: ParameterKind,
}

impl Parameter {
    pub fn new(text: &String) -> Parameter {
        Parameter {
            name: text.clone(),
            kind: ParameterKind::Only,
        }
    }
}

#[derive(Debug)]
pub struct Thinker {
    pub parameters: Vec<Parameter>,
    pub inputs: InputLine,
}

impl Thinker {
    fn choice_input_line(text: &Vec<String>) -> InputLine {
        match text.len() {
            1 => InputLine::One,
            2 => InputLine::Two,
            _ => InputLine::NoIdea,
        }
    }

    fn understand_parameter(text: &Vec<String>) -> Vec<Parameter> {
        text.into_iter()
            .map(|x| x.split(" "))
            .flatten()
            .map(|x| x.to_string())
            .map(|x| Parameter::new(&x))
            .collect::<Vec<Parameter>>()
    }
    pub fn new(text: Vec<String>) -> Thinker {
        Thinker {
            parameters: Thinker::understand_parameter(&text),
            inputs: Thinker::choice_input_line(&text),
        }
    }

    pub fn to_args(&self) -> Vec<String> {
        self.parameters
            .clone()
            .into_iter()
            .map(|x| x.name.to_lowercase().clone())
            .collect::<Vec<String>>()
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }

    #[test]
    fn oneline() {}
}
