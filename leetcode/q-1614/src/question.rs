pub fn process(_input: String) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(_input
        .chars()
        .fold((0, 0), |(mut acc, mut count), c| {
            match c {
                '(' => {
                    count += 1;
                }
                ')' => count -= 1,
                _ => {}
            }
            if acc < count {
                acc = count
            }
            (acc, count)
        })
        .0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "(1+(2*3)+((8)/4))+1".to_string();

        let result = process(input).unwrap();
        let answer = 3;
        assert_eq!(result, answer);
    }
}
