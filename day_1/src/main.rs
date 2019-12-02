
fn main() {
    let input_str = include_str!("../input.txt");
    let input: i32 = input_str.parse().expect("not a valid integer");
    let part1_answer = part1(input);
    println!("part1: {}", part1_answer);
}

fn part1(input: i32) -> i32 {
    /* your code here */
}

#[cfg(test)]
mod tests_part1 {
    use super::*;

    #[test]
    fn test_12() {
        assert_eq!(part1(12), 2);
    }

    #[test]
    fn test_14() {
        assert_eq!(part1(14), 2);
    }

    #[test]
    fn test_1969() {
        assert_eq!(part1(1969), 654);
    }

    #[test]
    fn test_100756() {
        assert_eq!(part1(100756), 33583);
    }
}
