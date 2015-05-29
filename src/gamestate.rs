use board::*;


#[derive(Copy,Clone,Debug)]
pub struct Point {pub x:i32, pub y:i32}

#[derive(Debug,Clone)]
pub struct Move  { from: Point, to: Point , note:&'static str}

#[derive(Debug)]
pub struct GameState
{
    pub board: Board,
    pub players: [Player;2],
    pub current_player: usize
}

pub type ProduceFn = fn(Point, &Player, &Board, &mut Vec<Move>);

pub fn produce_pawn_moves(pos:Point, player:&Player, board:&Board, moves: &mut Vec<Move>)
{
    let Point{x,y} = pos;
    moves.push(Move{from:pos, to:Point{x:x,y:y+1*player.direction}, note:"normale"});
    if 4-(player.direction * 3) == pos.y{ // 1->3 7->5
        moves.push(Move{from:pos, to:Point{x:x,y:y+2*player.direction},note: "rush"});
    }
}
pub fn produce_no_moves(pos:Point, player:&Player, board:&Board, moves: &mut Vec<Move>) {}

impl GameState
{
    pub fn new() -> GameState
    {
        GameState {
            board: [[Field::Empty; 8]; 8],
            players: [
                Player { color: Color::Black, direction: 1},
                Player { color: Color::White, direction: -1}
            ],
            current_player: 0
        }
    }

    pub fn current_player(&self) -> &Player
    {
        &self.players[self.current_player]
    }

    pub fn get_field(&self,pos:Point) -> &Field
    {
        &self.board[pos.y as usize][pos.x as usize]
    }

    fn get_move_producer(&self, piece:ChessPiece) -> ProduceFn
    {
        match piece{
            ChessPiece::Pawn => produce_pawn_moves,
            _ => produce_no_moves,
        }
    }

    // TODO make this multithreaded
    pub fn get_moves(&self, player:&Player) -> Vec<Move>
    {
        //iterate over all fields
        //if my chesspiece get all moves
        let mut moves = vec!();
        for y in 0..8 { for x in 0..8 {

            let pos = Point{x:x,y:y};
            if let &Field::Piece(piece) = self.get_field(pos){
                if piece.color == player.color{
                    let move_producer = self.get_move_producer(piece.piece);
                    move_producer(pos, player, &self.board, &mut moves);
                }
            }
        } }
        return moves;
    }

    pub fn print_board(&self)
    {
        for row in self.board.iter() {
            print!("|");
            for col in row{ print!("{}", col.char()); }
            println!("|");
        }
    }


    pub fn performe_move(&mut self, &Move{to,from, note}:&Move)
    {
        println!("from: {:?}({:?}), to: {:?}({:?})",
                 from,
                 self.board[from.y as usize][from.x as usize],
                 to,
                 self.board[to.y as usize][to.x as usize],
                 );
        let from_field = self.board[from.y as usize][from.x as usize] ;
        self.board[from.y as usize][from.x as usize] = Field::Empty;
        self.board[to.y as usize][to.x as usize] = from_field;
        self.swap_player();
    }

    fn swap_player(&mut self){
        if self.current_player == 0 { self.current_player = 1}
        else { self.current_player = 0}
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


fn verify_on_board(pos:Point) -> bool {
    match pos {
       Point{x:0...8,y:0...8} => true,
       Point{x:_,y:_} => false
    }
}


