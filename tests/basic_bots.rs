extern crate pleco;


use pleco::engine::Searcher;
use pleco::bot_prelude::*;
use pleco::tools::gen_rand_legal_board;



#[test]
fn test_all_bot() {
    for _x in 0..5 {
        let board = gen_rand_legal_board();
        RandomBot::best_move_depth(board.shallow_clone(), 4);
        SimpleBot::best_move_depth(board.shallow_clone(), 4);
        AlphaBetaBot::best_move_depth(board.shallow_clone(), 4);
        ParallelSearcher::best_move_depth(board.shallow_clone(), 4);
        JamboreeSearcher::best_move_depth(board.shallow_clone(), 4);
    }
}