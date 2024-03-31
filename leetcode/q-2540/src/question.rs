pub fn process(nums1: Vec<i32>, nums2: Vec<i32>) -> Result<i32, Box<dyn std::error::Error>> {
    let mut common = -1;
    let (mut i, mut j) = (0, 0);
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] == nums2[j] {
            common = nums1[i];
            break;
        } else if nums1[i] > nums2[j] {
            j += 1
        } else {
            i += 1
        }
    }
    Ok(common)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let nums1 = vec![1, 2, 3];
        let nums2 = vec![2, 4];

        let result = process(nums1, nums2).unwrap();
        let answer = 2;
        assert_eq!(result, answer);
    }
}
