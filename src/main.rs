#![allow(dead_code)]

extern crate rand;

mod player;

use std::io;
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

    /*fn play(&mut self) {
    }
    */
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
    println!("Hello, world!");
    let mut g: Game = Game::new();
    g.initiate();
    println!("input test {:?}", g.players[1].guess());
    /*let mut c = String::new();
    let q = io::stdin().read_line(&mut c);
    let q = read::read_buffer().unwrap();
    println!("{}",q.unwrap());
    let qi = q.parse::<u8>().unwrap();
    println!("{:?}", c);
    assert_eq!(qi==4, false);
    let s1: &str = "test1 \n
                    test2 ";
    let s2: &str = "test3 \n
                    test4 ";
    let s = [s1, s2];
    println!("{}", s1);
    */
}
