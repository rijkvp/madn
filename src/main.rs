mod game;

use game::Game;
use notan::draw::*;
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new().set_vsync(true);
    notan::init_with(State::new)
        .add_config(win_config)
        .add_config(DrawConfig)
        .draw(draw)
        .update(update)
        .build()
}

#[derive(AppState)]
struct State {
    game: Game,
    font: Font,
}

impl State {
    fn new(gfx: &mut Graphics) -> Self {
        let font = gfx
            .create_font(include_bytes!("../res/FiraSans-Regular.ttf"))
            .unwrap();
        Self {
            font,
            game: Game::default(),
        }
    }
}

fn update(app: &mut App, state: &mut State) {
    if app.keyboard.was_pressed(KeyCode::Space) {
        state.game.next_turn();
    } else if app.keyboard.is_down(KeyCode::Return) {
        state.game.next_turn();
    }
}

const CELL_SIZE: f32 = 16.0;
const MARGIN: f32 = 12.0;
const CELL_SPACE: f32 = 12.0;
const TEAM_COLORS: [Color; 4] = [Color::BLUE, Color::YELLOW, Color::GREEN, Color::RED];
const CELL_COLORS: [Color; 4] = [
    Color::new(0.3, 0.3, 1.0, 1.0),
    Color::new(1.0, 1.0, 0.3, 1.0),
    Color::new(0.3, 1.0, 0.3, 1.0),
    Color::new(1.0, 0.3, 0.3, 1.0),
];

fn another((base, cross): (i32, i32), side: usize) -> (i32, i32) {
    // directions: 0 = north, 1 = east, 2 = south, 3 = west
    let (base_mod, cross_mod) = match side {
        0 => (0, 3),
        1 => (1, 0),
        2 => (2, 1),
        3 => (3, 2),
        _ => unreachable!(),
    };
    let (base_mod_x, base_mod_y) = match base_mod {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => unreachable!(),
    };
    let (cross_mod_x, cross_mod_y) = match cross_mod {
        0 => (0, -1),
        1 => (1, 0),
        2 => (0, 1),
        3 => (-1, 0),
        _ => unreachable!(),
    };
    let x_offset = base * base_mod_x + cross * cross_mod_x;
    let y_offset = base * base_mod_y + cross * cross_mod_y;
    let (x_start, y_start) = match side {
        0 => (4, 10),
        1 => (0, 4),
        2 => (6, 0),
        3 => (10, 6),
        _ => unreachable!(),
    };
    (x_offset + x_start, y_offset + y_start)
}

fn get_grid_pos(norm_index: i32, side: usize) -> (i32, i32) {
    // TODO: Convert to static lookup tables
    let (cross, base) = {
        if norm_index < 5 {
            (0, norm_index)
        } else if norm_index == 9 {
            (4, 5)
        } else {
            (norm_index - 4, 4)
        }
    };
    another((base, cross), side)
}

fn get_grid_home_pos(index: usize, side: usize) -> (i32, i32) {
    another(((index + 1) as i32, -1), side)
}

fn grid_to_screen((x, y): (i32, i32)) -> (f32, f32) {
    (
        MARGIN + CELL_SIZE + x as f32 * (2.0 * CELL_SIZE + CELL_SPACE),
        MARGIN + CELL_SIZE + y as f32 * (2.0 * CELL_SIZE + CELL_SPACE),
    )
}

fn draw_cell(draw: &mut Draw, (x, y): (f32, f32), color: Color) {
    draw.circle(CELL_SIZE)
        .position(x, y)
        .color(color)
        .fill()
        .stroke(3.0)
        .stroke_color(Color::BLACK);
}

fn draw_peg(draw: &mut Draw, (x, y): (f32, f32), color: Color) {
    draw.circle(CELL_SIZE * 0.8)
        .position(x, y)
        .color(color)
        .fill()
        .stroke(2.0)
        .stroke_color(Color::BLACK);
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let board = state.game.board();
    let mut draw = gfx.create_draw();
    draw.clear(Color::SADDLE_BROWN);
    for (index, value) in board.cells().enumerate() {
        let norm_index = index % 10;
        let side = index / 10;
        let (x, y) = grid_to_screen(get_grid_pos(norm_index as i32, side));
        let cell_color = if norm_index == 0 {
            CELL_COLORS[side]
        } else {
            Color::WHITE
        };
        draw_cell(&mut draw, (x, y), cell_color);
        if *value > 0 {
            let peg_color = TEAM_COLORS[*value as usize - 1];
            draw_peg(&mut draw, (x, y), peg_color);
        }
        draw.text(&state.font, &format!("{}", index))
            .position(x, y)
            .h_align_center()
            .v_align_middle()
            .size(0.7 * CELL_SIZE)
            .color(Color::BLACK);
    }
    for player in state.game.players() {
        for (index, value) in board.home_cells(player).enumerate() {
            let (x, y) = grid_to_screen(get_grid_home_pos(index, player.index()));
            draw_cell(&mut draw, (x, y), CELL_COLORS[player.index()]);
            let index_char = ('A' as u8 + index as u8) as char;
            draw.text(&state.font, &format!("{}", index_char))
                .position(x, y)
                .h_align_center()
                .v_align_middle()
                .size(1.5 * CELL_SIZE)
                .color(Color::BLACK);
            if *value {
                draw_peg(&mut draw, (x, y), TEAM_COLORS[player.index()]);
            }
        }
    }
    draw.text(&state.font, &board.stats())
        .position(0.0, 0.0)
        .size(22.0)
        .color(Color::BLACK);
    gfx.render(&draw);
}
