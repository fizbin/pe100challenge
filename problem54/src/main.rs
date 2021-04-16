use std::cmp::Ordering;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;
use std::path::Path;
use std::str::FromStr;
use HandType::*;

#[derive(PartialEq, Eq, Clone, Copy, Default, Debug)]
pub struct Card([char; 2]);

const RANK_ORDER: &str = "23456789TJQKA";

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub enum HandType {
    HighCard([Card; 1]),
    OnePair([Card; 2]),
    TwoPair([Card; 4]),
    ThreeKind([Card; 3]),
    Straight([Card; 5]),
    Flush([Card; 5]),
    FullHouse([Card; 5]),
    FourKind([Card; 4]),
    StraightFlush([Card; 5]),
    RoyalFlush([Card; 5]),
}

impl PartialOrd for Card {
    // Partial order by rank only
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let rank0 = self.0[0];
        let rank1 = other.0[0];
        if rank0 == rank1 {
            if self.0[1] == other.0[1] {
                return Some(Ordering::Equal);
            } else {
                return None;
            }
        } else {
            let pos0 = RANK_ORDER.find(rank0);
            let pos1 = RANK_ORDER.find(rank1);
            return Some(pos0.cmp(&pos1));
        }
    }
}

impl Ord for Card {
    // full order by rank and then - if same rank - alphabetically: C, D, H, S
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other)
            .unwrap_or_else(|| self.0.cmp(&other.0))
    }
}

