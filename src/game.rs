use rand::Rng;
use std::io::{self, Read};

const BOARD_SIZE: usize = 40;
const PLAYER_COUNT: usize = 4;
const SIDE_SIZE: usize = BOARD_SIZE / PLAYER_COUNT;
const PEG_COUNT: usize = 4;
const PLAYER_NAMES: &[&str] = &["Blue", "Yellow", "Green", "Red"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Peg {
    Out,
    In(usize),
    Home(usize),
}

impl Peg {
    fn is_out(&self) -> bool {
        *self == Self::Out
    }

    fn is_in(&self) -> bool {
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
    pub fn index(&self) -> usize {
        self.0 as usize
    }

    fn num(&self) -> u8 {
        (self.0 + 1) as u8
    }

    pub fn name(&self) -> &str {
        PLAYER_NAMES[self.index()]
    }

    fn next(&self) -> Player {
        Player((self.index() + 1) % PLAYER_COUNT)
    }
}

pub struct Board {
    board: [u8; BOARD_SIZE],
    home: [[bool; PEG_COUNT]; PLAYER_COUNT],
    pegs: [[Peg; PEG_COUNT]; PLAYER_COUNT],
}

impl Default for Board {
    fn default() -> Self {
        Self {
            board: [0u8; BOARD_SIZE],
            home: [[false; PEG_COUNT]; PLAYER_COUNT],
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
        let dest = (peg_pos + moves) % BOARD_SIZE;
        let home_pos = (player.index() * SIDE_SIZE + (BOARD_SIZE - 1)) % BOARD_SIZE;
        let pos_in_round = (BOARD_SIZE + dest - home_pos) % BOARD_SIZE;
        if pos_in_round > 30 {
            // Move in HOME
            println!("MOVE HOME: {} peg {}, {} places", player.name(), peg, moves);
        } else {
            // Normal move
            println!("MOVE: {} peg {}, {} places", player.name(), peg, moves);
            self.board[peg_pos] = 0; // Clear the previous position
            self.place_peg(dest, player, peg);
        }
    }

    /// Insert a new peg on the board
    pub fn insert_peg(&mut self, player: Player, peg: usize) {
        println!("INSERT: {}", player.name());
        let start_pos = player.index() * SIDE_SIZE;
        self.place_peg(start_pos, player, peg);
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

    pub fn home_cells(&self, player: Player) -> std::slice::Iter<bool> {
        self.home[player.index()].iter()
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

    pub fn players(&self) -> impl Iterator<Item = Player> {
        (0..PLAYER_COUNT).into_iter().map(|i| Player(i)).into_iter()
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
            let pegs_out = self
                .board
                .player_pegs(player)
                .enumerate()
                .filter_map(|(i, p)| if p.is_out() { Some(i) } else { None })
                .collect::<Vec<usize>>();
            let pegs_in = self
                .board
                .player_pegs(player)
                .enumerate()
                .filter_map(|(i, p)| if p.is_in() { Some(i) } else { None })
                .collect::<Vec<usize>>();
            let pegs_home = self
                .board
                .player_pegs(player)
                .enumerate()
                .filter_map(|(i, p)| if p.is_home() { Some(i) } else { None })
                .collect::<Vec<usize>>();

            let roll = roll_dice();
            println!("ROLL: {} rolls {}", player.name(), roll);
            if roll == 6 && pegs_out.len() > 0 {
                // If 6 is rolled AND no pegs in a new one MUST be inserted
                // For now we just insert one whenever possible
                let first_unused_peg = self
                    .board
                    .player_pegs(player)
                    .position(|p| p.is_out())
                    .unwrap();
                self.board.insert_peg(player, first_unused_peg);
                inserted = Some(first_unused_peg);
            } else if let Some(inserted_peg) = inserted {
                // The player MUST move the inserted that one
                self.board.move_peg(player, inserted_peg, roll);
                inserted = None;
            } else if pegs_in.len() > 0 {
                // TODO: Select random peg to move
                let in_pegs = self
                    .board
                    .player_pegs(player)
                    .enumerate()
                    .filter(|(_, p)| p.is_in())
                    .collect::<Vec<(usize, &Peg)>>();
                self.board.move_peg(player, in_pegs[0].0, roll);
            }

            if roll != 6 {
                break;
            }
        }
    }
}
