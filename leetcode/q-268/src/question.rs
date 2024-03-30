pub fn process(_input: &str) -> Result<String, Box<dyn std::error::Error>> {
    Ok("q_268 - part 1".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "INPUT GOES HERE";

        let result = process(input).unwrap();
        let answer = "ANSWER GOES HERE".to_string();
        assert_eq!(result, answer);
    }
}
