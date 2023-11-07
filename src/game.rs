use rand::Rng;
use std::io::{self, Read};

pub const BOARD_SIZE: usize = 40;
pub const PLAYER_COUNT: usize = 4;
pub const PEG_COUNT: usize = 4;
pub const PLAYER_NAMES: &[&str] = &["Blue", "Yellow", "Green", "Red"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Peg {
    Out,
    In(usize),
    Home(usize),
}

impl Peg {
    fn is_unused(&self) -> bool {
        match self {
            Self::Out => true,
            _ => false,
        }
    }

    fn is_onboard(&self) -> bool {
        match self {
            Self::In(_) => true,
            _ => false,
        }
    }

    fn is_home(&self) -> bool {
        match self {
            Self::Home(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Player(usize);

impl Player {
    fn index(&self) -> usize {
        self.0 as usize
    }

    fn start_pos(&self) -> usize {
        self.index() * 10
    }

    fn num(&self) -> u8 {
        (self.0 + 1) as u8
    }

    fn name(&self) -> &str {
        PLAYER_NAMES[self.index()]
    }

    fn next(&self) -> Player {
        Player((self.index() + 1) % PLAYER_COUNT)
    }
}

pub struct Board {
    board: [u8; BOARD_SIZE],
    pegs: [[Peg; PEG_COUNT]; PLAYER_COUNT],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [0u8; BOARD_SIZE],
            pegs: [[Peg::Out; PEG_COUNT]; PLAYER_COUNT],
        }
    }
}

impl Board {
    /// Moves the a peg of a player on the board
    pub fn move_peg(&mut self, player: Player, peg: usize, moves: usize) {
        let Peg::In(peg_pos) = self.pegs[player.index()][peg] else {
            panic!("cannot move peg that is not on the board");
        };
        self.board[peg_pos] = 0;
        let dest = (peg_pos + moves) % BOARD_SIZE;

        self.place_peg(dest, player, peg);

        println!("MOVE: {} peg {}, {} places", player.name(), peg, moves);
        // TODO: move in home
    }

    /// Insert a new peg on the board
    pub fn insert_peg(&mut self, player: Player, peg: usize) {
        println!("INSERT: {}", player.name());
        self.place_peg(player.start_pos(), player, peg);
    }

    /// Place a peg on the board, potentially slaying another peg
    pub fn place_peg(&mut self, pos: usize, player: Player, peg: usize) {
        let current = self.board[pos];
        if current != 0 {
            let player_idx = (current - 1) as usize;
            println!("THROW OUT: {}", PLAYER_NAMES[player_idx]);
            for peg in &mut self.pegs[player_idx] {
                if *peg == Peg::In(pos) {
                    *peg = Peg::Out;
                }
            }
        }
        self.board[pos] = player.num();
        self.pegs[player.index()][peg] = Peg::In(pos);
    }

    pub fn cells(&self) -> std::slice::Iter<u8> {
        self.board.iter()
    }

    pub fn player_pegs(&self, player: Player) -> std::slice::Iter<Peg> {
        self.pegs[player.index()].iter()
    }

    pub fn stats(&self) -> String {
        let mut str = String::new();
        for player_idx in 0..PLAYER_COUNT {
            str.push_str(PLAYER_NAMES[player_idx]);
            str.push(' ');
            for peg in self.pegs[player_idx] {
                str.push(match peg {
                    Peg::Out => 'O',
                    Peg::In(_) => 'I',
                    Peg::Home(_) => 'H',
                });
            }
            str.push('\n');
        }
        str
    }
}

fn roll_dice() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}

pub struct Game {
    board: Board,
    current_player: Player,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            board: Board::default(),
            current_player: Player(0),
        }
    }
}

impl Game {
    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn run(&mut self) {
        let mut input_buf = [0u8; 1];
        loop {
            self.perform_turn(self.current_player);

            println!(">> Press any key for next turn..");
            io::stdin().read_exact(&mut input_buf).unwrap();
            // next player
            self.current_player = self.current_player.next();
        }
    }

    pub fn next_turn(&mut self) {
        self.perform_turn(self.current_player);
        self.current_player = self.current_player.next();
    }

    pub fn perform_turn(&mut self, player: Player) {
        println!("\nTURN: {}", player.name());

        let mut inserted = None::<usize>;
        loop {
            let pegs_unused = self
                .board
                .player_pegs(player)
                .enumerate()
                .filter_map(|(i, p)| if p.is_unused() { Some(i) } else { None })
                .collect::<Vec<usize>>();
            let pegs_onboard = self
                .board
                .player_pegs(player)
                .enumerate()
                .filter_map(|(i, p)| if p.is_onboard() { Some(i) } else { None })
                .collect::<Vec<usize>>();
            let pegs_home = self
                .board
                .player_pegs(player)
                .enumerate()
                .filter_map(|(i, p)| if p.is_home() { Some(i) } else { None })
                .collect::<Vec<usize>>();

            let roll = roll_dice();
            println!("ROLL: {} rolls {}", player.name(), roll);
            if roll == 6 && pegs_unused.len() > 0 {
                // If 6 is rolled and the players has news e must insert
                // MUST if no pegs on board
                let first_unused_peg = self
                    .board
                    .player_pegs(player)
                    .position(|p| p.is_unused())
                    .unwrap();
                self.board.insert_peg(player, first_unused_peg);
                inserted = Some(first_unused_peg);
            } else if let Some(inserted_peg) = inserted {
                // The player MUST move the inserted that one
                self.board.move_peg(player, inserted_peg, roll);
                inserted = None;
            } else if pegs_onboard.len() > 0 {
                let peg_index = self
                    .board
                    .player_pegs(player)
                    .position(|p| p.is_onboard())
                    .unwrap();
                self.board.move_peg(player, peg_index, roll);
            }

            if roll != 6 {
                break;
            }
        }
    }
}
