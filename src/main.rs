use rand::Rng;
use std::io::{self, Read};

const BOARD_SIZE: usize = 40;
const PLAYER_COUNT: usize = 4;
const PEG_COUNT: usize = 4;
const PLAYER_NAMES: &[&str] = &["Red", "Green", "Blue", "Yellow"];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Peg {
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
struct Player(usize);

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

struct Board {
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
    fn move_peg(&mut self, player: Player, peg: usize, amount: usize) {
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

    fn insert_peg(&mut self, player: Player, peg: usize) {
        let start_pos = player.start_pos();
        self.board[start_pos] = player.num();
        self.pegs[player.index()][peg] = Peg::OnBoard(start_pos);
        println!("INSERT: {}", player.name());
    }

    fn player_pegs(&self, player: Player) -> std::slice::Iter<Peg> {
        self.pegs[player.index()].iter()
    }
}

fn roll_dice() -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}

fn main() {
    let mut board = Board::default();
    let mut player: Player = Player(0);
    let mut input_buf = [0u8; 1];
    loop {
        println!("{}s turn", player.name());
        let pegs_unused = board
            .player_pegs(player)
            .enumerate()
            .filter_map(|(i, p)| if p.is_unused() { Some(i) } else { None })
            .collect::<Vec<usize>>();
        let pegs_onboard = board
            .player_pegs(player)
            .enumerate()
            .filter_map(|(i, p)| if p.is_onboard() { Some(i) } else { None })
            .collect::<Vec<usize>>();
        let pegs_home = board
            .player_pegs(player)
            .enumerate()
            .filter_map(|(i, p)| if p.is_home() { Some(i) } else { None })
            .collect::<Vec<usize>>();

        let mut inserted = None::<usize>;
        loop {
            let roll = roll_dice();
            println!("ROLL: {} rolls {}", player.name(), roll);
            if let Some(inserted_peg) = inserted {
                board.move_peg(player, inserted_peg, roll);
                inserted = None;
            } else if roll == 6 && pegs_unused.len() > 0 {
                let peg_index = board
                    .player_pegs(player)
                    .position(|p| p.is_unused())
                    .expect("there should be at least one unused peg");
                board.insert_peg(player, peg_index);
                inserted = Some(peg_index);
            } else if pegs_onboard.len() > 0 {
                let peg_index = board
                    .player_pegs(player)
                    .position(|p| p.is_onboard())
                    .expect("there should be at least one peg on board");
                board.move_peg(player, peg_index, roll);
            }

            if roll != 6 {
                break;
            }
        }

        println!(">> Press any key for next turn..");
        io::stdin().read_exact(&mut input_buf).unwrap();

        // next player
        player = player.next();
    }
}
