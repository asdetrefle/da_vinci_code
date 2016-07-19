pub mod card;
pub mod read;

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

    pub fn guess(&self) -> (usize, usize, u8) {
        let mut valid = false;
        let mut pi: usize = 0;
        let mut ci: usize = 0;
        let mut cr: u8 = 0;

        while !valid {
            println!("Please select a player you want to guess : ");
            let buffer = read::read_int();
            match buffer {
                Ok(val) => {
                    pi = val as usize - 1;
                    valid = true;
                },
                Err(_) => valid = false
            }
        }

        valid = false;
        while !valid {
            println!("Please select a card you want to guess : ");
            let buffer = read::read_int();
            match buffer {
                Ok(val) => {
                    ci = val as usize - 1;
                    valid = true;
                },
                Err(_) => valid = false
            }
        }

        valid = false;
        while !valid {
            println!("Please guess the rank of the selected card : ");
            let buffer = read::read_int();
            match buffer {
                Ok(val) => {
                    cr = val;
                    valid = true;
                },
                Err(_) => valid = false
            }
        }

        return (pi, ci, cr)
    }

    //fn play(&self, other: &mut Self, card_ind: usize, card_rank: u8) -> bool;
    pub fn reveal(&mut self, i: usize) {
        self.cards[i].reveal() 
    }

    pub fn is_dead(&self) -> bool {
        let hidden_cards = self.cards.clone().into_iter().filter(|ref mut x| !x.visible).collect::<Vec<_>>();
        hidden_cards.is_empty()
    }
        
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
