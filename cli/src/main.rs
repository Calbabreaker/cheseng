use std::io::Write;

fn main() {
    let mut board = cheseng::Board::default();
    println!("{}\n", board);

    loop {
        if let Err(err) = on_update(&mut board) {
            println!("{}", err);
        }
    }
}

fn on_update(board: &mut cheseng::Board) -> Result<(), &'static str> {
    let input = get_input("Enter move (eg. e2e4): ").or(Err("Failed to get input!"))?;
    let test_move = input
        .parse::<cheseng::Move>()
        .or(Err("Invalid move notation!"))?;

    let legal_move = board.as_legal_move(test_move).ok_or("Not a legal move!")?;
    board.make_move(legal_move);
    println!("\n{}\n", board);
    Ok(())
}

fn get_input(message: &str) -> std::io::Result<String> {
    print!("{}", message);
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim_end().to_owned())
}
