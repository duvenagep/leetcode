pub fn process(nums1: Vec<i32>, nums2: Vec<i32>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut r = nums1
        .into_iter()
        .filter(|v| nums2.contains(v))
        .collect::<Vec<i32>>();

    r.sort();
    r.dedup();

    Ok(r)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![4, 9, 5];
        let input2 = vec![9, 4, 9, 8, 4];

        let result = process(input, input2).unwrap();
        let answer = vec![9, 4];
        assert_eq!(result, answer);
    }
}
