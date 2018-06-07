use card;

const size: usize = 52;

#[derive(Copy, Clone, Getters)]
pub struct Deck {
    cards: [card::Card; size],
    index: usize, // numero de cartas que han sido repartidas
}

impl Deck {
    fn size(&self) -> usize {
        return size - self.index;
    }
    fn deck(&mut self) {
        for i in 0..(size) {
            self.cards[i].rank = 1;
            self.cards[i].tipo = card::Suit::diamonds;
        }
    }
    fn dealcard(&mut self, numeroCarta: usize) {
        let backupCards = self.cards; //: [card::Card; size];
        let mut offset = 0;
        for i in 0..(size - self.index) {
            if i != numeroCarta {
                self.cards[i] = backupCards[i + offset]
            } else if i != numeroCarta {
                offset = 1
            }
            backupCards[numeroCarta];
        }
    }
}
