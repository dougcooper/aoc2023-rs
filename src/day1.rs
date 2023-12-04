type Code = String;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Code> {
    input
        .lines()
        .map(|val|{
            val.to_string()
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[String]) -> u32 {
    input.iter().map(|val|{
        decode_string(val)
    }).sum()
}

fn decode_string(code: &String)->u32{
    let dig1 = code.chars().into_iter().find(|e|{
        e.is_digit(10)
    }).unwrap();

    let dig2 = code.chars().into_iter().rev().find(|e|{
        e.is_digit(10)
    }).unwrap();
    let num = format!("{}{}",dig1,dig2);
    num.parse::<u32>().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("a1a",11)]
    #[case("a3g1a",31)]
    #[case("a31a",31)]
    #[case("5a1",51)]
    #[case("5ab",55)]
    #[case("ca1",11)]
    fn decode1(#[case] input:&str,#[case] expected: u32){
        assert_eq!(decode_string(&input.to_string()),expected);
    }
}