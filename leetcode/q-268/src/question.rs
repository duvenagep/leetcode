pub fn process(nums: Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    Ok((1..=nums.len()).sum::<usize>() as i32 - nums.into_iter().sum::<i32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];

        let result = process(input).unwrap();
        let answer = 8;
        assert_eq!(result, answer);
    }
}
