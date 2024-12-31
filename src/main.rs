mod player;

use crate::player::Player;

use ruscii::app::{App, Config, State};
use ruscii::terminal::{Window, Color};
use ruscii::drawing::{Pencil, RectCharset};
use ruscii::keyboard::{KeyEvent, Key};
use ruscii::spatial::{Vec2};

fn main() {
    let config: Config = Config::new()
        .fps(30);
    let mut app = App::config(config);
    
    let mut player = Player::new();

    app.run(|app_state: &mut State, window: &mut Window| {
        let term_size = window.size();
        let square_size = Vec2::xy(64, 32);

        let top_left = Vec2::xy(
            (term_size.x / 2).saturating_sub(square_size.x / 2),
            (term_size.y / 2).saturating_sub(square_size.y / 2),
        );

        for key_event in app_state.keyboard().last_key_events() {
            match key_event {
                KeyEvent::Pressed(Key::Esc) => app_state.stop(),
                KeyEvent::Pressed(Key::W) => player.move_y(-1, 30),
                KeyEvent::Pressed(Key::S) => player.move_y(1, 30),
                KeyEvent::Pressed(Key::A) => player.move_x(-1, 62),
                KeyEvent::Pressed(Key::D) => player.move_x(1, 62),
                _ => (),
            }
        }
        
        // RENDERING
        let mut pencil = Pencil::new(window.canvas_mut());

        draw_filled_rect(&mut pencil, '.', top_left, Vec2::xy(65, 33));

        pencil.draw_rect(&RectCharset::double_lines(), top_left, Vec2::xy(65, 33));
        
        //draw_background(&mut pencil);
        
        let player_pos = top_left + Vec2::xy(player.pos[0], player.pos[1]) + Vec2::xy(1, 1);

        pencil.draw_text("☺", player_pos);
    });
}

fn draw_filled_rect(pencil: &mut Pencil, fill: char, position: Vec2, dimension: Vec2) {
    pencil.move_origin(position);
    for i in 0..dimension.x {
        pencil.draw_vline(fill, Vec2::xy(i, 0), dimension.y);
    }
    pencil.move_origin(-position);
}
