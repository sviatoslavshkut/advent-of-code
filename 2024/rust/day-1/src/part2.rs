use std::collections::HashMap;

pub fn process(input: &str) -> miette::Result<String> {
    let mut left = vec![];
    let mut right = HashMap::new();

    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        let count = right
            .entry(items.next().unwrap().parse::<i32>().unwrap())
            .or_insert(0);
        *count += 1;
    }
    let mut result: i32 = 0;

    for item in left {
        let weight = right.get(&item).unwrap_or(&0);
        result += item * weight;
    }

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use crate::part2::process;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        assert_eq!("31", process(input)?);
        Ok(())
    }
}
