#[derive(PartialEq,Debug)]
struct Reveal {
    red: u32,
    green: u32,
    blue: u32,
}

impl Reveal {
    fn new(raw: &str) -> Self {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        raw.split(',').for_each(|val| {
            let (num, c) = val.trim().split_once(' ').unwrap();
            match c {
                "red" => {
                    r = num.parse::<u32>().unwrap();
                }
                "green" => {
                    g = num.parse::<u32>().unwrap();
                }
                "blue" => {
                    b = num.parse::<u32>().unwrap();
                }
                _ => panic!("found a non rgb color {val}"),
            }
        });
        Reveal {
            red: r,
            green: g,
            blue: b,
        }
    }
}

#[derive(PartialEq,Debug)]
struct Game {
    id: u32,
    reveals: Vec<Reveal>,
}

impl Game {
    fn new(raw: &str) -> Self {
        let (game, reveals) = raw.split_once(':').unwrap();
        let (_, num) = game.split_once(' ').unwrap();
        Game {
            id: num.parse::<u32>().unwrap(),
            reveals: reveals
                .split(';')
                .into_iter()
                .map(|r| Reveal::new(r))
                .collect(),
        }
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Game> {
    input.lines().map(|val| Game::new(val)).collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[Game]) -> u32 {
    input
        .iter()
        .map(|game| {
            if let Some(_) = game
                .reveals
                .iter()
                .find(|r| r.red > 12 || r.green > 13 || r.blue > 14)
            {
                0
            } else {
                game.id
            }
        })
        .sum()
}

#[aoc(day2, part2)]
fn solve_part2(input: &[Game]) ->u32{
    input.iter().map(|game|{
        //find max r g b across all reveals
        let max_r = game.reveals.iter().max_by(|x,y|{
            x.red.cmp(&y.red)
        }).unwrap().red;
        let max_g = game.reveals.iter().max_by(|x,y|{
            x.green.cmp(&y.green)
        }).unwrap().green;
        let max_b = game.reveals.iter().max_by(|x,y|{
            x.blue.cmp(&y.blue)
        }).unwrap().blue;
        max_r * max_g * max_b
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", Game{
        id: 1,
        reveals: vec![
            Reveal{red: 4,green: 0,blue: 3},
            Reveal{ red: 1, green: 2, blue: 6 },
            Reveal{ red: 0, green: 2, blue: 0 }
        ]
    })]
    #[case("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", Game{
        id: 2,
        reveals: vec![
            Reveal{red: 0,green: 2,blue: 1},
            Reveal{ red: 1, green: 3, blue: 4 },
            Reveal{ red: 0, green: 1, blue: 1 }
        ]
    })]
    #[case("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", Game{
        id: 3,
        reveals: vec![
            Reveal{red: 20,green: 8,blue: 6},
            Reveal{ red: 4, green: 13, blue: 5 },
            Reveal{ red: 1, green: 5, blue: 0 }
        ]
    })]
    #[case("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", Game{
        id: 4,
        reveals: vec![
            Reveal{red: 3,green: 1,blue: 6},
            Reveal{ red: 6, green: 3, blue: 0 },
            Reveal{ red: 14, green: 3, blue: 15 }
        ]
    })]
    #[case("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", Game{
        id: 5,
        reveals: vec![
            Reveal{red: 6,green: 3,blue: 1},
            Reveal{ red: 1, green: 2, blue: 2 },
        ]
    })]
    fn test_new_game(#[case] input: &str, #[case] expected: Game) {
        assert_eq!(Game::new(input), expected);
    }

    #[test]
    fn split_test() {
        assert_eq!(" 7 green".trim().split_once(' ').unwrap(), ("7", "green"))
    }
}
