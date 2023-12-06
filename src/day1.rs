use aho_corasick::{AhoCorasick, Match};
use lazy_static::lazy_static;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|val| val.to_string()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[String]) -> u32 {
    input.iter().map(|val| decode_nums(val)).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[String]) -> u32 {
    input.iter().map(|val| decode_nums2(val)).sum()
}

fn decode_nums(code: &String) -> u32 {
    let dig1 = code.chars().into_iter().find(|e| e.is_numeric()).unwrap();

    let dig2 = code
        .chars()
        .into_iter()
        .rev()
        .find(|e| e.is_digit(10))
        .unwrap();
    let num = format!("{}{}", dig1, dig2);
    num.parse::<u32>().unwrap()
}

fn decode_nums2(code: &String) -> u32 {
    let matches: Vec<Match> = AC.find_overlapping_iter(code).map(|m| m).collect();
    let dig1 = code
        .chars()
        .into_iter()
        .enumerate()
        .find(|(i, c)| c.is_digit(10) || matches.first().and_then(|m|{
            if m.range().contains(i){
                Some(())
            }else{
                None
            }
        }).is_some())
        .unwrap();

    let dig2 = code
        .chars()
        .into_iter()
        .rev()
        .enumerate()
        .find(|(i, c)| c.is_digit(10) || matches.last().and_then(|m|{
            if m.range().contains(&(code.len()-i-1)){
                Some(())
            }else{
                None
            }
        }).is_some())
        .unwrap();

    let num = format!(
        "{}{}",
        if dig1.1.is_digit(10) {
            dig1.1
        } else {
            NUMS_C[matches.first().unwrap().pattern().as_usize()]
        },
        if dig2.1.is_digit(10) {
            dig2.1
        } else {
            NUMS_C[matches.last().unwrap().pattern().as_usize()]
        }
    );
    num.parse::<u32>().expect(format!("the num is {num}").as_str())
}

const NUMS_C: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

const PATTERNS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

lazy_static! {
    static ref AC: AhoCorasick = AhoCorasick::builder()
        .ascii_case_insensitive(true)
        .build(PATTERNS)
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use aho_corasick::PatternID;
    use rstest::rstest;

    #[rstest]
    #[case("1abc2", 12)]
    #[case("pqr3stu8vwx", 38)]
    #[case("a1b2c3d4e5f", 15)]
    #[case("treb7uchet", 77)]
    #[case("5lkr", 55)]
    #[case("55", 55)]
    fn decode1(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(decode_nums(&input.to_string()), expected);
    }

    #[rstest]
    #[case("two1nine", 29)]
    #[case("eightwothree", 83)]
    #[case("abcone2threexyz", 13)]
    #[case("xtwone3four", 24)]
    #[case("4nineeightseven2", 42)]
    #[case("zoneight234", 14)]
    #[case("7pqrstsixteen", 76)]
    #[case("six9mnfjmtsf2kfmznkxntninesevenrpmfjfpgsk", 67)]
    fn decode3(#[case] input: &str, #[case] expected: u32) {
        assert_eq!(decode_nums2(&input.to_string()), expected);
    }

    #[test]
    fn test_aho() {
        let foo = "six9mnfjmtsf2kfmznkxntninesevenrpmfjfpgsk";
        let matches: Vec<PatternID> = AC.find_iter(foo).map(|mat| mat.pattern()).collect();
        assert_eq!(
            vec![PatternID::must(5), PatternID::must(8), PatternID::must(6),],
            matches
        );
    }

    #[test]
    fn test_iter() {
        let a = [1, 2, 3];

        let mut iter = a.iter().enumerate().rev();

        assert_eq!(iter.next().unwrap().0, 2);
    }
}
