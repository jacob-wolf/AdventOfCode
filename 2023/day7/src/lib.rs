pub mod p1 {
    use std::{
        cmp::Ordering,
        collections::{HashMap, HashSet},
    };
    #[derive(Debug, Clone)]
    pub enum HandType {
        FiveKind,
        FourKind,
        FullHouse,
        ThreeKind,
        TwoPair,
        OnePair,
        HighCard,
    }
    #[derive(Debug, Clone)]
    pub struct Hand {
        score: usize,
        pub bid: usize,
        //sorted_cards: Vec<usize>,
        unsorted_cards: Vec<usize>,
        hand_type: HandType,
    }

    impl Hand {
        fn set_score_from_hand_type(&mut self) {
            let hand_type = self.hand_type.clone();

            self.score = match hand_type {
                HandType::HighCard => 1,
                HandType::OnePair => 2,
                HandType::TwoPair => 3,
                HandType::ThreeKind => 4,
                HandType::FullHouse => 5,
                HandType::FourKind => 6,
                HandType::FiveKind => 7,
            };
        }

        pub fn parse_hand(cards: &str, bid: &usize) -> Self {
            let card_map = [
                '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
            ]
            .iter()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect::<HashMap<&char, usize>>();

            let unsorted_cards: Vec<usize> = cards
                .chars()
                .map(|char| card_map.get(&char).unwrap())
                .cloned()
                .collect::<Vec<usize>>();
            let hand_set: HashSet<usize> = HashSet::from_iter(unsorted_cards.clone());
            let hand_type: HandType = match hand_set.len() {
                1 => HandType::FiveKind,
                2 => {
                    // Full house or four of a kind
                    //count 1,4 or 2,3
                    if hand_set
                        .iter()
                        .map(|known_card| {
                            unsorted_cards
                                .clone()
                                .iter()
                                .filter(|card| { *card }.eq(known_card))
                                .collect::<Vec<&usize>>()
                                .len()
                        })
                        .any(|a| a == 1 || a == 4)
                    {
                        HandType::FourKind
                    } else {
                        HandType::FullHouse
                    }
                }
                3 => {
                    if hand_set
                        .iter()
                        .map(|known_card| {
                            unsorted_cards
                                .clone()
                                .iter()
                                .filter(|card| { *card }.eq(known_card))
                                .collect::<Vec<&usize>>()
                                .len()
                        })
                        .any(|a| a == 3)
                    {
                        HandType::ThreeKind
                    } else {
                        HandType::TwoPair
                    }
                }
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                _ => HandType::HighCard,
            };

            let mut new_hand: Hand = Hand {
                score: 0,
                bid: bid.clone(),
                unsorted_cards,
                hand_type,
            };
            new_hand.set_score_from_hand_type();
            new_hand
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            let initial = self.score.cmp(&other.score);

            if let Ordering::Equal = initial {
                if self.unsorted_cards.eq(&other.unsorted_cards) {
                    return Ordering::Equal;
                } else {
                    let mut cmp_index = 0;
                    loop {
                        let comp =
                            self.unsorted_cards[cmp_index].cmp(&other.unsorted_cards[cmp_index]);
                        if let Ordering::Equal = comp {
                            cmp_index += 1;
                        } else {
                            return comp;
                        }
                    }
                }
            }
            initial
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.unsorted_cards.eq(&other.unsorted_cards)
        }
    }

    impl Eq for Hand {}
}

pub mod p2 {
    //'J' now is the lowest card, score is computed as a "Joke score" where jokers match highest available card in hand
    use std::{
        cmp::Ordering,
        collections::{HashMap, HashSet},
    };
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum HandType {
        FiveKind,
        FourKind,
        FullHouse,
        ThreeKind,
        TwoPair,
        OnePair,
        HighCard,
    }
    #[derive(Debug, Clone)]
    pub struct Hand {
        score: usize,
        pub bid: usize,
        unsorted_cards: Vec<usize>,
        pub hand_type: HandType,
    }

