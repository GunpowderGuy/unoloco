use card;

#[derive(Clone, new, Getters)]
pub struct Hand {
    // hand o jugador
    name: String,
    hand: Vec<card::Card>,
}

impl Hand {
    fn getHandSize(&self) -> usize {
        return self.hand.len();
    }
    fn addCard(&mut self, input: card::Card) {
        self.hand.push(input);
    }
    fn showHand(&self) {
        for i in 0..self.hand.len() {
            self.hand[i].suitString();
        }
    }
    fn hasCardWithRank(&self, inputRank: usize) -> bool {
        let mut siOno = false;
        for i in 0..self.hand.len() {
            if self.hand[i].rank == inputRank {
                siOno = true;
            }
        }
        return siOno;
    }
    fn hasCardWithSuit(&self, inputEnum: card::Suit) -> bool {
        let mut siOno = false;
        for i in 0..self.hand.len() {
            if self.hand[i].tipo == inputEnum {
                siOno = true;
            }
        }
        return siOno;
    }
    fn removeCardFromHand(&mut self, targetCard: usize) {
        // el juagodr juega una carta
        self.hand.remove(targetCard);
    }
}

pub fn generate() -> Hand {
    //    let retorno = Hand::new("self", card::Card { card::Card.generateCard()});
    let mut mistack = Vec::new();
    for i in 0..8 {
        let card = card::generate();
        mistack.push(card);
    }
    return Hand {
        name: "slef".to_string(),
        hand: mistack,
    };
}
