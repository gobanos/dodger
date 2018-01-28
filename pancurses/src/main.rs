extern crate libdodger;
extern crate pancurses;
extern crate time;

use pancurses::{initscr, endwin, Input, noecho, resize_term, Window, set_blink};
use libdodger::{Game, Player, Baddie};
use std::time::Duration;
use std::thread;

fn main() {
    let window = initscr();
    window.refresh();
    window.keypad(true);
    window.nodelay(true);

    set_blink(true);
    noecho();

    let (height, width) = window.get_max_yx();

    let mut game = Game::new(width as u32, height as u32);

    loop {
        let start_frame = time::precise_time_s();

        render(&game, &window);

        let action = match window.getch() {
            Some(Input::KeyDC) => break,
            Some(Input::KeyResize) => {
                resize_term(0, 0);
                continue;
            }
            Some(Input::KeyLeft) => { -1 }
            Some(Input::KeyRight) => { 1 }
            _ => { 0 }
        };

        game.turn(action);

        let remaining_time = (start_frame + 1.0 / 60.0) - time::precise_time_s();
        thread::sleep(Duration::from_millis((remaining_time * 1_000.0) as u64));
    }

    endwin();
}

fn render(game: &Game, window: &Window) {
    window.clear();

    draw_player(&game.player, &window);

    for baddie in &game.baddies {
        draw_baddie(baddie, &window);
    }

    let (height, width) = window.get_max_yx();

    window.mvaddstr(0, 0, &format!("{}x{}", width, height));
    window.mvaddstr(1, 0, &format!("{} baddies alive", game.baddies.len()));
}

fn draw_player(player: &Player, window: &Window) {
    let pos = &player.body.pos;
    let rect = &player.body.rect;

    for x in pos.x..pos.x + rect.width as i32 {
        for y in pos.y..pos.y + rect.height as i32 {
            window.mvaddch(window.get_max_y() - y, x, 'O');
        }
    }
    window.mvaddch(window.get_max_y() - pos.y, pos.x, '0');
}

fn draw_baddie(baddie: &Baddie, window: &Window) {
    let pos = &baddie.body.pos;
    let rect = &baddie.body.rect;

    for x in pos.x..pos.x + rect.width as i32 {
        for y in pos.y..pos.y + rect.height as i32 {
            window.mvaddch(window.get_max_y() - y, x, 'X');
        }
    }
    window.mvaddch(window.get_max_y() - pos.y, pos.x, '0');
}