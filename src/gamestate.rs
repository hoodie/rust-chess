use board::*;


#[derive(Debug)]
pub struct Point (pub i32, pub i32);
pub struct Move  { from: Point, to: Point }

pub struct GameState
{
    pub board: Board,
    pub current_player: Player
}

pub type VerifyFn = fn(Move, Player, GameState) -> bool;

impl GameState
{
    pub fn new() -> GameState
    {
        GameState {
            board: [[Field::Empty; 8]; 8],
            current_player: Player { color: Color::White, direction: -1}
        }
    }

    pub fn get_field(&self,p:Point) -> &Field{
        &self.board[p.0 as usize][p.1 as usize]
    }

    fn get_verifiers(piece:ChessPiece) -> VerifyFn
    {
        match piece{
            ChessPiece::Pawn => verify_pawn_move,
            _ => verify_pawn_move,
        }
    }

    pub fn verify_move(m:Move, p:Player, state:GameState) -> bool{
        false
    }

    pub fn print_board(&self)
    {
        for col in self.board.iter() {
            print!("|");
            for row in col{ print!("{}", row.char()); }
            println!("|");
        }
    }

    pub fn init_board(&mut self)
    {
        let mut board = self.board;
        for i in 0..8{
            board[1][i] = Field::Piece(BL_PAWN );
            board[6][i] = Field::Piece(WH_PAWN );
        }
        //black side
        board[0][0] = Field::Piece(BL_ROOK   );
        board[0][7] = Field::Piece(BL_ROOK   );
        board[0][1] = Field::Piece(BL_KNIGHT );
        board[0][6] = Field::Piece(BL_KNIGHT );
        board[0][2] = Field::Piece(BL_BISHOP );
        board[0][5] = Field::Piece(BL_BISHOP );
        board[0][4] = Field::Piece(BL_KING   );
        board[0][3] = Field::Piece(BL_QUEEN  );

        //white side
        board[7][0] = Field::Piece(WH_ROOK   );
        board[7][7] = Field::Piece(WH_ROOK   );
        board[7][1] = Field::Piece(WH_KNIGHT );
        board[7][6] = Field::Piece(WH_KNIGHT );
        board[7][2] = Field::Piece(WH_BISHOP );
        board[7][5] = Field::Piece(WH_BISHOP );
        board[7][3] = Field::Piece(WH_KING   );
        board[7][4] = Field::Piece(WH_QUEEN  );

        self.board = board;
    }
}


fn verify_on_board(p:Point) -> bool {
    match p {
       Point(0...8,0...8) => true,
       Point(_,_) => false
    }
}

pub fn verify_pawn_move(m:Move, player:Player, state:GameState) -> bool {
    true
        && verify_on_board(m.from)
        && verify_on_board(m.to)
}

