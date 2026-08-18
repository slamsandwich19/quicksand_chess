mod move_list;
mod debug;

// third party
use cozy_chess::Board;

// project
use move_list::MoveList;
use debug::print_board;

fn main() {
    // Start position
    let board = Board::default();
    print_board(&board);

    // get legal moves
    let mut move_list = MoveList::new();
    board.generate_moves(|moves| {
        for mv in moves {
            move_list.push(mv);
        }
        return false
    });

    for index in 0..(move_list.count()) {
        println!("{}", move_list[index].unwrap());
    }
}
