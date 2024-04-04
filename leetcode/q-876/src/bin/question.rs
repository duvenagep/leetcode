use q_876::question::process;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = vec![1, 2, 3, 4, 5];
    let result = process(file);
    println!("{:?}", result);
    Ok(())
}
