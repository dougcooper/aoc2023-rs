use fancy_regex::Regex;
use lazy_static::lazy_static;

#[derive(Clone)]
pub struct MyMatch {
    pub is_numeric: bool,
    pub val: u32,
    pub start: usize,
    pub end: usize,
    pub is_gear: bool
}

type Matches = Vec<MyMatch>;
pub struct MatchSet(Option<Matches>, Option<Matches>, Option<Matches>);

impl Default for MatchSet {
    fn default() -> Self {
        Self(None, None, None)
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"\d+|[^A-Za-z0-9.]").unwrap();
}

fn find_matches(line: &str) -> Vec<MyMatch> {
    let mut matches = vec![];
    //parse regex
    RE.find_iter(line).for_each(|m| {
        if let Ok(m) = m {
            if let Ok(num) = m.as_str().parse::<u32>() {
                matches.push(MyMatch {
                    is_numeric: true,
                    val: num,
                    start: m.start(),
                    end: m.end(),
                    is_gear: false
                });
            } else {
                matches.push(MyMatch {
                    is_numeric: false,
                    val: 0,
                    start: m.start(),
                    end: m.end(),
                    is_gear: if m.as_str().contains("*"){true}else{false}
                });
            }
        }
    });
    matches
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<MatchSet> {
    //find matches in each line and collect all lines
    let matches:Vec<Matches> = input.lines().map(|line| find_matches(line)).collect();
    let mut triples = vec![];
    let mut i = 0;
    while i < matches.len(){
        triples.push(MatchSet{
            0: i.checked_sub(1).and_then(|i|{Some(matches.get(i).unwrap().to_vec())}),
            1: Some(matches.get(i).unwrap().to_vec()),
            2: i.checked_add(1).and_then(|i|{Some(matches.get(i).unwrap_or(&vec![]).to_vec())})
        });
        i+=1;
    }
    triples
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<MatchSet>) -> u32 {

    input
        .iter()
        .map(|triple| {
            process_line1(&triple)
        })
        .sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<MatchSet>) -> u32 {
    input
        .iter()
        .map(|triple| {
            process_line2(&triple)
        })
        .sum()
}

fn process_line1(set: &MatchSet) -> u32 {
    let mut numbers = vec![];
    //find numeric matches
    let middles = set.1.as_ref().unwrap();
    'outer: for (i, middle) in middles.iter().enumerate() {
        if middle.is_numeric {
            let range = middle.start.checked_sub(1).unwrap_or(middle.start)..middle.end+1;
            //check above
            if let Some(above) = set.0.as_ref(){
                for above in above {
                    if !above.is_numeric && range.contains(&above.start) {
                        numbers.push(middle.val);
                        continue 'outer;
                    }
                }
            }
            //check right
            if let Some(right) = set.1.as_ref().unwrap().get(i + 1) {
                if !right.is_numeric && right.end == range.end{
                    numbers.push(middle.val);
                    continue 'outer;
                }
            }
            if let Some(below) = set.2.as_ref(){
                //check below
                for below in below {
                    if !below.is_numeric && range.contains(&below.start) {
                        numbers.push(middle.val);
                        continue 'outer;
                    }
                }
            }

            //check left
            if let Some(i) = i.checked_sub(1) {
                if let Some(left) = set.1.as_ref().unwrap().get(i) {
                    if !left.is_numeric && left.start == range.start{
                        numbers.push(middle.val);
                        continue 'outer;
                    }
                }
            }
        }
    }
    numbers.iter().sum()
}

fn process_line2(set: &MatchSet) -> u32 {
    let mut numbers = vec![];
    //find numeric matches
    let middles = set.1.as_ref().unwrap();
    for (i, middle) in middles.iter().enumerate() {
        if middle.is_gear {
            let mut gear_nums = vec![];
            //check above
            if let Some(above) = set.0.as_ref(){
                for above in above {
                    let range = above.start.checked_sub(1).unwrap_or(above.start)..above.end+1;
                    if above.is_numeric && range.contains(&middle.start) {
                        gear_nums.push(above.val);
                    }
                }
            }
            //check right
            if let Some(right) = set.1.as_ref().unwrap().get(i + 1) {
                if right.is_numeric && right.start == middle.end{
                    gear_nums.push(right.val);
                }
            }
            if let Some(below) = set.2.as_ref(){
                //check below
                for below in below {
                    let range = below.start.checked_sub(1).unwrap_or(below.start)..below.end+1;
                    if below.is_numeric && range.contains(&middle.start) {
                        gear_nums.push(below.val);
                    }
                }
            }

            //check left
            if let Some(i) = i.checked_sub(1) {
                if let Some(left) = set.1.as_ref().unwrap().get(i) {
                    if left.is_numeric && left.end == middle.start{
                        gear_nums.push(left.val);
                    }
                }
            }
            if gear_nums.len()==2{
                numbers.push(gear_nums[0]*gear_nums[1]);
            }
        }
    }
    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use std::iter::zip;
    use unindent::Unindent;
    use rstest::rstest;

    use super::*;

    #[test]
    fn test_regex() {
        let mut m = RE.find_iter("467..114..");
        assert_eq!(m.next().unwrap().unwrap().as_str(), "467");
        assert_eq!(m.next().unwrap().unwrap().as_str(), "114");
    }

    #[rstest]
    #[case("467..114..", vec![467,114])]
    #[case("..35..633.",vec![35,633])]
    #[case("...*......",vec![0])]
    #[case("......#...",vec![0])]
    fn test_matcher(#[case] input: &str, #[case] result: Vec<u32>) {
        let m = find_matches(input);
        let m: Vec<u32> = m.iter().map(|m| m.val).collect();
        assert_eq!(m, result);
    }

    #[rstest]
    #[case(r"...*......
             ..35..633.
             ......#...",668)]
    #[case(r"617*......
             .....+.58.
             ..592.....",0)]
    #[case(r"......#...
             617*......
             .....+.58.",617)]
    fn test_process_line1(#[case] input: &str, #[case] result: u32) {
        let u = input.unindent();
        let m = input_generator(u.as_str());
        let p = process_line1(&m[1]);
        assert_eq!(p, result);
    }

    #[test]
    fn test_solver_part1() {
        let input = 
          r"467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..";
        let u = input.unindent();
        let m = input_generator(u.as_str());
        let results = [467, 0, 35 + 633, 0, 617, 0, 592, 755, 0, 664 + 598];
        for (triple, result) in zip(&m, results) {
            let p = process_line1(&triple);
            assert_eq!(p, result);
        }

        assert_eq!(solve_part1(&m), 4361);
    }

    #[rstest]
    #[case(r"467..114..
             ...*......
             ..35..633.",16345)]
    #[case(r"......#...
             617*......
             .....+.58.",0)]
    #[case(r"......755.
             ...$.*....
             .664.598..",451490)]
    fn test_process_line2(#[case] input: &str, #[case] result: u32) {
        let u = input.unindent();
        let m = input_generator(u.as_str());
        let p = process_line2(&m[1]);
        assert_eq!(p, result);
    }
}
