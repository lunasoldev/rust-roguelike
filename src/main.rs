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
        
        // Draw the main area
        pencil.set_foreground(Color::DarkGrey);
        draw_filled_rect(&mut pencil, '·', top_left, Vec2::xy(65, 33));
        pencil.set_foreground(Color::White);
        pencil.draw_rect(&RectCharset::double_lines(), top_left, Vec2::xy(65, 33));
        
        // Draw the leveling UI
        let level_info = format!("LVL:{} XP:{}/{}", player.lvl(), player.xp(), player.xp_needed());

        let level_x = (term_size.x.saturating_sub(level_info.len() as i32) / 2) as u16;

        pencil.draw_text(&level_info as &str, Vec2::xy(level_x, top_left.y - 1));

        // Draw Stats on the side
        let stats_x = (top_left.x / 2) as u16;
        let sidebar_width = top_left.x / 2;

        let hp_text = &format!("HP: {}/{}", player.hp(), player.max_hp()) as &str;

        let str_text = &format!("STR: {}", player.str()) as &str;
        let dex_text = &format!("DEX: {}", player.dex()) as &str;
        let atk_text = &format!("ATK: {}", player.atk()) as &str;
        let def_text = &format!("DEF: {}", player.def()) as &str;

        let stat_entries: Vec<&str> = vec![
            hp_text,
            "" as &str,
            str_text, 
            dex_text,
            "" as &str,
            atk_text, 
            def_text,
        ];

        draw_ui_box(&mut pencil, "STATS" as &str, Vec2::xy(stats_x, top_left.y), Vec2::xy(sidebar_width, 33), &stat_entries);

        // TODO Draw Inventory on the side using draw_ui_box()
        let inv_x = (top_left.x + 65) as u16;
        pencil.draw_rect(&RectCharset::simple_lines(), Vec2::xy(inv_x, top_left.y), Vec2::xy(sidebar_width, 33));

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

fn draw_ui_box(pencil: &mut Pencil, title: &str, pos: Vec2, size: Vec2, entries: &Vec<&str>) {

    pencil.draw_rect(&RectCharset::simple_lines(), pos, size);
    pencil.set_origin(pos);

    pencil.draw_text(title, Vec2::xy(size.x.saturating_sub(title.len() as i32) / 2, 1));

    pencil.draw_hline('-', Vec2::xy(1, 2), size.x - 2);
    
    let mut i = 4;
    for entry in entries.iter() {
        pencil.draw_text(entry, Vec2::xy(size.x.saturating_sub(entry.len() as i32) / 2, i));
        i += 1;
    }

    pencil.set_origin(Vec2::zero());
}