    fn determine_joker_replacement(unsorted_cards: &Vec<usize>) -> usize {
        let mut card_freqs = unsorted_cards
            .iter()
            .cloned()
            .map(|card| {
                let freq = unsorted_cards
                    .iter()
                    .filter(|c| { *c }.eq(&card))
                    .collect::<Vec<&usize>>()
                    .len();
                (freq, card)
            })
            .collect::<Vec<(usize, usize)>>();
        // sort the cards by frequency then by card value
        card_freqs.sort_by(|(f_a, c_a), (f_b, c_b)| {
            if let Ordering::Equal = f_a.cmp(f_b) {
                return c_a.cmp(c_b);
            }
            f_a.cmp(f_b)
        });

        for index in (0_usize..card_freqs.len()).rev() {
            if card_freqs[index].1 != 0 {
                return card_freqs[index].1;
            }
        }

        card_freqs[0].1
    }

    impl Hand {
        fn set_score_from_hand_type(&mut self) {
            let hand_type = self.hand_type.clone();

            self.score = match hand_type {
                HandType::HighCard => 1,
                HandType::OnePair => 2,
                HandType::TwoPair => 3,
                HandType::ThreeKind => 4,
                HandType::FullHouse => 5,
                HandType::FourKind => 6,
                HandType::FiveKind => 7,
            };
        }

        pub fn parse_hand(cards: &str, bid: &usize) -> Self {
            let card_map = [
                'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
            ]
            .iter()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect::<HashMap<&char, usize>>();

            let unsorted_cards: Vec<usize> = cards
                .chars()
                .map(|char| card_map.get(&char).unwrap())
                .cloned()
                .collect::<Vec<usize>>();

            let joker_replacement = determine_joker_replacement(&unsorted_cards);

            let joke_hand_cards = unsorted_cards
                .iter()
                .cloned()
                .map(|card| {
                    if card.eq(&0) {
                        //joker is 0 replace with high card for hand_type/score calculation
                        return joker_replacement.clone();
                    }
                    card
                })
                .collect::<Vec<usize>>();

            let joke_hand_set: HashSet<usize> = HashSet::from_iter(joke_hand_cards.clone());
            let hand_type: HandType = match joke_hand_set.len() {
                1 => HandType::FiveKind,
                2 => {
                    // Full house or four of a kind
                    //count 1,4 or 2,3
                    if joke_hand_set
                        .iter()
                        .map(|known_card| {
                            unsorted_cards
                                .clone()
                                .iter()
                                .filter(|card| { *card }.eq(known_card))
                                .collect::<Vec<&usize>>()
                                .len()
                        })
                        .any(|a| a == 1 || a == 4)
                    {
                        HandType::FourKind
                    } else {
                        HandType::FullHouse
                    }
                }
                3 => {
                    if joke_hand_set
                        .iter()
                        .map(|known_card| {
                            joke_hand_cards
                                .clone()
                                .iter()
                                .filter(|card| { *card }.eq(known_card))
                                .collect::<Vec<&usize>>()
                                .len()
                        })
                        .any(|a| a == 3)
                    {
                        HandType::ThreeKind
                    } else {
                        HandType::TwoPair
                    }
                }
                4 => HandType::OnePair,
                5 => HandType::HighCard,
                _ => HandType::HighCard,
            };

            let mut new_hand: Hand = Hand {
                score: 0,
                bid: bid.clone(),
                unsorted_cards,
                //sorted_cards: sorted_cards.clone(),
                hand_type,
            };
            new_hand.set_score_from_hand_type();
            new_hand
        }
    }

    impl Ord for Hand {
        fn cmp(&self, other: &Self) -> Ordering {
            let initial = self.score.cmp(&other.score);

            if let Ordering::Equal = initial {
                if self.unsorted_cards.eq(&other.unsorted_cards) {
                    return Ordering::Equal;
                } else {
                    let mut cmp_index = 0;
                    loop {
                        let comp =
                            self.unsorted_cards[cmp_index].cmp(&other.unsorted_cards[cmp_index]);
                        if let Ordering::Equal = comp {
                            cmp_index += 1;
                        } else {
                            return comp;
                        }
                    }
                }
            }
            initial
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Hand {
        fn eq(&self, other: &Self) -> bool {
            self.unsorted_cards.eq(&other.unsorted_cards)
        }
    }

    impl Eq for Hand {}
}
