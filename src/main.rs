/*
 * Main.rs
 *
 * Handles setup and TUI
 */

mod scene;
use crate::scene::{Message, Scene};
mod requests;
use crate::requests::{get_sp500_tickers, request_tickers};

use rand::prelude::*;
use std::{thread, time};
use ncurses::*;

static COLOR_PAIR_BACKGROUND: i16 = 1;
static COLOR_PAIR_GREEN     : i16 = 2;
static COLOR_PAIR_RED       : i16 = 3;
static COLOR_PAIR_NORMAL    : i16 = 4;


fn main() {
    let mut rng = rand::thread_rng();
    
    // start ncurses
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    timeout(0);
    
    let mut screen_height : i32 = 0;
    let mut screen_width  : i32 = 0;
    getmaxyx(stdscr(), &mut screen_height, &mut screen_width);

    // Init color, set background to black
    start_color();
    init_pair(COLOR_PAIR_BACKGROUND, COLOR_BLACK, COLOR_BLACK);
    init_pair(COLOR_PAIR_GREEN,      COLOR_GREEN, COLOR_BLACK);
    init_pair(COLOR_PAIR_RED,        COLOR_RED,   COLOR_BLACK);
    init_pair(COLOR_PAIR_NORMAL,     COLOR_WHITE, COLOR_BLACK);

    bkgd(' ' as chtype | COLOR_PAIR(COLOR_PAIR_BACKGROUND) as chtype); // fill background



    
    let mut reports; // TODO: connecting to the matrix art
    {
	attron(COLOR_PAIR(COLOR_PAIR_NORMAL));
	mvprintw(0, 0, "Downloading S&P500 ticker symbols...");
	refresh();
	let sp = get_sp500_tickers();
	mvprintw(0, 0, "Downloading stock prices...         ");
	refresh();
	reports = request_tickers(&sp);
	mvprintw(0, 0, "Shuffling ticker symbols...         ");
	refresh();
	reports.shuffle(&mut rng);
	mvprintw(0, 0, "                                    ");
	refresh();
	attroff(COLOR_PAIR(COLOR_PAIR_NORMAL));
    }

    

    let mut scene = Scene::new(screen_width, screen_height, 15, COLOR_PAIR(COLOR_PAIR_BACKGROUND), true);
    for report in reports {
	scene.push(Message::new_from_report(report, COLOR_PAIR(COLOR_PAIR_GREEN), COLOR_PAIR(COLOR_PAIR_RED), COLOR_PAIR(COLOR_PAIR_NORMAL)));
    }

    while getch() != 113 {
	scene.advance();
	refresh();
	thread::sleep(time::Duration::from_millis(100));
    }

    endwin();
}
