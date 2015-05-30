use board::*;

#[derive(Copy,Clone,Debug)]
pub struct Point {pub x:i32, pub y:i32}

#[derive(Debug,Clone)]
pub struct Move  { from: Point, to: Point , pub note:&'static str}

pub type ProduceFn = fn(Point, &Player, &GameState, &mut Vec<Move>);

pub fn produce_pawn_moves(pos:Point, player:&Player, state:&GameState, moves: &mut Vec<Move>)
{
    let Point{x,y} = pos; //origin
    let possible_move = Move{from:pos, to:Point{x:x,y:y+1*player.direction}, note:"normale"}; // normal move one forward
    let possible_rush = Move{from:pos, to:Point{x:x,y:y+2*player.direction},note: "rush"}; //  only from start point
    let possible_capture_l = Move {
        from:pos,
        to:Point{
            x:x-1*player.direction,
            y:y+1*player.direction
        }, note:"capture r"}; // capture to the right

    let possible_capture_r = Move {
        from:pos,
        to:Point{
            x:x+1*player.direction,
            y:y+1*player.direction
        }, note:"capture l"}; // capture to the left

    // verify moves
    if GameState::verify_on_board(possible_move.to) && !state.field_contains_occonent(possible_move.to, player){ // no opponent figure directly infront of pawn
        moves.push(possible_move);
        // 1->3 7->5 and no opponent figure two fields infront of pawn
        if GameState::verify_on_board(possible_rush.to)
            && 4-(player.direction * 3) == pos.y
            && !state.field_contains_occonent(possible_rush.to, player){
                moves.push(possible_rush); }
    }
    //else {println!("cant move from {:?}", &pos )};
    // verify captures
    if GameState::verify_on_board(possible_capture_l.to) && state.field_contains_occonent(possible_capture_l.to, player){ moves.push(possible_capture_l)};
    if GameState::verify_on_board(possible_capture_r.to) && state.field_contains_occonent(possible_capture_r.to, player){ moves.push(possible_capture_r)};
}

pub fn produce_bishop_moves(pos:Point, player:&Player, state:&GameState, moves: &mut Vec<Move>)
{
    for i in 0..4{
        produce_moves_for_dir(pos,player,state,moves,(i/2)*2-1, (i%2)*2-1)
    }
}

pub fn produce_knight_moves(pos:Point, player:&Player, state:&GameState, moves: &mut Vec<Move>){}
pub fn produce_rook_moves(pos:Point, player:&Player, state:&GameState, moves: &mut Vec<Move>)
{
    for (x_dir, y_dir) in vec![ (0,1), (1,0), (0,-1), (-1,0) ]{
        produce_moves_for_dir(pos,player,state,moves,x_dir, y_dir)
    }
}

fn produce_moves_for_dir (pos:Point, player:&Player, state:&GameState, moves: &mut Vec<Move>, x_dir:i32,y_dir:i32)
{

        let mut check_pos = pos; // copy?
        //println!("\nx_dir: {}    y_dir: {}",x_dir, y_dir);
        //println!("check_pos: {:?}",check_pos);

        //for _ in 0..8{
        loop{
            check_pos = Point{
                x: check_pos.x+x_dir,
                y: check_pos.y+y_dir
            };
            //println!("check_pos: {:?}",check_pos);
            if GameState::verify_on_board(check_pos){
                let field = state.get_field(check_pos);
                match field {
                    &Field::Empty => {
                        //println!("added(empty)");
                        let possible_move = Move{from:pos, to:check_pos, note:"normale"}; // normal move one forward
                        moves.push(possible_move);
                    }
                    &Field::Piece(piece) => {
                        if state.field_contains_occonent(check_pos, state.current_player()){;
                            //println!("added(player)");
                            let possible_capture = Move{from:pos, to:check_pos, note:"capture"}; // normal move one forward
                            moves.push(possible_capture);
                        }
                        break; //return to start
                        //println!("|hit occupied field");
                    }
                }
            }
            else{
                //println!("|hit end of board");
                break;
            }
        }
}

//pub fn produce__moves(pos:Point, player:&Player, state:&GameState, moves: &mut Vec<Move>){}
//pub fn produce__moves(pos:Point, player:&Player, state:&GameState, moves: &mut Vec<Move>){}
//pub fn produce__moves(pos:Point, player:&Player, state:&GameState, moves: &mut Vec<Move>){}
pub fn produce_no_moves(pos:Point, player:&Player, state:&GameState, moves: &mut Vec<Move>) {}

#[derive(Debug)]
pub struct GameState
{
    pub board: Board,
    pub players: [Player;2],
    pub current_player: usize
}

impl GameState
{
    //static methods
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

    pub fn verify_on_board(pos:Point) -> bool
    {
        match pos {
            Point{x:0...7,y:0...7} => true,
            Point{x:_,y:_} => false
        }
    }

    // instance methods
    pub fn current_player(&self) -> &Player
    {
        &self.players[self.current_player]
    }

    pub fn field_contains_occonent(&self, pos:Point, player:&Player) -> bool
    {
            if let &Field::Piece(piece) = self.get_field(pos){
            return piece.color != player.color;
        }
        return false;
    }

    pub fn get_field(&self,pos:Point) -> &Field
    {
        &self.board[pos.y as usize][pos.x as usize]
    }

    fn get_move_producer(&self, piece:ChessPiece) -> ProduceFn
    {
        match piece{
            ChessPiece::Pawn => produce_pawn_moves,
            ChessPiece::Bishop => produce_bishop_moves,
            ChessPiece::Rook => produce_rook_moves,
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
                    move_producer(pos, player, &self, &mut moves);
                }
            }
        } }
        return moves;
    }

    pub fn print_board(&self)
    {
        for y in (0..8) {
            let row = self.board[y];
            print!("{}| ", y);
            for x in (0..8) {
                let col = self.board[y][x];
                print!("{} ", col.char());
            }
            println!("|");
        }
            println!(" | 0 1 2 3 4 5 6 7");
        }

    pub fn performe_move(&mut self, &Move{to,from, note}:&Move)
    {
        let from_field = self.board[from.y as usize][from.x as usize] ;
        let to_field = self.board[to.y as usize][to.x as usize] ;

        if let Field::Piece(piece) = from_field{
            println!("{:?} {:?}: \"{}\" {:?} -> {:?}", self.current_player().color, piece.piece, note, from, to,);
        } else{
            println!("{:?} EMPTY: \"{}\" {:?} -> {:?}", self.current_player().color, note, from, to,);
        }

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
        board[7][4] = Field::Piece(WH_KING   );
        board[7][3] = Field::Piece(WH_QUEEN  );

        self.board = board;
    }
}




