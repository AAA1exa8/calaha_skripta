use lru::LruCache;
use std::num::NonZeroUsize;
use std::fmt::Debug;
use std::cmp::{max, min};
use colored::*;

fn main() {
    let mut kalah = Kalah::new();
    let games = kalah.get_children();
    let mut cache = LruCache::new(NonZeroUsize::new(500_000_000).unwrap());
    let (score, best_move) = minimax(&kalah, 10, i32::MIN, i32::MAX, true, &mut cache);
    println!("Best score: {}, Best move: {}", score, best_move);
    kalah = games[best_move].clone();
    loop {
        // print!("Enter index: ");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let index = input.trim().parse::<usize>().unwrap();
        let last_index = kalah.move_stones(index);
        println!("{:#?}", kalah);
        if (last_index + 1) % 7 != 0 {
            println!("computing");
            let games = kalah.get_children();
            let (score, best_move) = minimax(&kalah, 10, i32::MIN, i32::MAX, true, &mut cache);
            println!("score: {}, best_move:\n {:#?}", score, games[best_move]);
            kalah = games[best_move].clone();
        }                    
    }
}



#[derive(Clone, PartialEq, Eq, Hash)]
struct Kalah {
    players_turn: Turn,
    game: [u8; 14],
}

#[derive(Clone, PartialEq, Eq, Debug, Copy, Hash)]
enum Turn {
    Player1,
    Player2,
}

impl std::fmt::Debug for Kalah {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "players_turn: {:?}\n", self.players_turn)?;
        write!(f, "game:\n")?;
        for i in (7..14).rev() {
            write!(f, "{:0x} ", i)?;
        }
        write!(f, "\n")?;
        for i in (7..14).rev() {
            write!(f, "{} ", format!("{}",self.game[i]).red())?;
        }
        write!(f, "\n  ")?;
        for i in 0..7 {
            write!(f, "{} ", format!("{}",self.game[i]).green())?;
        }
        write!(f, "\n  ")?;
        for i in 0..7 {
            write!(f, "{} ", i)?;
        }
        write!(f, "\n")
    }
}

impl Kalah {
    fn new() -> Kalah {
        Kalah {
            players_turn: Turn::Player1,
            game: [6,6,6,6,6,6,0,6,6,6,6,6,6,0]
        }
    }

    fn heuristic(&self) -> i32 {
        match self.players_turn {
            Turn::Player1 => {
                let (player_1, player_2) = self.game.split_at(7);
                let player_1_score = player_1[6]*2 + player_1.iter().sum::<u8>();
                let player_2_score = player_2[6]*2 + player_2.iter().sum::<u8>();
                (player_1_score as i32) - (player_2_score as i32)
            },
            Turn::Player2 => {
                let (player_1, player_2) = self.game.split_at(7);
                let player_1_score = player_1[6]*2 + player_1.iter().sum::<u8>();
                let player_2_score = player_2[6]*2 + player_2.iter().sum::<u8>();
                (player_2_score as i32) - (player_1_score as i32)
            }
        }
    }

    fn get_children(&self) -> Vec<Kalah> {
        let mut childeren = Vec::new();
        match self.players_turn {
            Turn::Player1 => {
                for i in 0..6 {
                    if self.game[i] != 0 {
                        let mut child = self.clone();
                        let last_index = child.move_stones(i);
                        if (last_index + 1) % 7 == 0 {
                            childeren.append(&mut child.get_children());
                        }else{
                            childeren.push(child);
                        }
                    }
                }
            },
            Turn::Player2 => {
                for i in 7..13 {
                    if self.game[i] != 0 {
                        let mut child = self.clone();
                        let last_index = child.move_stones(i);
                        if (last_index + 1) % 7 == 0 {
                            childeren.append(&mut child.get_children());
                        }else{
                            childeren.push(child);
                        }
                    }
                }
            }
        };
        childeren
    }


    fn move_stones(&mut self, index: usize) -> usize {
        let mut stones = self.game[index];
        self.game[index] = 0;
        let mut current_index = index;
        while stones > 0 {
            current_index = (current_index + 1) % 14;
            if current_index == match self.players_turn {
                Turn::Player1 => 13,
                Turn::Player2 => 6,
            } {
                continue;
            }
            self.game[current_index] += 1;
            stones -= 1;
        }
        if self.game[current_index] == 1 && match self.players_turn {
            Turn::Player1 => {
                current_index < 6
            },
            Turn::Player2 => {
                current_index > 6 && current_index < 13
            }
        } {
            let opposite_index = 12 - current_index;
            self.game[current_index] = 0;
            self.game[match self.players_turn {
                Turn::Player1 => 6,
                Turn::Player2 => 13,
            }] += self.game[opposite_index] + 1;
            self.game[opposite_index] = 0;
        }
        if (current_index + 1) % 7 != 0 {
            self.players_turn = match self.players_turn {
                Turn::Player1 => Turn::Player2,
                Turn::Player2 => Turn::Player1,
            }
        }
        current_index
    }

    fn game_over(&self) -> bool {
        let (player_1, player_2) = self.game.split_at(7);
        player_1.iter().take(6).sum::<u8>() == 0 || player_2.iter().take(6).sum::<u8>() == 0
    }
}

fn minimax(node: &Kalah, depth: u64, alpha: i32, beta: i32, maximizing_player: bool, cache: &mut LruCache<Kalah, (i32, usize)>) -> (i32, usize) {
    if let Some(&(score, move_)) = cache.get(node) {
        return (score, move_);
    }

    if depth == 0 || node.game_over() {
        let result = (node.heuristic(), 0);
        cache.put(node.clone(), result);
        return result;
    }

    if maximizing_player {
        let mut max_eval = i32::MIN;
        let mut best_move = 0;
        let mut alpha = alpha;
        for (i, child) in node.get_children().iter().enumerate() {
            let (eval, _) = minimax(child, depth - 1, alpha, beta, false, cache);
            if eval > max_eval {
                max_eval = eval;
                best_move = i;
            }
            alpha = max(alpha, eval);
            if beta <= alpha {
                break;
            }
        }
        let result = (max_eval, best_move);
        cache.put(node.clone(), result);
        return result;
    } else {
        let mut min_eval = i32::MAX;
        let mut best_move = 0;
        let mut beta = beta;
        for (i, child) in node.get_children().iter().enumerate() {
            let (eval, _) = minimax(child, depth - 1, alpha, beta, true, cache);
            if eval < min_eval {
                min_eval = eval;
                best_move = i;
            }
            beta = min(beta, eval);
            if beta <= alpha {
                break;
            }
        }
        let result = (min_eval, best_move);
        cache.put(node.clone(), result);
        return result;
    }
}