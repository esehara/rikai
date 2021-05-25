{{problem}}

fn main() -> std::io::Result<()>{
    
{{limit}}
    
    input();
    answer();

    Ok(())
}

fn input() {
{{input}}

}

fn answer() {
{{output}}

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4)
    }
{{example}}
}