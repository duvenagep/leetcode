use q_3005::question::process;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = vec![1, 2, 2, 3, 1, 4];
    let result = process(file);
    println!("{:?}", result);
    Ok(())
}
