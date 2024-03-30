pub fn process(_input: Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let r = _input.into_iter().max();
    Ok(r.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![3, 0, 1];

        let result = process(input).unwrap();
        let answer = 2;
        assert_eq!(result, answer);
    }
}
