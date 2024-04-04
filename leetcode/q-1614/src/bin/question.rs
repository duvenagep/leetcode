use q_1614::question::process;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = "(1)+((2))+(((3)))".to_string();
    let result = process(file);
    println!("{:?}", result);
    Ok(())
}
