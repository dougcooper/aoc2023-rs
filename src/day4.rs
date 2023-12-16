#[derive(PartialEq,Debug)]
pub struct Card {
    pub id: u32,
    pub winning_numbers: Vec<u32>,
    pub my_numbers: Vec<u32>,
}

impl Card{
    fn new(input:&str)->Self{
        let (card_no, numbers) = input.split_once(':').unwrap();
        let (_, id) = card_no.split_once(' ').unwrap();
        let (w, m) = numbers.split_once('|').unwrap();
        let winning:Vec<&str> = w.trim().split(' ').filter(|s|!s.is_empty()).collect();
        let mine:Vec<&str> = m.trim().split(' ').filter(|s|!s.is_empty()).collect();

        Card {
            id: id.trim().parse::<u32>().expect(format!("id is {id}").as_str()),
            winning_numbers: winning.iter().map(|num|{num.parse::<u32>().unwrap()}).collect(),
            my_numbers: mine.iter().map(|num|{num.parse::<u32>().unwrap()}).collect(),
        }
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<Card> {
    //find matches in each line and collect all lines
    let cards: Vec<Card> = input
        .lines()
        .map(|line| {
            Card::new(line)
        })
        .collect();
    cards
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &Vec<Card>) -> u32 {

    input
        .iter()
        .map(|card| {
            process_card1(&card)
        })
        .sum()
}

fn process_card1(card: &Card) -> u32 {
    let winners: Vec<&u32> = card.my_numbers.iter().filter(|n|{
        card.winning_numbers.contains(n)
    }).collect();

    let mut score = 0;
    winners.iter().for_each(|_|{
        if score == 0{
            score = 1;
        }else{
            score = score*2;
        }
    });
    score
}

#[cfg(test)]
mod tests{
    use std::vec;

    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",Card{ 
        id: 1, 
        winning_numbers: vec![41,48,83,86,17], 
        my_numbers: vec![83,86,6,31,17,9,48,53]
    })]
    fn test_card(#[case] input:&str,#[case]result:Card){
        let card = Card::new(input);
        assert_eq!(card,result);
    }
}
