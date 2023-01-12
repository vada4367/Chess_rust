#[allow(non_snake_case)]
mod Chess;

fn main() {
    let mut chess = Chess::Chess::init(8, 8);
    chess.init_figures();
    chess.print();
}