impl Card {
    fn suit(&self) -> char {
        self.0[1]
    }
    fn rank(&self) -> char {
        self.0[0]
    }
    fn rank_int(&self) -> usize {
        RANK_ORDER.find(self.0[0]).map(|x| x + 2).unwrap_or(0)
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ParseCardError {
    WrongNumberOfChars,
    BadRank,
    BadSuit,
}

impl FromStr for Card {
    type Err = ParseCardError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s2 = s.trim();
        let mut chars = s2.chars();
        match (chars.next(), chars.next(), chars.next()) {
            (Some(rank), Some(suit), None) => {
                if !RANK_ORDER.contains(rank) {
                    Err(ParseCardError::BadRank)
                } else if !"HCDS".contains(suit) {
                    Err(ParseCardError::BadSuit)
                } else {
                    Ok(Card([rank, suit]))
                }
            }
            _ => Err(ParseCardError::WrongNumberOfChars),
        }
    }
}

// Assumes that hand is sorted highest rank to lowest rank
fn score_hand(hand: &[Card; 5]) -> HandType {
    let mut best_so_far = [HighCard([hand.iter().max().unwrap().clone()])];
    let mut has_score = move |score| {
        //if best_so_far.le(&score) {
        //    best_so_far = score;
        //}
        best_so_far[0] = score;
    };
    let first_suit = hand[0].suit();
    let first_rank = hand[0].rank_int();
    if (hand[1].rank_int() == first_rank - 1)
        && (hand[2].rank_int() == first_rank - 2)
        && (hand[3].rank_int() == first_rank - 3)
        && (hand[4].rank_int() == first_rank - 4)
    {
        has_score(Straight(hand.clone()));
    }
    if hand.iter().all(|c| c.suit() == first_suit) {
        // some kind of flush
        match best_so_far {
            [Straight(x)] => {
                if hand[0].rank() == 'A' {
                    return RoyalFlush(x);
                } else {
                    return StraightFlush(x);
                }
            }
            _ => has_score(Flush(hand.clone())),
        }
    }
    let ranks: Vec<usize> = hand.iter().map(|x| x.rank_int()).collect();
    let rank_set = HashSet::<_>::from_iter(ranks.iter());
    match rank_set.len() {
        2 =>
        // either four-of-a-kind or full house
        {
            if ranks[1..4].iter().all(|x| *x == ranks[0]) {
                let mut four_cards: [Card; 4] = Default::default();
                four_cards.copy_from_slice(&hand[0..4]);
                has_score(FourKind(four_cards))
            } else if ranks[1..4].iter().all(|x| *x == ranks[4]) {
                let mut four_cards: [Card; 4] = Default::default();
                four_cards.copy_from_slice(&hand[1..5]);
                has_score(FourKind(four_cards))
            } else if (ranks[0] == ranks[1]) && (ranks[1] == ranks[2]) && (ranks[3] == ranks[4]) {
                has_score(FullHouse(hand.clone()));
            } else if (ranks[0] == ranks[1]) && (ranks[2] == ranks[3]) && (ranks[3] == ranks[4]) {
                has_score(FullHouse(hand.clone()));
            } else {
                panic!("Couldn't score hand {:?}", &hand);
            }
        }
        3 =>
        // two pair or three-of-a-kind
        {
            if ranks[0] == ranks[1] {
                if ranks[1] == ranks[2] {
                    has_score(ThreeKind([hand[0], hand[1], hand[2]]))
                } else if ranks[2] == ranks[3] {
                    has_score(TwoPair([hand[0], hand[1], hand[2], hand[3]]));
                } else if ranks[3] == ranks[4] {
                    has_score(TwoPair([hand[0], hand[1], hand[3], hand[4]]));
                } else {
                    panic!("Couldn't score hand {:?}", &hand);
                }
            } else if ranks[1] == ranks[2] {
                if ranks[2] == ranks[3] {
                    has_score(ThreeKind([hand[1], hand[2], hand[3]]))
                } else if ranks[3] == ranks[4] {
                    has_score(TwoPair([hand[1], hand[2], hand[3], hand[4]]));
                } else {
                    panic!("Couldn't score hand {:?}", &hand);
                }
            } else if (ranks[2] == ranks[3]) && (ranks[3] == ranks[4]) {
                has_score(ThreeKind([hand[2], hand[3], hand[4]]))
            } else {
                panic!("Couldn't score hand {:?}", &hand);
            }
        }
        4 =>
        // one pair
        {
            if ranks[0] == ranks[1] {
                has_score(OnePair([hand[0], hand[1]]));
            } else if ranks[1] == ranks[2] {
                has_score(OnePair([hand[1], hand[2]]));
            } else if ranks[2] == ranks[3] {
                has_score(OnePair([hand[2], hand[3]]));
            } else if ranks[3] == ranks[4] {
                has_score(OnePair([hand[3], hand[4]]));
            } else {
                panic!("Couldn't score hand {:?}", &hand);
            }
        }
        5 =>
        // High Card; already handled
        {
            ()
        }
        x => {
            // we don't allow jokers in this impl, so...
            panic!("Impossible rank count {} for hand {:?}", x, &hand);
        }
    }
    best_so_far[0]
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    let mut p1win = 0;
    match read_lines("./p054_poker.txt") {
        Err(err) => panic!("{}", err),
        Ok(lines) => {
            for line in lines {
                match line {
                    Err(err) => panic!("{}", err),
                    Ok(line2) => {
                        let cards_parse: Result<Vec<Card>, _> =
                            line2.split(' ').map(|x| x.parse()).collect();
                        match cards_parse {
                            Err(err) => panic!("{:?}", err),
                            Ok(cards) => {
                                let mut p1: [Card; 5] = Default::default();
                                let mut p2: [Card; 5] = Default::default();
                                p1.copy_from_slice(&cards[0..=4]);
                                p2.copy_from_slice(&cards[5..=9]);
                                p1.sort();
                                p1.reverse();
                                p2.sort();
                                p2.reverse();
                                let p1score = score_hand(&p1);
                                let p2score = score_hand(&p2);
                                let oldp1win = p1win;
                                match p1score.partial_cmp(&p2score) {
                                    None | Some(Ordering::Equal) => {
                                        if p1 > p2 {
                                            p1win += 1;
                                        }
                                    }
                                    Some(Ordering::Greater) => p1win += 1,
                                    _ => (),
                                }
                                if p1win == oldp1win {
                                    println!("{} -> P2 {:?} vs {:?}", line2, p1score, p2score);
                                } else {
                                    println!("{} -> P1 {:?} vs {:?}", line2, p1score, p2score);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", p1win);
}
