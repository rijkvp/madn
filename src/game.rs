use rand::Rng;
use std::io::{self, Read};

pub const BOARD_SIZE: usize = 40;
pub const PLAYER_COUNT: usize = 4;
pub const PEG_COUNT: usize = 4;
pub const PLAYER_NAMES: &[&str] = &["Blue", "Yellow", "Green", "Red"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Peg {
    Unused,
    OnBoard(usize),
    Home(usize),
}

impl Peg {
    fn is_unused(&self) -> bool {
        match self {
            Self::Unused => true,
            _ => false,
        }
    }

    fn is_onboard(&self) -> bool {
        match self {
            Self::OnBoard(_) => true,
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
            pegs: [[Peg::Unused; PEG_COUNT]; PLAYER_COUNT],
        }
    }
}

impl Board {
    pub fn move_peg(&mut self, player: Player, peg: usize, amount: usize) {
        let Peg::OnBoard(peg_pos) = self.pegs[player.index()][peg] else {
            panic!("cannot move peg that is not on the board");
        };
        self.board[peg_pos] = 0;
        let dest = (peg_pos + amount) % BOARD_SIZE;
        self.board[dest] = player.num();
        self.pegs[player.index()][peg] = Peg::OnBoard(dest);
        println!("MOVE: {} peg {}, {} places", player.name(), peg, amount);
        // TODO: move in home
    }

    pub fn insert_peg(&mut self, player: Player, peg: usize) {
        let start_pos = player.start_pos();
        self.board[start_pos] = player.num();
        self.pegs[player.index()][peg] = Peg::OnBoard(start_pos);
        println!("INSERT: {}", player.name());
    }

    pub fn cells(&self) -> std::slice::Iter<u8> {
        self.board.iter()
    }

    pub fn player_pegs(&self, player: Player) -> std::slice::Iter<Peg> {
        self.pegs[player.index()].iter()
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

    pub fn player(&self) -> Player {
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
        println!("{}s turn", player.name());

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
            if let Some(inserted_peg) = inserted {
                self.board.move_peg(player, inserted_peg, roll);
                inserted = None;
            } else if roll == 6 && pegs_unused.len() > 0 {
                let peg_index = self
                    .board
                    .player_pegs(player)
                    .position(|p| p.is_unused())
                    .expect("there should be at least one unused peg");
                self.board.insert_peg(player, peg_index);
                inserted = Some(peg_index);
            } else if pegs_onboard.len() > 0 {
                let peg_index = self
                    .board
                    .player_pegs(player)
                    .position(|p| p.is_onboard())
                    .expect("there should be at least one peg on board");
                self.board.move_peg(player, peg_index, roll);
            }

            if roll != 6 {
                break;
            }
        }
    }
}
