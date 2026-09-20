#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct Place {
    pub row: usize,
    pub file: usize
}

impl Place {
    fn outside_board(&self) -> bool {
        if self.row >= 8 {
            return true;
        }
        if self.file >= 8 {
            return true;
        }
        false
    }
    pub fn goto(&self, to: &Self) -> Move {
        Move {
            to: *to,
            from: *self,
            promotion: None
        }
    }
    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() != 2 {
            return None;
        }

        let mut chars = s.chars();
        let letter = chars.next();
        let num = chars.next();

        let file = match letter.map(|c| c.to_ascii_uppercase()) {
            Some('A') => 0,
            Some('B') => 1,
            Some('C') => 2,
            Some('D') => 3,
            Some('E') => 4,
            Some('F') => 5,
            Some('G') => 6,
            Some('H') => 7,
            _ => return None
        };

        let row = match num.map(|n| n.to_digit(10))?? {
            1 => 7,
            2 => 6,
            3 => 5,
            4 => 4,
            5 => 3,
            6 => 2,
            7 => 1,
            8 => 0,
            _ => return None
        };

        Some(Place { row, file })
    }

    /// returns None if it is an invalid Place, as in outside the board
    pub fn into_str(&self) -> Option<[char; 2]> {
        Some([
            match self.file {
                0 => 'A',
                1 => 'B',
                2 => 'C',
                3 => 'D',
                4 => 'E',
                5 => 'F',
                6 => 'G',
                7 => 'H',
                _ => return None
            },
            match self.row {
                0 => '8',
                1 => '7',
                2 => '6',
                3 => '5',
                4 => '4',
                5 => '3',
                6 => '2',
                7 => '1',
                _ => return None
            }
        ])
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    from: Place,
    to: Place,
    promotion: Option<PieceTypes>
}

impl Move {
    fn in_between_squares(&self) -> Vec<Place> {
        let diff_x = self.from.file.abs_diff(self.to.file);
        let diff_y = self.from.row.abs_diff(self.to.row);

        if diff_x != diff_y {
            if !(diff_x == 0 || diff_y == 0) {
                return vec![];
            }
        }

        let to_add_x: isize = match (diff_x, self.from.file.checked_sub(self.to.file)) {
            (0, _) => 0,
            (_, Some(_)) => -1,
            (_, None) => 1
        };

        let to_add_y: isize = match (diff_y, self.from.row.checked_sub(self.to.row)) {
            (0, _) => 0,
            (_, Some(_)) => -1,
            (_, None) => 1
        };

        let mut ret = Vec::new();
        let to = diff_x.max(diff_y).try_into().unwrap_or(isize::MAX);
        for n in 1..to {
            ret.push(Place { 
                row: self.from.row.saturating_add_signed(to_add_y * n), 
                file: self.from.file.saturating_add_signed(to_add_x * n),
            });
        }
            
        ret
    }

    fn in_between_and_end(&self) -> Vec<Place> {
        let mut ret = self.in_between_squares();

        ret.push(self.to);

        ret
    }

    pub fn from_str(s: &str) -> Option<Self> {
        if s.len() != 4 {
            return None;
        }
        Some(Place::from_str(&s[0..2])?.goto(&Place::from_str(&s[2..4])?))
    }

    pub fn into_promotion(mut self, promotion: Option<PieceTypes>) -> Self {
        self.promotion = promotion;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub enum Color {
    #[default] White,
    Black
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Color::Black => Color::White,
            Color::White => Color::Black
        }
    } 
}

#[derive(Clone, Debug, Copy, PartialEq, Eq, Hash)]
pub enum PieceTypes {
    Pawn {passantable: bool} ,
    Rook,
    Bishop,
    Knight,
    Queen,
    King
}

