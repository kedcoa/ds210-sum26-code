use std::f32::MIN;

use tic_tac_toe_stencil::agents::Agent;
use tic_tac_toe_stencil::board::Board;
use tic_tac_toe_stencil::player::Player;

// Your solution solution.
pub struct SolutionAgent {}

// Put your solution here.
impl Agent for SolutionAgent {
    // Should returns (<score>, <x>, <y>)
    // where <score> is your estimate for the score of the game
    // and <x>, <y> are the position of the move your solution will make.
    //player = whoever's turn it is currently 
    fn solve(board: &mut Board, player: Player, _time_limit: u64) -> (i32, usize, usize) {
        //1. check if game is over: 
        if board.game_over() {
            return (board.score(),0,0);
        }
        
        //2. if not over, get all possible moves 
        let moves = board.moves();

        //3. Min Max method > x is maximizer and o is minimizer (Score)
        let mut best_score = match player { //matches player's field of being x or o 
            Player::X => i32::MIN, //if player is x, you want them to get the lowest score
            Player::O => i32::MAX,  //if player is o, you want them to get the highest score 
        };

        let mut best_move = moves[0]; 
        //have to assign this because worst possible scenario 
       
        //saying that now x's turn becomes o's turn and vice versa
        let next_player = match player {
            Player::X => Player::O,
            Player::O => Player::X, 
        }; 
        
        for m in moves {
            board.apply_move(m, player); //simulates your first move by actually changing the board
            let (score, _, _) = SolutionAgent::solve(board, next_player, _time_limit); 
            //recursive function that calls the entire function again but for next_player to see their score 
            board.undo_move(m, player); //changes board back to what it was 
            
            let better_move = match player {
                Player::X => score > best_score, 
                //if you're x, and score from your simluated move is 
                //greater than best_score, then it's the better move
                Player::O => score < best_score,
            }; 
            if better_move { //if better_move is true, then record as the current best move
                best_score = score; 
                best_move = m;

            }
        }
        (best_score, best_move.0, best_move.1)
    }
}
