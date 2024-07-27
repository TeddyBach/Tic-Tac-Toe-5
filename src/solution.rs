use crate::board::{Board, Cell};
use crate::player::Player;
use crate::agents::Agent;

// Your solution agent.

fn heuristic(board: &mut Board) -> i32{
  let mut temp_score: i32 = 0;
  for i in 0..board.get_cells().len() {
    for j in 0..board.get_cells().len() {
      // Count row.
      if j + 2 < board.get_cells().len() {
        let x = &board.get_cells()[i][j];
        let y = &board.get_cells()[i][j+1];
        let z = &board.get_cells()[i][j+2];
        
        if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
          temp_score += 3;
        } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
          temp_score -= 3;
        }

        else if let (Cell::X, Cell::X, Cell::Empty) = (x, y, z) {
          temp_score += 2;
        }
        else if let (Cell::X, Cell::Empty, Cell::X) = (x, y, z) {
          temp_score += 2;
        }
        else if let (Cell::Empty, Cell::X, Cell::X) = (x, y, z) {
          temp_score += 2;
        }
        else if let (Cell::O, Cell::O, Cell::X) = (x, y, z) {
          temp_score += 1;
        }
        else if let (Cell::O, Cell::X, Cell::O) = (x, y, z) {
          temp_score += 1;
        }
        else if let (Cell::X, Cell::O, Cell::O) = (x, y, z) {
          temp_score += 1;
        }

        else if let (Cell::O, Cell::O, Cell::Empty) = (x, y, z) {
          temp_score -= 2;
        }
        else if let (Cell::Empty, Cell::O, Cell::O) = (x, y, z) {
          temp_score -= 2;
        }
        else if let (Cell::O, Cell::Empty, Cell::O) = (x, y, z) {
          temp_score -= 2;
        }
        else if let (Cell::X, Cell::X, Cell::O) = (x, y, z) {
          temp_score -= 1;
        }
        else if let (Cell::X, Cell::O, Cell::X) = (x, y, z) {
          temp_score -= 1;
        }
        else if let (Cell::O, Cell::X, Cell::X) = (x, y, z) {
          temp_score -= 1;
        }
      }
      // Count col.
      if i + 2 < board.get_cells().len() {
        let x = &board.get_cells()[i][j];
        let y = &board.get_cells()[i+1][j];
        let z = &board.get_cells()[i+2][j];
        
        if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
          temp_score += 3;
        } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
          temp_score -= 3;
        }

        else if let (Cell::X, Cell::X, Cell::Empty) = (x, y, z) {
          temp_score += 2;
        }
        else if let (Cell::X, Cell::Empty, Cell::X) = (x, y, z) {
          temp_score += 2;
        }
        else if let (Cell::Empty, Cell::X, Cell::X) = (x, y, z) {
          temp_score += 2;
        }
        else if let (Cell::O, Cell::O, Cell::X) = (x, y, z) {
          temp_score += 1;
        }
        else if let (Cell::O, Cell::X, Cell::O) = (x, y, z) {
          temp_score += 1;
        }
        else if let (Cell::X, Cell::O, Cell::O) = (x, y, z) {
          temp_score += 1;
        }

        else if let (Cell::O, Cell::O, Cell::Empty) = (x, y, z) {
          temp_score -= 2;
        }
        else if let (Cell::Empty, Cell::O, Cell::O) = (x, y, z) {
          temp_score -= 2;
        }
        else if let (Cell::O, Cell::Empty, Cell::O) = (x, y, z) {
          temp_score -= 2;
        }
        else if let (Cell::X, Cell::X, Cell::O) = (x, y, z) {
          temp_score -= 1;
        }
        else if let (Cell::X, Cell::O, Cell::X) = (x, y, z) {
          temp_score -= 1;
        }
        else if let (Cell::O, Cell::X, Cell::X) = (x, y, z) {
          temp_score -= 1;
        }
      }
      // 1st diagonal
      if i + 2 < board.get_cells().len() && j + 2 < board.get_cells().len() {
        let x = &board.get_cells()[i][j];
        let y = &board.get_cells()[i+1][j+1];
        let z = &board.get_cells()[i+2][j+2];
        
        if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
          temp_score += 3;
        } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
          temp_score -= 3;
        }

        else if let (Cell::X, Cell::X, Cell::Empty) = (x, y, z) {
          temp_score += 2;
        }
        else if let (Cell::X, Cell::Empty, Cell::X) = (x, y, z) {
          temp_score += 2;
        }
        else if let (Cell::Empty, Cell::X, Cell::X) = (x, y, z) {
          temp_score += 2;
        }
        else if let (Cell::O, Cell::O, Cell::X) = (x, y, z) {
          temp_score += 1;
        }
        else if let (Cell::O, Cell::X, Cell::O) = (x, y, z) {
          temp_score += 1;
        }
        else if let (Cell::X, Cell::O, Cell::O) = (x, y, z) {
          temp_score += 1;
        }

        else if let (Cell::O, Cell::O, Cell::Empty) = (x, y, z) {
          temp_score -= 2;
        }
        else if let (Cell::Empty, Cell::O, Cell::O) = (x, y, z) {
          temp_score -= 2;
        }
        else if let (Cell::O, Cell::Empty, Cell::O) = (x, y, z) {
          temp_score -= 2;
        }
        else if let (Cell::X, Cell::X, Cell::O) = (x, y, z) {
          temp_score -= 1;
        }
        else if let (Cell::X, Cell::O, Cell::X) = (x, y, z) {
          temp_score -= 1;
        }
        else if let (Cell::O, Cell::X, Cell::X) = (x, y, z) {
          temp_score -= 1;
        }
      }
      
      // 2nd diagonal
      if i + 2 < board.get_cells().len() && j >= 2 {
        let x = &board.get_cells()[i][j];
        let y = &board.get_cells()[i+1][j-1];
        let z = &board.get_cells()[i+2][j-2];
        
        if let (Cell::X, Cell::X, Cell::X) = (x, y, z) {
          temp_score += 3;
        } else if let (Cell::O, Cell::O, Cell::O) = (x, y, z) {
          temp_score -= 3;
        }

        else if let (Cell::X, Cell::X, Cell::Empty) = (x, y, z) {
          temp_score += 2;
        }
        else if let (Cell::X, Cell::Empty, Cell::X) = (x, y, z) {
          temp_score += 2;
        }
        else if let (Cell::Empty, Cell::X, Cell::X) = (x, y, z) {
          temp_score += 2;
        }
        else if let (Cell::O, Cell::O, Cell::X) = (x, y, z) {
          temp_score += 1;
        }
        else if let (Cell::O, Cell::X, Cell::O) = (x, y, z) {
          temp_score += 1;
        }
        else if let (Cell::X, Cell::O, Cell::O) = (x, y, z) {
          temp_score += 1;
        }

        else if let (Cell::O, Cell::O, Cell::Empty) = (x, y, z) {
          temp_score -= 2;
        }
        else if let (Cell::Empty, Cell::O, Cell::O) = (x, y, z) {
          temp_score -= 2;
        }
        else if let (Cell::O, Cell::Empty, Cell::O) = (x, y, z) {
          temp_score -= 2;
        }
        else if let (Cell::X, Cell::X, Cell::O) = (x, y, z) {
          temp_score -= 1;
        }
        else if let (Cell::X, Cell::O, Cell::X) = (x, y, z) {
          temp_score -= 1;
        }
        else if let (Cell::O, Cell::X, Cell::X) = (x, y, z) {
          temp_score -= 1;
        }
      }
    }
  }

  return temp_score;
}
pub struct SolutionAgent {}


  fn moves_exceed_four(board: &mut Board, player: Player, counter: i32) -> (i32, usize, usize) {
    if board.game_over() {
      return (board.score(), 10, 10);
    }
    if counter > 4 {
      // We use 10, 10 because there are no more moves available when the game
      // is over.
      let cells: &Vec<Vec<Cell>> = board.get_cells();
      cells[0][0] == Cell::X;
      return (heuristic(board), 10, 10);
    }
  
    // Get all the available legal moves.
    let moves = board.moves();
    
    // Try the first move, store it as the best move and score for now.
    let m = moves[0];
    let board_after_move = board.apply_move(m, player);  // `board` is unaffected, it is copied and the move is applied to the copy.
    let (mut best_score, _, _) = moves_exceed_four(board, player.invert(), counter + 1);
      board.undo_move(m, player);
    let mut best_move = m;
  
    // Try remaining moves, see if any of them are better.
    for i in 1..moves.len() {
      let m = moves[i];
      let board_after_move = board.apply_move(m, player);
      let (score, _, _) = moves_exceed_four(board, player.invert(), counter + 1);
        board.undo_move(m, player);
      match player {
        Player::X => {
          // We need the move with the max score for player X
          if score > best_score {
            best_score = score;
            best_move = m;
          }
        },
        Player::O => {
          // We need the move with the min score for player O
          if score < best_score {
            best_score = score;
            best_move = m;
          }
        },
      }
    }
  
    return (best_score, best_move.0, best_move.1);
  }
  
// Put your solution here.
impl Agent for SolutionAgent {
  fn solve(board: &mut Board, player: Player) -> (i32, usize, usize) {
    // If you want to make a recursive call to this agent, use
    // `SolutionAgent::solve(...)`
    moves_exceed_four(board, player, 0)
  }
}
    