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
}

// deck() crear e inicializar una serie de cartas
// dealcard() remover una carta para darsela a un jugador
