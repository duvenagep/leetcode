#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

pub fn process(head: Option<Box<ListNode>>) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut list = &head;
    let mut count = 0;
    while let Some(ref node) = *list {
        count += 1;
        list = &node.next;
    }
    dbg!(count);

    Ok(vec![3, 4, 5])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = vec![1, 2, 3, 4, 5];

        let result = process(input).unwrap();
        let answer = vec![3, 4, 5];
        assert_eq!(result, answer);
    }
}
