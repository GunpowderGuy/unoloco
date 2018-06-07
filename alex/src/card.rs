#[derive(Copy, Clone, PartialEq)]
pub enum Suit {
    hearts,
    spades,
    diamonds,
    clubs,
}

#[derive(Copy, Clone, new, Getters, PartialEq)]
pub struct Card {
    pub tipo: Suit,
    pub rank: usize,
}

impl Card {
    fn getRank(&self) -> usize {
        return self.rank;
    }
    fn getSuit(&self) -> Suit {
        return self.tipo;
    }
    fn rankString(&self) -> String {
        match self.rank {
            1 => return "one".to_owned(),
            2 => "two".to_owned(),
            3 => "three".to_owned(),
            4 => "four".to_owned(),
            5 => "five".to_owned(),
            6 => "six".to_owned(),
            7 => "seven".to_owned(),
            8 => "eight".to_owned(),
            9 => "nine".to_owned(),
            10 => "ten".to_owned(),
            11 => "eleven".to_owned(),
            12 => "twelve".to_owned(),
            _ => "twelve".to_owned(),
        }
    }
    pub fn suitString(&self) -> &str {
        match self.tipo {
            hearts => "hearts",     //.to_owned(),
            spades => "spades",     //.to_owned(),
            diamonds => "diamonds", //.to_owned(),
            clubs => "clubs",       //.to_owned(),
        }
    }
    fn toString(&self) -> String {
        return self.rankString() + self.suitString();
    }
}

//use std::ops::Add;
//impl PartialEq for Card {
//type Output = bool;
//fn PartialEq(&self,dos :Card) -> bool {
//if dos.tipo == self tipo
//}
//}en vez de implementar == para card lo voy a derivar
