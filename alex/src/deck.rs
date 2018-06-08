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
    fn dealcard(&mut self, numeroCarta: usize) -> card::Card {
        let backupCards = self.cards; //: [card::Card; size];
        let mut offset = 0;

        for i in 0..(size - self.index) {
            if i != numeroCarta {
                self.cards[i] = backupCards[i + offset]
            } else if i != numeroCarta {
                offset = 1
            }
        }
        self.index = self.index + 1; // incrementar el numeroCartas repartidas
        return backupCards[numeroCarta];
    }
}

pub fn generate() -> Deck {
    //    let retorno = Hand::new("self", card::Card { card::Card.generateCard()});
    let mut mistack: [card::Card; size] = [card::generate(); size];

    return Deck {
        index: 0,
        cards: mistack,
    };
}
