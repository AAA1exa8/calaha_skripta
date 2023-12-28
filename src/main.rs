use std::collections::HashMap;

fn main() {
    let mut kalah = Kalah::new();
    let games = kalah.get_children();
    // let mut cache = HashMap::new(); 
    let (score, best_move) = minimax(&kalah, 10, true, /*&mut cache*/);
    println!("score: {}, best_move: {:#?}", score, games[best_move]);
}



#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Kalah {
    players_turn: Turn,
    game: [u8; 14],
}

#[derive(Clone, PartialEq, Eq, Debug, Copy, Hash)]
enum Turn {
    Player1,
    Player2,
}

impl Kalah {
    fn new() -> Kalah {
        Kalah {
            players_turn: Turn::Player1,
            game: [6,6,6,6,6,6,0,6,6,6,6,6,6,0]
        }
    }

    fn heuristic(&self) -> i32 {
        let (player_1, player_2) = self.game.split_at(7);
        let player_1_score = player_1[6] + player_1.iter().sum::<u8>();
        let player_2_score = player_2[6] + player_2.iter().sum::<u8>();
        (player_1_score as i32) - (player_2_score as i32)
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
        childeren.reverse();
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
        if (current_index + 1) % 7 != 0 {
            self.players_turn = match self.players_turn {
                Turn::Player1 => Turn::Player2,
                Turn::Player2 => Turn::Player1,
            }
        }
        if self.game[current_index] == 1 && current_index != 6 && current_index != 13 {
            let opposite_index = 12 - current_index;
            self.game[current_index] = 0;
            self.game[6] += self.game[opposite_index] + 1;
            self.game[opposite_index] = 0;
        }
        current_index
    }

    fn game_over(&self) -> bool {
        let (player_1, player_2) = self.game.split_at(7);
        player_1.iter().take(6).sum::<u8>() == 0 || player_2.iter().take(6).sum::<u8>() == 0
    }
}

fn minimax(node: &Kalah, depth: u64, maximizing_player: bool/*, cache: &mut HashMap<Kalah, (i32, usize)>*/) -> (i32, usize) {
    // if let Some(&(score, move_)) = cache.get(node) {
    //     return (score, move_);
    // }

    if depth == 0 || node.game_over() {
        let result = (node.heuristic(), 0);
        // cache.insert(node.clone(), result);
        return result;
    }

    if maximizing_player {
        let mut max_eval = i32::MIN;
        let mut best_move = 0;
        for (i, child) in node.get_children().iter().enumerate() {
            let (eval, _) = minimax(child, depth - 1, false, /*cache*/);
            if eval > max_eval {
                max_eval = eval;
                best_move = i;
            }
        }
        let result = (max_eval, best_move);
        // cache.insert(node.clone(), result);
        return result;
    } else {
        let mut min_eval = i32::MAX;
        let mut best_move = 0;
        for (i, child) in node.get_children().iter().enumerate() {
            let (eval, _) = minimax(child, depth - 1, true, /*cache*/);
            if eval < min_eval {
                min_eval = eval;
                best_move = i;
            }
        }
        let result = (min_eval, best_move);
        // cache.insert(node.clone(), result);
        return result;
    }
}