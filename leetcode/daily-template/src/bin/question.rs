use {{crate_name}}::question::process;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = vec![];
    let result = process(file);
    println!("{:?}", result);
    Ok(())
}
