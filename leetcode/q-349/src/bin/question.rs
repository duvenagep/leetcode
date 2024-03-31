use q_349::question::process;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let v1 = vec![1, 2, 2, 1];
    let v2 = vec![1, 2];
    let result = process(v1, v2);
    println!("{:?}", result);
    Ok(())
}
