use prettychars::glyph;

// Example: Renders a complete 8x8 chess board with pieces in starting position,
// using double-line box drawing for the grid and Unicode shade characters
// for the checkerboard pattern. Run with: cargo run --example chessboard

fn main() {
    draw_chessboard();
}

fn draw_chessboard() {
    // Box drawing glyphs
    let tl = glyph("box.double.tl").unwrap();
    let tr = glyph("box.double.tr").unwrap();
    let bl = glyph("box.double.bl").unwrap();
    let br = glyph("box.double.br").unwrap();
    let h = glyph("box.double.h").unwrap();
    let v = glyph("box.double.v").unwrap();
    let cross = glyph("box.double.cross").unwrap();
    let t_down = glyph("box.double.t-down").unwrap();
    let t_up = glyph("box.double.t-up").unwrap();
    let t_left = glyph("box.double.t-left").unwrap();
    let t_right = glyph("box.double.t-right").unwrap();

    // Chess pieces - white
    let w_king = glyph("chess.king.white").unwrap();
    let w_queen = glyph("chess.queen.white").unwrap();
    let w_rook = glyph("chess.rook.white").unwrap();
    let w_bishop = glyph("chess.bishop.white").unwrap();
    let w_knight = glyph("chess.knight.white").unwrap();
    let w_pawn = glyph("chess.pawn.white").unwrap();

    // Chess pieces - black
    let b_king = glyph("chess.king.black").unwrap();
    let b_queen = glyph("chess.queen.black").unwrap();
    let b_rook = glyph("chess.rook.black").unwrap();
    let b_bishop = glyph("chess.bishop.black").unwrap();
    let b_knight = glyph("chess.knight.black").unwrap();
    let b_pawn = glyph("chess.pawn.black").unwrap();

    // Block characters for squares
    let light_square = glyph("shade.light").unwrap();
    let dark_square = glyph("shade.medium").unwrap();

    // Initial board position
    let mut board = vec![vec![None; 8]; 8];

    // Setup black pieces (row 0 and 1)
    board[0] = vec![
        Some(b_rook),
        Some(b_knight),
        Some(b_bishop),
        Some(b_queen),
        Some(b_king),
        Some(b_bishop),
        Some(b_knight),
        Some(b_rook),
    ];
    for item in &mut board[1] {
        *item = Some(b_pawn);
    }

    // Setup white pieces (row 6 and 7)
    for item in &mut board[6] {
        *item = Some(w_pawn);
    }
    board[7] = vec![
        Some(w_rook),
        Some(w_knight),
        Some(w_bishop),
        Some(w_queen),
        Some(w_king),
        Some(w_bishop),
        Some(w_knight),
        Some(w_rook),
    ];

    // Column labels
    println!("\n    a  b  c  d  e  f  g  h");
    print!("  {}", tl);
    for i in 0..8 {
        print!("{}{}{}", h, h, h);
        if i < 7 {
            print!("{}", t_down);
        } else {
            print!("{}", tr);
        }
    }
    println!();

    // Draw each row
    for (row_idx, row) in board.iter().enumerate() {
        print!("{} {}", 8 - row_idx, v);

        for (col_idx, piece) in row.iter().enumerate() {
            // Determine square color (checkerboard pattern)
            let is_light = (row_idx + col_idx) % 2 == 0;
            let bg = if is_light { light_square } else { dark_square };

            // Print piece or background
            if let Some(p) = piece {
                print!("{}{}{}", bg, p, bg);
            } else {
                print!("{}{}{}", bg, bg, bg);
            }

            print!("{}", v);
        }

        println!(" {}", 8 - row_idx);

        // Draw horizontal divider (except after last row)
        if row_idx < 7 {
            print!("  {}", t_right);
            for _ in 0..7 {
                print!("{}{}{}{}", h, h, h, cross);
            }
            println!("{}{}{}{}", h, h, h, t_left);
        }
    }

    // Bottom border
    print!("  {}", bl);
    for i in 0..8 {
        print!("{}{}{}", h, h, h);
        if i < 7 {
            print!("{}", t_up);
        } else {
            print!("{}", br);
        }
    }
    println!();
    println!("    a  b  c  d  e  f  g  h\n");

    // Legend
    println!("Legend:");
    println!(
        "  White: {} {} {} {} {} {}",
        w_king, w_queen, w_rook, w_bishop, w_knight, w_pawn
    );
    println!(
        "  Black: {} {} {} {} {} {}",
        b_king, b_queen, b_rook, b_bishop, b_knight, b_pawn
    );
}
