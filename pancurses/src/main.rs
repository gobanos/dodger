extern crate libdodger;
extern crate pancurses;

use pancurses::{initscr, endwin, Input, noecho, resize_term, Window, set_blink};
use libdodger::{Game, Player, Baddie};

fn main() {
    let window = initscr();
    window.refresh();
    window.keypad(true);

    set_blink(true);
    noecho();

    let (height, width) = window.get_max_yx();

    let mut game = Game::new(width as u32, height as u32);

    loop {
        render(&game, &window);

        match window.getch() {
            Some(Input::KeyDC) => break,
            Some(Input::KeyResize) => { resize_term(0, 0); }
            Some(Input::KeyLeft) => { game.turn(-1); }
            Some(Input::KeyRight) => { game.turn(1); }
            Some(_) => { game.turn(0); },
            None => { game.turn(0); },
        }
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