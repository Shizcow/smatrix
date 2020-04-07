/*
 * Main.rs
 *
 * Handles setup and TUI
 */

mod requests;
use crate::requests::*;

use matrixise::*;

use rand::seq::SliceRandom;

use std::time::{Duration, Instant};

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
    let sp; // keep these around to update later
    {
	attron(COLOR_PAIR(COLOR_PAIR_NORMAL));
	mvprintw(0, 0, "Downloading S&P500 ticker symbols...");
	refresh();
	sp = get_sp500_tickers();
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

    let mut scene = Scene::new(15, COLOR_BLACK, true, Duration::from_millis(20));
    scene.append(reports.into_iter().map(|report| report.to_message(COLOR_PAIR(COLOR_PAIR_GREEN), COLOR_PAIR(COLOR_PAIR_RED), COLOR_PAIR(COLOR_PAIR_NORMAL))).collect());

    
    scene.start();


    let time = Instant::now();
    while scene.alive() {
	if time.elapsed() > Duration::from_millis(1000*30) {
	    reports = request_tickers(&sp);
	    scene.append_update(reports.into_iter().map(|report| report.to_message(COLOR_PAIR(COLOR_PAIR_GREEN), COLOR_PAIR(COLOR_PAIR_RED), COLOR_PAIR(COLOR_PAIR_NORMAL))).collect());
	}
    }
}
