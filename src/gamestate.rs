use piece::*;
use piece::Color::*;
use board::*;
use player::Player;
use std::io::stdin;
use std::collections::HashMap;

pub type ProduceFn = fn(&GameState, Point, &Player, &mut Vec<Move>);

//enum CheckState {CheckMate, Threatened, StaleMate, Alive}

#[derive(Debug)]
pub struct GameState {
    pub board: [[Field;8];8],
    pub players: [Player;2],
    pub moves: HashMap<Player, Vec<Move>>,
    pub current_player: usize
}

impl GameState {

    pub fn new() -> GameState {
        let mut game = GameState {
            board: [[Field::Empty; 8]; 8],
            players: [
                Player::new(Black, 1),
                Player::new(White, -1),
            ],
            //moves: HashMap::new(),
            moves: HashMap::with_capacity(2),
            current_player: 0
        } ;
        game.init_board();
        game.update_moves();
        game
    }

    /// Sets up the board.
    fn init_board(&mut self) {
        let mut board = self.board;

        for i in 0..8{
            board[1][i] = Field::Piece(BL_PAWN );
            board[6][i] = Field::Piece(WH_PAWN );
        }

        //black side                             //white side
        board[0][0] = Field::Piece(BL_ROOK   );  board[7][0] = Field::Piece(WH_ROOK   );
        board[0][7] = Field::Piece(BL_ROOK   );  board[7][7] = Field::Piece(WH_ROOK   );
        board[0][1] = Field::Piece(BL_KNIGHT );  board[7][1] = Field::Piece(WH_KNIGHT );
        board[0][6] = Field::Piece(BL_KNIGHT );  board[7][6] = Field::Piece(WH_KNIGHT );
        board[0][2] = Field::Piece(BL_BISHOP );  board[7][2] = Field::Piece(WH_BISHOP );
        board[0][5] = Field::Piece(BL_BISHOP );  board[7][5] = Field::Piece(WH_BISHOP );
        board[0][4] = Field::Piece(BL_KING   );  board[7][4] = Field::Piece(WH_KING   );
        board[0][3] = Field::Piece(BL_QUEEN  );  board[7][3] = Field::Piece(WH_QUEEN  );

        self.board = board;
    }

    /// Tests whether the current players king is threatened.
    pub fn test_check(&self,mov:&Move) -> bool {
        match ( self.get_field(mov.from), self.get_field(mov.to) ){

            (Field::Piece(me), Field::Piece(you))  => {
                if me.color != you.color && you.piece == Suit::King{
                    println!("{}({}) threatens {}({}) (PRESS ENTER)", me,mov.from, you, mov.to);
                    let mut devnull= String::new();
                    stdin().read_line(&mut devnull).unwrap();
                    return true;
                } false
            },
            _ => false
        }
    }

    /// The player whos turn it currently is.
    pub fn get_current_player(&self) -> &Player {
        &self.players[self.current_player]
    }

    pub fn get_field(&self,pos:Point) -> Field {
        match pos {
            Point{x:0...7,y:0...7} => self.board[pos.y as usize][pos.x as usize],
            Point{x:_,y:_} => Field::Outside,
        }
    }

    fn field_contains_opponent(&self, pos:Point, player:&Player) -> bool {
        if let Field::Piece(piece) = self.get_field(pos){
            return piece.color != player.color;
        }
        false
    }

    // TODO make this multithreaded
    fn possible_moves(&self, player:&Player) -> Box<Vec<Move>> {
        //iterate over all fields
        //if my chesspiece get all moves
        let mut my_moves = vec![];
        {
            let moves = &mut my_moves;
            for y in 0..8 { for x in 0..8 {
                let pos = Point{x:x,y:y};
                if let Field::Piece(piece) = self.get_field(pos){
                    if piece.color == player.color{
                        let move_producer = self.produce_moves(piece.piece);
                        move_producer(&self, pos, player, moves);
                    }
                }
            } }
        }
        return Box::new(my_moves);
    }

    /// Prints the board to the command line.
    pub fn print_board(&self) {
        for y in 0..8 {
            let row = self.board[y];
            print!("{}|", y+1);
            for x in 0..8 {
                let col = row[x];
                print!("{} ", col.char());
            }
            println!("");
        }
        println!(" |A B C D E F G H");
    }

    /// Performs one of the possible moves for the current player.
    pub fn performe_move(&mut self, index:usize) {
        let &Move{to,from, note} = &self.moves.get(self.get_current_player()).unwrap()[index];
        let from_field = self.board[from.y as usize][from.x as usize];

        if let Field::Piece(piece) = from_field{
            println!("{}: \"{}\" {} -> {}", piece, note, from, to,);
        } else{
            println!("{} EMPTY: \"{}\" {} -> {}", self.get_current_player().color, note, from, to,);
        }

        self.board[from.y as usize][from.x as usize] = Field::Empty;
        self.board[to.y as usize][to.x as usize] = from_field;
        self.swap_player();
        self.update_moves();
    }

    fn update_moves(&mut self) {
        self.moves = HashMap::new();
        for player in &self.players{
            let moves = self.possible_moves(player);
            self.moves.insert(player.clone(), *moves);
        }
    }

    fn swap_player(&mut self) {
        self.current_player = ((self.current_player as i8 * -1) + 1) as usize;
    }

    fn verify_on_board(&self, pos:Point) -> bool {
        match self.get_field(pos) {
            Field::Outside => false,
            _ => true
        }
    }


