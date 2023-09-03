mod game;

use game::Game;
use notan::draw::*;
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(State::new)
        .draw(draw)
        .update(update)
        .add_config(DrawConfig)
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

const STEP: f32 = 42.0;
const CELL_SIZE: f32 = 16.0;
const MARGIN: f32 = 12.0;
const TEAM_COLORS: [Color; 4] = [Color::BLUE, Color::YELLOW, Color::GREEN, Color::RED];
const CELL_COLORS: [Color; 4] = [
    Color::new(0.3, 0.3, 1.0, 1.0),
    Color::new(1.0, 1.0, 0.3, 1.0),
    Color::new(0.3, 1.0, 0.3, 1.0),
    Color::new(1.0, 0.3, 0.3, 1.0),
];

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::SADDLE_BROWN);
    for (index, value) in state.game.board().cells().enumerate() {
        let norm_index = index % 10;
        let (cross, base) = {
            if norm_index < 5 {
                (0.0, norm_index as f32 * STEP)
            } else if norm_index == 9 {
                (4.0 * STEP, 5.0 * STEP)
            } else {
                ((norm_index - 4) as f32 * STEP, 4.0 * STEP)
            }
        };
        let side = index / 10;
        // directions: 0 = north, 1 = east, 2 = south, 3 = west
        let (base_mod, cross_mod) = match side {
            0 => (0, 3),
            1 => (1, 0),
            2 => (2, 1),
            3 => (3, 2),
            _ => unreachable!(),
        };
        let (base_mod_x, base_mod_y) = match base_mod {
            0 => (0.0, -1.0),
            1 => (1.0, 0.0),
            2 => (0.0, 1.0),
            3 => (-1.0, 0.0),
            _ => unreachable!(),
        };
        let (cross_mod_x, cross_mod_y) = match cross_mod {
            0 => (0.0, -1.0),
            1 => (1.0, 0.0),
            2 => (0.0, 1.0),
            3 => (-1.0, 0.0),
            _ => unreachable!(),
        };
        let x_offset = base * base_mod_x + cross * cross_mod_x;
        let y_offset = base * base_mod_y + cross * cross_mod_y;
        let (x_start, y_start) = match side {
            0 => (4.0 * STEP, 10.0 * STEP),
            1 => (0.0, 4.0 * STEP),
            2 => (6.0 * STEP, 0.0),
            3 => (10.0 * STEP, 6.0 * STEP),
            _ => unreachable!(),
        };

        let (x, y) = (
            x_offset + x_start + CELL_SIZE + MARGIN,
            y_offset + y_start + CELL_SIZE + MARGIN,
        );
        let cell_color = if norm_index == 0 {
            CELL_COLORS[side as usize]
        } else {
            Color::WHITE
        };
        draw.circle(CELL_SIZE)
            .position(x, y)
            .color(cell_color)
            .fill()
            .stroke(3.0)
            .stroke_color(Color::BLACK);
        if *value > 0 {
            let color = TEAM_COLORS[*value as usize - 1];
            draw.circle(CELL_SIZE * 0.8)
                .position(x, y)
                .color(color)
                .fill()
                .stroke(2.0)
                .stroke_color(Color::BLACK);
        }
        draw.text(&state.font, &format!("{}", index))
            .position(x, y)
            .h_align_center()
            .v_align_middle()
            .size(0.7 * CELL_SIZE)
            .color(Color::BLACK);
    }
    gfx.render(&draw);
}