impl Default for PieceTypes {
    fn default() -> Self {
        PieceTypes::Pawn {passantable: false} 
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct Piece {
    pub piece_type: PieceTypes,
    pub place: Place,
    pub color: Color,
    last_moved: usize
}

impl Piece {
    fn create(piece_type: PieceTypes, place: Place, color: Color) -> Self {
        Self {
            last_moved: 0,
            piece_type,
            place,
            color
        }
    }

    pub fn into_char(&self) -> char {
        match (self.color, self.piece_type) {
            (Color::White, PieceTypes::King) => '♔',
            (Color::White, PieceTypes::Queen) => '♕',
            (Color::White, PieceTypes::Rook) => '♖',
            (Color::White, PieceTypes::Bishop) => '♗',
            (Color::White, PieceTypes::Knight) => '♘',
            (Color::White, PieceTypes::Pawn { .. }) => '♙',
            (Color::Black, PieceTypes::King) => '♚',
            (Color::Black, PieceTypes::Queen) => '♛',
            (Color::Black, PieceTypes::Rook) => '♜',
            (Color::Black, PieceTypes::Bishop) => '♝',
            (Color::Black, PieceTypes::Knight) => '♞',
            (Color::Black, PieceTypes::Pawn { .. }) => '♟',
        }
    }

    pub fn into_ascii(&self) -> char {
        match (self.color, self.piece_type) {
            (Color::White, PieceTypes::Pawn { .. }) => 'P',
            (Color::White, PieceTypes::Rook) => 'R',
            (Color::White, PieceTypes::Bishop) => 'B',
            (Color::White, PieceTypes::Knight) => 'N',
            (Color::White, PieceTypes::Queen) => 'Q',
            (Color::White, PieceTypes::King) => 'K',
            (Color::Black, PieceTypes::Pawn { .. }) => 'p',
            (Color::Black, PieceTypes::Rook) => 'r',
            (Color::Black, PieceTypes::Bishop) => 'b',
            (Color::Black, PieceTypes::Knight) => 'n',
            (Color::Black, PieceTypes::Queen) => 'q',
            (Color::Black, PieceTypes::King) => 'k',
        }
    }

    fn bishop_moves(&self) -> Vec<Place> {
        let mut moves = Vec::new();
        let own_row = self.place.row;
        let own_file = self.place.file;

        let max_back = own_file.min(own_row);

        let top_left = Place { row: own_row.saturating_sub(max_back), file: own_file.saturating_sub(max_back)};
        let bottom_left = Place { row: own_row.saturating_add(own_file), file: own_file.saturating_sub(own_file)};
        let top_right = Place { row: own_row.saturating_sub(own_row), file: own_file.saturating_add(own_row)};
            
        moves.extend(self.place.goto(&top_left).in_between_and_end());
        moves.extend(self.place.goto(&top_right).in_between_and_end());
        moves.extend(self.place.goto(&bottom_left).in_between_and_end());
        moves.extend(self.place.goto(&Place { row: own_row.saturating_add(7), file: own_file.saturating_add(7)}).in_between_and_end());
        moves.extend(vec![top_left, top_right, bottom_left]);

        moves.into_iter().filter(|s| !s.outside_board()).collect()
    }

    fn rook_moves(&self) -> Vec<Place> {
        let mut moves = Vec::new();
        let own_row = self.place.row;
        let own_file = self.place.file;
        
        moves.extend(self.place.goto(&Place { row: own_row.saturating_sub(7), file: own_file}).in_between_and_end());
        moves.extend(self.place.goto(&Place { row: own_row, file: own_file.saturating_add(7)}).in_between_and_end());
        moves.extend(self.place.goto(&Place { row: own_row.saturating_add(7), file: own_file}).in_between_and_end());
        moves.extend(self.place.goto(&Place { row: own_row, file: own_file.saturating_sub(7)}).in_between_and_end());
        
        moves.into_iter().filter(|s| !s.outside_board()).collect()
    }

    fn queen_moves(&self) -> Vec<Place> {
        let mut rook = self.rook_moves();
        rook.extend(self.bishop_moves());
        rook
    }

    fn king_moves_basic(&self) -> Vec<Place> {
        let own_row = self.place.row;
        let own_file = self.place.file;

        let moves = vec![Place {
            row: own_row + 1,
            file: own_file + 1
        },
        Place {
            row: own_row + 1,
            file: own_file
        },
        Place {
            row: own_row + 1,
            file: own_file.wrapping_sub(1)
        },
        Place {
            row: own_row,
            file: own_file.wrapping_sub(1)
        },
        Place {
            row: own_row.wrapping_sub(1),
            file: own_file.wrapping_sub(1)
        },
        Place {
            row: own_row.wrapping_sub(1),
            file: own_file
        },
        Place {
            row: own_row.wrapping_sub(1),
            file: own_file + 1
        },
        Place {
            row: own_row,
            file: own_file + 1
        }];

        moves.into_iter().filter(|s| !s.outside_board()).collect()
    }

    fn knight_moves(&self) -> Vec<Place> {
        let own_row = self.place.row;
        let own_file = self.place.file;

        let moves = vec![Place {
            row: own_row + 2,
            file: own_file + 1
        },
        Place {
            row: own_row + 1,
            file: own_file + 2
        },
        Place {
            row: own_row.wrapping_sub(2),
            file: own_file.wrapping_sub(1)
        },
        Place {
            row: own_row.wrapping_sub(1),
            file: own_file.wrapping_sub(2)
        },
        Place {
            row: own_row + 1,
            file: own_file.wrapping_sub(2)
        },
        Place {
            row: own_row + 2,
            file: own_file.wrapping_sub(1)
        },
        Place {
            row: own_row.wrapping_sub(1),
            file: own_file + 2
        },
        Place {
            row: own_row.wrapping_sub(2),
            file: own_file + 1
        }];

        moves.into_iter().filter(|s| !s.outside_board()).collect()
    }

    fn pawn_moves(&self, position: &Position) -> Vec<Move> {
        let mut candidates: Vec<Place> = Vec::new();
        
        let forward: isize = match self.color {
            Color::Black => 1,
            Color::White => -1
        };

        let determine_pawn_capture = |towards: Place| {
            if position.piece_on(towards).is_some_and(|f| f.color != self.color) {
                return true;
            }

            let passant_square = Place {
                row: towards.row.saturating_sub_signed(forward),
                file: towards.file
            };

            position.piece_on(passant_square).is_some_and(|p| match p.piece_type {
                PieceTypes::Pawn { passantable: true } => true,
                _ => false
            })
        };

        let capture_right = Place {
            file: self.place.file + 1,
            row: self.place.row.saturating_add_signed(forward)
        };

        let capture_left = Place {
            file: self.place.file.wrapping_sub(1),
            row: self.place.row.saturating_add_signed(forward)
        };

        if determine_pawn_capture(capture_right) {
            candidates.push(capture_right);
        }

        if determine_pawn_capture(capture_left) {
            candidates.push(capture_left);
        }

        let just_forward = Place {
            file: self.place.file,
            row: self.place.row.saturating_add_signed(forward)
        };

        if position.piece_on(just_forward).is_none() {
            candidates.push(just_forward);
        }
        

        let mut moves = Vec::new();

        for candidate in candidates {
            if candidate.row == 0 || candidate.row == 7 {
                moves.push(Move { 
                    from: self.place, 
                    to: candidate, 
                    promotion: Some(PieceTypes::Bishop) 
                });
                moves.push(Move { 
                    from: self.place, 
                    to: candidate, 
                    promotion: Some(PieceTypes::Knight) 
                });
                moves.push(Move { 
                    from: self.place, 
                    to: candidate, 
                    promotion: Some(PieceTypes::Queen) 
                });
                moves.push(Move { 
                    from: self.place, 
                    to: candidate, 
                    promotion: Some(PieceTypes::Rook) 
                });
                continue;
            }
            moves.push(Move {
                from: self.place,
                to: candidate,
                promotion: None,
            });
        }

        if !(self.last_moved == 0 && (self.place.row == 6 || self.place.row == 1)) {
            return moves;
        }
        
        let candidate_move = Place {
            file: self.place.file,
            row: self.place.row.saturating_add_signed(forward.saturating_mul(2)),
        };

        if !(position.theres_obstacle(&self.place, &candidate_move) || position.piece_on(candidate_move).is_some()) {
            moves.push(Move {
                from: self.place,
                to: candidate_move,
                promotion: None
            })
        }

        moves.into_iter().filter(|s| !s.to.outside_board()).collect()
    }


    fn king_moves(&self, position: &Position) -> Vec<Move> {
        let mut moves = self.king_moves_basic();
        if self.last_moved == 0 {
            let left_rook = position.piece_on(Place { row: self.place.row, file: 0 });
            if left_rook.is_some_and(|p| p.color == self.color && p.piece_type == PieceTypes::Rook && p.last_moved == 0) {
                moves.push(Place { row: self.place.row, file: self.place.file - 2 });
            }
            let right_rook = position.piece_on(Place { row: self.place.row, file: 7 });
            if right_rook.is_some_and(|p| p.color == self.color && p.piece_type == PieceTypes::Rook && p.last_moved == 0) {
                moves.push(Place { row: self.place.row, file: self.place.file + 2 });
            }
        }
        
        let mut ret = vec![];

        let mut attacking: Vec<Place> = position.moves_from_without_king(self.color.opposite())
            .into_iter()
            .map(|m| m.to)
            .collect();

        let opposite_king = position.pieces.iter().filter(|p| p.color == self.color.opposite() && p.piece_type == PieceTypes::King).next();
        if let Some(k) = opposite_king {
            attacking.extend(k.king_moves_basic());
        }

        for pl in moves {
            if let Some(extra) = self.place.goto(&pl).in_between_squares().pop() {
                if attacking.contains(&extra) {
                    continue;
                }
                if position.piece_on(extra).is_some() {
                    continue;
                }
            }
            if !attacking.contains(&pl) {
                ret.push(Move {
                    from: self.place,
                    to: pl,
                    promotion: None
                });
            }
        }

        ret.into_iter().filter(|m| !position.piece_on(m.to).is_some_and(|pi| pi.color == self.color)).collect()
    }

    pub fn moves_disregard_check(&self, position: &Position) -> Vec<Move>{
        let nonspecial_moves = match self.piece_type {
            PieceTypes::Bishop => self.bishop_moves(),
            PieceTypes::Rook => self.rook_moves(),
            PieceTypes::Queen => self.queen_moves(),
            PieceTypes::Pawn {..} => Vec::new(),
            PieceTypes::Knight => self.knight_moves(),
            PieceTypes::King => { return self.king_moves(&position); }
        };

        // special pawn and king
        
        let specials = match self.piece_type {
            PieceTypes::Pawn {..} => self.pawn_moves(position),
            _ => Vec::new()
        };

        let obstacles_filtered = match (nonspecial_moves, &self.piece_type) {
            (m, PieceTypes::Knight) => m,
            (m, _) => m.into_iter().filter(|pl| !position.theres_obstacle(&self.place, pl)).collect(),
        };

        // filter moving onto oneself

        obstacles_filtered
            .into_iter()
            .filter(|pl| !position.piece_on(*pl).is_some_and(|p| p.color == self.color))
            .map(|pl| Move { from: self.place, to: pl, promotion: None})
            .chain(specials.into_iter())
            .collect()

    }

    pub fn moves(&self, position: &Position) -> Vec<Move> {
        let all_moves = self.moves_disregard_check(position);
        all_moves.into_iter().filter(|m| {
            match position.clone().execute_move(*m) {
                Ok(p) => p.legal_position(),
                Err(_) => false
            }
        }).collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Square {
    #[default] Empty,
    Piece(Piece)
}


#[derive(Debug, Clone)]
pub struct Position {
    turn: Color,
    move_number: usize,
    pub pieces: Vec<Piece>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EndStates {
    Checkmate,
    Stalemate,
    None
}

impl Position {

    pub fn game_end(&self) -> EndStates {
        let check = self.in_check(self.turn);
        let moves = self.all_moves();

        dbg!(check);
        dbg!(&moves);

        if check && moves.len() == 0 {
            return EndStates::Checkmate;
        }

        if moves.len() == 0 {
            return EndStates::Stalemate;
        }

        return EndStates::None;
    }

    pub fn in_check(&self, color: Color) -> bool {
        let king = self.pieces.iter().filter(|p| p.color == color && p.piece_type == PieceTypes::King).next();
        let king = match king {
            Some(k) => k,
            None => return false
        };

        self.pieces
            .iter()
            .flat_map(|p| p.moves_disregard_check(self).into_iter())
            .map(|m| m.to)
            .fold(false, |r, sq| r || (sq == king.place))
    }
    
    pub fn legal_position(&self) -> bool {
        let white_king = self.pieces.iter().filter(|p| p.color == Color::White && p.piece_type == PieceTypes::King).next();
        let black_king = self.pieces.iter().filter(|p| p.color == Color::Black && p.piece_type == PieceTypes::King).next();

        match (white_king, black_king) {
            (Some(_), Some(_)) => (),
            _ => return false
        };

        return !self.in_check(self.turn.opposite());
    }

    fn theres_obstacle(&self, from: &Place, to: &Place) -> bool {
        let between = from.goto(to).in_between_squares();
        for sq in between {
            if let Some(_) = self.piece_on(sq) {
                return true;
            }
        }

        false
    }

    pub fn piece_on(&self, square: Place) -> Option<&Piece> {
        for piece in &self.pieces {
            if piece.place == square {
                return Some(piece);
            }
        }
        None
    }

    fn piece_on_mut(&mut self, square: Place) -> Option<&mut Piece> {
        for p in &mut self.pieces {
            if p.place != square {
                continue;
            }

            return Some(p)
        }
        None
    }

    fn moves_from_without_king(&self, color: Color) -> Vec<Move>{
        let mut moves = Vec::new();
        
        for piece in &self.pieces {
            if piece.color != color {
                continue;
            }
            if piece.piece_type == PieceTypes::King {
                continue;
            }
            moves.extend(piece.moves_disregard_check(self));
        }

        moves
    }

    /// return all possible moves from specified color
    pub fn all_moves_from(&self, color: Color) -> Vec<Move> {
        let mut moves = Vec::new();
        for piece in &self.pieces {
            if piece.color != color {
                continue;
            }

            moves.extend(piece.moves(self));
        }
        
        moves
    }

    /// returns all possible moves from whose turn it is.
    pub fn all_moves(&self) -> Vec<Move> {
        self.all_moves_from(self.turn)
    }

    fn move_and_capture(mut self, to_move: Move) -> Result<Self, Self> {
        if let Some((index, _capturing_piece)) = self.pieces.iter().enumerate().find(|(_, p)| p.place == to_move.to) {
            self.pieces.remove(index);
        }

        let moving_piece = self.pieces.iter().enumerate().find(|(_, p)| p.place == to_move.from);
        let (index, _) = match moving_piece {
            None => return Err(self),
            Some(p) => p
        };

        let moving_piece = self.pieces.get_mut(index).expect("impossible");
        self.move_number = self.move_number + 1;
        moving_piece.last_moved = self.move_number;
        moving_piece.place = to_move.to;


        self.turn = self.turn.opposite();

        Ok(self)
    }

    fn move_pawn_before_promotion(mut self, to_move: Move) -> Result<Self, Self>{
        match self.piece_on_mut(to_move.from) {
            Some(p) => p.piece_type = PieceTypes::Pawn { passantable: to_move.from.row.abs_diff(to_move.to.row) == 2 },
            None => return Err(self)
        }

        if to_move.from.file.abs_diff(to_move.to.file) == 0 {
            return self.move_and_capture(to_move);
        }
        match self.piece_on(to_move.to) {
            Some(_) => return self.move_and_capture(to_move),
            None => ()
        };

        // here must be passant 
        let mut to_capture = to_move.to;
        to_capture.row = to_move.from.row;
        let piece_to_capture = self.piece_on(to_capture);
        let piece_to_capture = match piece_to_capture {
            Some(p) if p.piece_type == PieceTypes::Pawn { passantable: true } && p.last_moved == self.move_number => p,
            _ => return Err(self)
        };

        if let Some((index, _capturing_piece)) = self.pieces.iter().enumerate().find(|(_, p)| p.place == piece_to_capture.place) {
            self.pieces.remove(index);
        }

        self.move_and_capture(to_move)
    }

    fn move_pawn(self, to_move: Move) -> Result<Self, Self> {
        let mut ret = self.move_pawn_before_promotion(to_move)?;
        let promoting_to = match to_move.promotion {
            Some(p) => p,
            None => return Ok(ret),
        };

        let to_promote = ret.piece_on_mut(to_move.to).expect("if successfully moved then this doesnt fail");
        to_promote.piece_type = promoting_to;

        return Ok(ret);
    }
    
    fn move_king(mut self, to_move: Move) -> Result<Self, Self> {
        let movenr = self.move_number;
        
        let diff_x = to_move.to.file.abs_diff(to_move.from.file);
        if diff_x != 2 {
            return self.move_and_capture(to_move);
        }

        let (to_left, rook) = match to_move.to.file {
            2 => (true, self.piece_on_mut(Place { row: to_move.to.row, file: 0 })),
            6 => (false, self.piece_on_mut(Place { row: to_move.to.row, file: 7})),
            _ => return Err(self)
        };

        let rook = match rook {
            Some(p) => p,
            None => return Err(self)
        };

        if to_left {
            rook.place.file = rook.place.file + 3;
        } else {
            rook.place.file = rook.place.file - 2;
        }
        rook.last_moved = movenr;

        self.move_and_capture(to_move)
    }

    // returns Ok(Self) if move could be implemented and Err(Self) if not possible, then it returns itself
    pub fn execute_move(self, to_move: Move) -> Result<Self, Self> {

        let piece = self.piece_on(to_move.from);
        let piece = match piece {
            Some(p) => p,
            None => return Err(self)
        };

        match piece.piece_type {
            PieceTypes::King => self.move_king(to_move),
            PieceTypes::Pawn { .. } => self.move_pawn(to_move),
            _ => self.move_and_capture(to_move)
        }
    }

    pub fn execute_move_checked(self, to_move: Move) -> Self {
        let all_moves = self.all_moves_from(self.turn);
        assert!(all_moves.contains(&to_move));
        self.execute_move(to_move).unwrap()
    }

    /// This function is made in honor of @ecogreen123 (context: was joking around in VC while coding)
    pub fn from_fen(fen: &str) -> Option<Self> {
        let turn = match fen.contains("w") {
            false => Color::Black,
            true => Color::White
        };

        let board_rep = match fen.split(" ").next() {
            None => return None,
            Some(s) => s
        };

        let flat_board: Vec<_> = board_rep.chars().flat_map(|c| match c {
            '1' => vec![None],
            '2' => vec![None; 2],
            '3' => vec![None; 3],
            '4' => vec![None; 4],
            '5' => vec![None; 5],
            '6' => vec![None; 6],
            '7' => vec![None; 7],
            '8' => vec![None; 8],
            'r' => vec![Some((Color::Black, PieceTypes::Rook))],
            'n' => vec![Some((Color::Black, PieceTypes::Knight))],
            'b' => vec![Some((Color::Black, PieceTypes::Bishop))],
            'q' => vec![Some((Color::Black, PieceTypes::Queen))],
            'k' => vec![Some((Color::Black, PieceTypes::King))],
            'p' => vec![Some((Color::Black, PieceTypes::Pawn {passantable: false}))],
            'R' => vec![Some((Color::White, PieceTypes::Rook))],
            'N' => vec![Some((Color::White, PieceTypes::Knight))],
            'B' => vec![Some((Color::White, PieceTypes::Bishop))],
            'Q' => vec![Some((Color::White, PieceTypes::Queen))],
            'K' => vec![Some((Color::White, PieceTypes::King))],
            'P' => vec![Some((Color::White, PieceTypes::Pawn {passantable: false}))],
            _ => vec![],
        }).collect();

        if flat_board.len() != 64 {
            return None;
        }

        let pieces = flat_board.into_iter().enumerate().filter_map(|(i, p)| match p {
            None => None,
            Some((c, t)) => Some(Piece::create(t, Place { row: i / 8, file: i % 8 }, c))
        }).collect();

        Some(Position {
            turn,
            move_number: 0,
            pieces
        })
    }
}

impl Default for Position {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        
        for row in 0..8 {
            for file in 0..8 {
                write!(f, "{}", self.pieces
                    .iter()
                    .filter(|p| p.place == Place { row, file })
                    .map(|p| p.into_char())
                    .next()
                    .unwrap_or('.'));
                
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "chess_tests.rs"]
mod tests;