    /// ChessPiece-Moves

    fn produce_moves(&self, piece:Suit) -> ProduceFn {
        match piece{
            Suit::Pawn   => GameState::produce_pawn_moves,
            Suit::Bishop => GameState::produce_bishop_moves,
            Suit::Rook   => GameState::produce_rook_moves,
            Suit::Queen  => GameState::produce_queen_moves,
            Suit::Knight => GameState::produce_knight_moves,
            Suit::King   => GameState::produce_king_moves,
            //_ => GameState::produce_no_moves,
        }
    }

    fn produce_pawn_moves(&self, pos:Point, player:&Player, moves: &mut Vec<Move>) {
        // TODO produce_pawn_moves can be shortened
        let Point{x,y} = pos; //origin
        let possible_move = Move{from:pos, to:Point{x:x,y:y+1i8*player.direction}, note:"normale"}; // normal move one forward
        let possible_rush = Move{from:pos, to:Point{x:x,y:y+2i8*player.direction}, note: "rush"}; //  only from start point
        let possible_capture_l = Move {
            from:pos,
            to:Point{
                x:x-1i8*player.direction,
                y:y+1i8*player.direction
            }, note:"capture r"}; // capture to the right

        let possible_capture_r = Move {
            from:pos,
            to:Point{
                x:x+1i8*player.direction,
                y:y+1i8*player.direction
            }, note:"capture l"}; // capture to the left

        // verify moves
        if self.verify_on_board(possible_move.to) && !self.field_contains_opponent(possible_move.to, player){ // no opponent figure directly infront of pawn
            self.test_check(&possible_move);
            // 1->3 7->5 and no opponent figure two fields infront of pawn
            if self.verify_on_board(possible_rush.to)
                && 4-(player.direction * 3) == pos.y
                    && !self.field_contains_opponent(possible_rush.to, player){
                        moves.push(possible_rush); }
        }
        //else {println!("cant move from {:?}", &pos )};
        // verify captures
        if self.verify_on_board(possible_capture_l.to) && self.field_contains_opponent(possible_capture_l.to, player){
            self.test_check(&possible_capture_l);
            moves.push(possible_capture_l)};
        if self.verify_on_board(possible_capture_r.to) && self.field_contains_opponent(possible_capture_r.to, player){
            self.test_check(&possible_capture_r);
            moves.push(possible_capture_r)};
    }

    fn produce_knight_moves(&self, pos:Point, player:&Player, moves: &mut Vec<Move>) {
        for i in 0..4{
            let x_dir = (i/2)*2-1;
            let y_dir = (i%2)*2-1;

            for mov in [
                Move{from:pos, to:Point{ x:pos.x + x_dir*2, y:pos.y + y_dir, }, note:"normale"},
                Move{from:pos, to:Point{ x:pos.x + x_dir, y:pos.y + y_dir*2, }, note:"normale"}].iter(){

                    let field = self.get_field(mov.to);
                    match field {
                        Field::Outside => () ,
                        Field::Empty => moves.push(mov.clone()),
                        Field::Piece(piece) =>
                            if piece.color != player.color{
                                let mov = Move{note : "capture",..*mov};
                                self.test_check(&mov);
                                moves.push(mov);
                            }
                    }
                }
        }
    }

    fn produce_bishop_moves(&self, pos:Point, player:&Player, moves: &mut Vec<Move>) {
        for i in 0..4{
            self.produce_moves_for_dir(pos,player,moves,(i/2)*2-1, (i%2)*2-1, self.board.len() as i8)
        }
    }

    fn produce_rook_moves(&self, pos:Point, player:&Player, moves: &mut Vec<Move>) {
        for (x_dir, y_dir) in vec![ (0,1), (1,0), (0,-1), (-1,0) ]{ self.produce_moves_for_dir(pos,player,moves,x_dir, y_dir, self.board.len() as i8) }
    }

    fn produce_queen_moves(&self, pos:Point, player:&Player, moves: &mut Vec<Move>) {
        self.produce_rook_moves(pos,player,moves);
        self.produce_bishop_moves(pos,player,moves);
    }

    fn produce_king_moves(&self, pos:Point, player:&Player, moves: &mut Vec<Move>) {
        // am I threatened
        for i in 0..4{
            self.produce_moves_for_dir(pos,player,moves,(i/2)*2-1, (i%2)*2-1, 1)
        }
        for (x_dir, y_dir) in vec![ (0,1), (1,0), (0,-1), (-1,0) ]{ self.produce_moves_for_dir(pos,player,moves,x_dir, y_dir,1) }
    }

    fn produce_moves_for_dir (&self, pos:Point, player:&Player, moves: &mut Vec<Move>, x_dir:i8,y_dir:i8, dist:i8) {
        let mut check_pos = pos;

        for _ in 0..dist{
            check_pos = Point{ x: check_pos.x+x_dir, y: check_pos.y+y_dir };
            let field = self.get_field(check_pos);
            match field {
                Field::Outside => break,
                Field::Empty => {
                    let possible_move = Move{from:pos, to:check_pos, note:"normale"}; // normal move one forward
                    moves.push(possible_move);
                }
                Field::Piece(piece) => {
                    if piece.color != player.color{
                        let possible_capture = Move{from:pos, to:check_pos, note:"capture"}; // normal move one forward
                        self.test_check(&possible_capture);
                        moves.push(possible_capture);
                    }
                    break; //return to start
                }
            }
        }
    }
}

