use {{crate_name}}::question::process;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = include_str!("../../input_1.txt");
    let result = process(file);
    println!("{:?}", result);
    Ok(())
}
