#![allow(dead_code)]

extern crate rand;

mod player;

// use std::io;
use rand::{thread_rng, Rng};

use player::card;
use player::read;

static COLORS: u8 = 2;
static RANKS : u8 = 14;
static TOTAL_CARDS : u8 = 28;

#[derive(Debug, Clone)]
struct Game {
    n_players: u8,
    level: usize,
    players: Vec<player::Player>,
    round: usize,
    turn: usize,
}

impl Game {

    fn new() -> Game {
        Game {
            n_players: 0,
            level: 0,
            players: Vec::new(),
            round: 0,
            turn: 0,
        }
    }
    
    fn initiate(&mut self) {
        // initiate the number of players
        println!("Please enter the number of players :");
        let mut _np = read::read_int();
        match _np {
            Ok(val) => {
                if val>=2 && val <=4 {
                    self.n_players = val;
                } else {
                    panic!("Please enter a valid integer!");
                }
            },
            Err(_) => panic!("Please enter a valid integer!"),
        }
        
        // initiate the names of players
        let mut names: Vec<String> = Vec::new(); 
        for i in 0..self.n_players as usize {
            println!("Please enter the name of player {} :", i+1);
            let name = read::read_buffer();
            match name {
                Ok(val) => names.push(val),
                Err(_) => panic!("Please enter a valid name!"),
            }
        }
        
        // initiate Players
        let mut c = generate_cards(self.n_players, self.level);
        let n_c = (TOTAL_CARDS / self.n_players) as usize;
        for i in 0..self.n_players as usize {
            let mut _cards = (&mut c[n_c*i..(n_c*i + n_c)]).to_vec();
            let mut _player = player::Player {
                name: names[i].clone(),
                cards: _cards,
                turn: false,
            };
            _player.auto_sort();
            self.players.push(_player);
        } 
        
        // initiate the pointer to the next player.
        let mut rng = thread_rng();
        self.turn = rng.gen_range(0, self.n_players) as usize;

    }

    fn play(&mut self) {
        self.initiate();
        // self.show()

        // to change to while self.is_on() {
        while true {
            if self.players[self.turn].is_dead() {
                println!("*** Player {} is already dead! ***", self.players[self.turn].name);
            } else {
                println!("*** Player {} to play ***", self.players[self.turn].name);
                self.round();
                while self.players[self.turn].turn {
                    println!("Do you want to continue to guess? (Y/N)");
                    let buffer = read::read_buffer();
                    if buffer.unwrap() == "Y" {
                        self.round();
                    } else {
                        self.players[self.turn].turn = false;
                    }
                }
            }
            self.next();
        }
    }
    
    // leave turn to next player 
    fn next(&mut self) {
        println!("-------------------------------------------------------");
        self.turn = (self.turn + 1) % (self.n_players as usize);
    }

    fn round(&mut self) {
        self.players[self.turn].turn = true;
        let (pi, ci, cr) = self.players[self.turn].guess();
        let is_correct = self.players[pi].test_guess(ci, cr);
        if !is_correct {
            println!("You need to show one of your cards! Please Select:");
            let buffer = read::read_int();
            match buffer {
                Ok(val) => self.players[self.turn].reveal(val as usize - 1),
                Err(_) => panic!("Please enter a valid integer number!"),
            };
            self.players[self.turn].turn = false;
            println!("Your turn is finished!");
        }
    }

    fn is_on(&self) {
        unimplemented!();
    }
}

fn generate_cards(number: u8, level: usize) -> Vec<card::Card> {
    let mut cards: Vec<card::Card> = Vec::new();
    for r in 0..RANKS {
        for c in 0..COLORS {
            cards.push(card::Card { rank : r, color : c, visible : false })
        }
    }

    let mut rng = thread_rng();
    rng.shuffle(&mut cards);
    return cards
}

fn main() {
    println!("Hello, Da Vinci!");
    let x = vec![1,2,4];
    let y = x.clone().into_iter().filter(|&i| i>5).collect::<Vec<_>>();
    println!("{:?}", y);
    println!("{:?}", x);
    let mut g: Game = Game::new();
    g.play();
}
