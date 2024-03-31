use q_2540::question::process;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let nums1 = vec![1, 2, 3];
    let nums2 = vec![2, 4];
    let result = process(nums1, nums2);
    println!("{:?}", result);
    Ok(())
}
