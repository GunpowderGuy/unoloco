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
}

//showHand() retornar string de todas las cartas poseidas
// hasCardWithRank() averiguar si tienes una carta con el mismo rango que el input
//hasCardWithSuit lo mismo que el metodo anterior , pero con suit
//removeCardFromHand el jugador juega una carta
