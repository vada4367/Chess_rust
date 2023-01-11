mod Chess;

fn main() {
    let mut Chess = Chess::Chess::init(8, 8);
    Chess.init_figures();
    Chess.print();
}
