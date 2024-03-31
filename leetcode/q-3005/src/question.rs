use std::collections::HashMap;

pub fn process(nums: Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let h = nums.iter().fold(HashMap::<i32, i32>::new(), |mut m, x| {
        *m.entry(*x).or_default() += 1;
        m
    });

    let mv = &h
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, v)| v)
        .unwrap();

    let r = &h
        .iter()
        .filter(|(k, v)| v == mv)
        .into_iter()
        .map(|(k, v)| v)
        .sum::<i32>();

    println!("{:?}", r);

    Ok(*r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![1, 2, 2, 3, 1, 4];

        let result = process(input).unwrap();
        let answer = 4;
        assert_eq!(result, answer);
    }
}
