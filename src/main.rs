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

static COLOR_PAIR_GREEN     : i16 = 1;
static COLOR_PAIR_RED       : i16 = 2;
static COLOR_PAIR_NORMAL    : i16 = 3;

fn main() {
    let window = initscr();
    curs_set(0);
    noecho();
    
    let mut reports; // TODO: connecting to the matrix art
    let sp; // keep these around to update later
    {
	window.attron(COLOR_PAIR(COLOR_PAIR_NORMAL as u32));
	window.mvprintw(0, 0, "Downloading S&P500 ticker symbols...");
	window.refresh();
	sp = get_sp500_tickers();
	window.mvprintw(0, 0, "Downloading stock prices...         ");
	window.refresh();
	reports = request_tickers(&sp);
	window.mvprintw(0, 0, "Shuffling ticker symbols...         ");
	window.refresh();
	reports.shuffle(&mut rand::thread_rng());
	window.clear();
	window.refresh();
	window.attroff(COLOR_PAIR(COLOR_PAIR_NORMAL as u32));
    }

    endwin();

    let mut scene = Scene::new(15, COLOR_BLACK, true, Duration::from_millis(20));
    scene.init_pair(COLOR_PAIR_GREEN,      COLOR_GREEN, COLOR_BLACK);
    scene.init_pair(COLOR_PAIR_RED,        COLOR_RED,   COLOR_BLACK);
    scene.init_pair(COLOR_PAIR_NORMAL,     COLOR_WHITE, COLOR_BLACK);
    scene.append(reports.into_iter().map(|report| report.to_message(COLOR_PAIR_GREEN, COLOR_PAIR_RED, COLOR_PAIR_NORMAL)).collect());

    
    scene.start();


    let time = Instant::now();
    while scene.alive() {
	if time.elapsed() > Duration::from_millis(1000*30) {
	    reports = request_tickers(&sp);
	    scene.append_update(reports.into_iter().map(|report| report.to_message(COLOR_PAIR_GREEN, COLOR_PAIR_RED, COLOR_PAIR_NORMAL)).collect());
	}
    }
}
