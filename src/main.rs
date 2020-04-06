/*
 * Main.rs
 *
 * Handles setup and TUI
 */

mod requests;
use crate::requests::*;

use matrixise::*;

use rand::seq::SliceRandom;

use ncurses::{COLOR_PAIR, init_pair, attron, attroff, refresh, mvprintw, chtype, bkgd};
use ncurses::constants::*;
static COLOR_PAIR_BACKGROUND: i16 = 1;
static COLOR_PAIR_GREEN     : i16 = 2;
static COLOR_PAIR_RED       : i16 = 3;
static COLOR_PAIR_NORMAL    : i16 = 4;

fn main() {
    matrixise::init(); // start ncurses
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
	reports.shuffle(&mut rand::thread_rng());
	mvprintw(0, 0, "                                    ");
	refresh();
	attroff(COLOR_PAIR(COLOR_PAIR_NORMAL));
    }

    let mut scene = Scene::new(15, COLOR_BLACK, true);

    // push a few reports, then start. Reduces startup lag
    if reports.len() > 20 {
	for _ in 0..20 {
	    scene.push(reports.pop().unwrap().to_message(COLOR_PAIR(COLOR_PAIR_GREEN), COLOR_PAIR(COLOR_PAIR_RED), COLOR_PAIR(COLOR_PAIR_NORMAL)));
	}
    } else {
	while reports.len() > 0 {
	    scene.push(reports.pop().unwrap().to_message(COLOR_PAIR(COLOR_PAIR_GREEN), COLOR_PAIR(COLOR_PAIR_RED), COLOR_PAIR(COLOR_PAIR_NORMAL)));
	}
    }
    scene.start();
    while reports.len() > 0 {
	scene.push(reports.pop().unwrap().to_message(COLOR_PAIR(COLOR_PAIR_GREEN), COLOR_PAIR(COLOR_PAIR_RED), COLOR_PAIR(COLOR_PAIR_NORMAL)));
    }
    

    scene.join();
}
