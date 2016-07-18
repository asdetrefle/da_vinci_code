use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt;

pub static SUITS: &'static [&'static str] = &["♠", "♥", "♦", "♣"];

pub enum Color {
    Spade,    // ♠
    Heart,    // ♥
    Diamond,  // ♦
    Clubs,    // ♣
}

#[derive(Eq, Clone)]
pub struct Card {
    pub rank: u8,
    pub color: u8,
    pub visible: bool,
}

impl Card {
    fn reveal(&mut self) {
        self.visible = true
    }

    fn hide(&mut self) {
        self.visible = false
    }

    fn revert(&mut self) {
        self.visible = !self.visible
    }
}

impl Ord for Card {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        assert!(self.rank != 0 && other.rank != 0, "Joker has no order");
        if self.rank == other.rank {
            self.color.cmp(&other.color)
        } else {
            self.rank.cmp(&other.rank)
        }
    }
}

impl PartialOrd for Card {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.rank == 0 || other.rank == 0 {
            None
        } else if self.rank == other.rank {
            Some(self.color.cmp(&other.color))
        } else {
            Some(self.rank.cmp(&other.rank))
        }
    }
}

impl PartialEq for Card {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.rank == 0 || other.rank == 0 {
            false
        } else {
            (self.rank, self.color).eq(&(other.rank, other.color))
        }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "
            \n ------- \
            \n | {c:>width$}  | \
            \n |     | \
            \n | {r:>width$}  | \
            \n -------
            ", r = self.rank, c = SUITS[self.color as usize], width = 2)
    }
}

