pub mod card;
pub mod read;

//use std::cmp::Ordering;
//use std::cmp::PartialOrd;
//use std::io;

#[derive(Debug, Clone)]
pub struct Player {
    pub name : String,
    pub cards : Vec<card::Card>,
    pub turn : bool,
}

impl Player {
    pub fn auto_sort(&mut self) {
        self.cards.partial_sort();
    }
    
    pub fn test_guess(&mut self, card_ind: usize, card_rank: u8) -> bool {
        if self.cards[card_ind].rank == card_rank {
            println!("Correct guess! The card is revealed.");
            self.cards[card_ind].visible = true;
            return true
        } else {
            println!("Wrong guess! The card is still hidden.");
            return false
        }
    }

    /* fn guess(&self) -> (usize, usize, u8) {
        let mut valid = false;
        let mut pi: usize;
        let mut ci: usize;
        let mut cr: u8;

        let mut p_ind = 
    
    /*
    /*fn play(&self, other: &mut Self, card_ind: usize, card_rank: u8) -> bool;
    /*fn reveal(&mut self);
    */
}

pub trait PartialSort {
    fn partial_sort(&mut self);
}

impl<T: Clone + Ord + PartialOrd> PartialSort for Vec<T> {
    fn partial_sort(&mut self) {
        let l = self.len();
        let mut sortable = (&mut **self).to_vec();
        let mut non_sortable_ind: Vec<usize> = Vec::new();
        for i in (0..l as usize).rev() {
            let non_sortable = sortable[i] != sortable[i];
            if non_sortable {
                non_sortable_ind.push(i);
                sortable.remove(i);
            }
        }
        sortable.sort();
        let mut j = 0;
        for i in 0..l as usize {
            if !non_sortable_ind.contains(&i) {
                self[i] = sortable[j].clone();
                j = j + 1;
            }
        }

    }
}
