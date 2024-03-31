use q_268::question::process;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let file = include_str!("../../input_1.txt");
    let file = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
    let result = process(file);
    println!("{:?}", result);
    Ok(())
}
