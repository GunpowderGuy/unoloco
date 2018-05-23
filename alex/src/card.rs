#[derive(Copy, Clone, PartialEq)]
pub enum Suit {
    hearts,
    spades,
    diamonds,
    clubs,
}

#[derive(Copy, Clone, new, Getters, PartialEq)]
pub struct Card {
    tipo: Suit,
    rank: usize,
}

impl Card {
    fn getRank(&self) -> usize {
        return self.rank;
    }
    fn getSuit(&self) -> Suit {
        return self.tipo;
    }
}

//rankstring() devuelve el texto (string) del rango de la carta
//suitString() convierte suit a un string
// toString() devuelve rankString() + suitString()

//use std::ops::Add;
//impl PartialEq for Card {
//type Output = bool;
//fn PartialEq(&self,dos :Card) -> bool {
//if dos.tipo == self tipo
//}
//}en vez de implementar == para card lo voy a derivar
