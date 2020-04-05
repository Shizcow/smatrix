/*
 * Main.rs
 *
 * Handles setup and TUI
 */
#![allow(unused)] // TODO: remove when code is good

mod scene;
use crate::scene::{Message, Scene};
mod requests;
use crate::requests::request_tickers;
use crate::requests::get_sp500_tickers;
use crate::requests::Report;

use rand::prelude::*;
use std::{thread, time};
use ncurses::*;

static COLOR_PAIR_BACKGROUND: i16 = 1;
static COLOR_PAIR_GREEN     : i16 = 2;
static COLOR_PAIR_RED       : i16 = 3;
static COLOR_PAIR_NORMAL    : i16 = 4;

/*
struct Streak {
    title: String, // title is printed differently
    contents: String,
    x_head: i32,
    y_head: i32,
    color: attr_t
}

impl Streak {
    //                                                   GREEN   RED     NEUTRAL
    fn new_from_report(report: &Report, x: i32, colors: (attr_t, attr_t, attr_t)) -> Self {
	let color = if report.change > 0.0 {colors.0} else if report.change < 0.0 {colors.1} else {colors.2};
	let contents = format!("{:.2}$ {:.2}%", report.change.abs(), &report.change_percent.abs());
	Self{y_head: 0-(contents.len()+report.ticker.len()) as i32, x_head: x, contents: contents, color: color, title: report.ticker.clone()+" "}
    }
    fn print(&self) {
	attron(A_BOLD());
	vprintw(self.y_head, self.x_head, &self.title, self.color);
	attroff(A_BOLD());
	vprintw(self.y_head+self.title.len() as i32, self.x_head, &self.contents, self.color);
    }
    // move down and print, returns false if off the bottom of screen
    fn update(&mut self){
	attron(COLOR_PAIR(COLOR_PAIR_BACKGROUND));
	mvaddch(self.y_head,self.x_head,' ' as u32);
	attroff(COLOR_PAIR(COLOR_PAIR_BACKGROUND));
	self.y_head+=1;
	self.print();
    }
    fn finished(&self, screen_height: i32) -> bool {
	self.y_head >= screen_height
    }
}
 */

// Things to change: the stock prices don't move, just the lights do
// Padding on top and bottom (configurable?) 
// Add modes: rain mode?


fn main() {
    let mut rng = rand::thread_rng();
    
    // sstart ncurses
    initscr();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    
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

    

    let mut scene = Scene::new(screen_width, screen_height, 15, COLOR_PAIR(COLOR_PAIR_NORMAL), true);
    for report in reports {
	scene.push(Message::new_from_report(report, COLOR_PAIR(COLOR_PAIR_GREEN), COLOR_PAIR(COLOR_PAIR_RED), COLOR_PAIR(COLOR_PAIR_NORMAL)));
    }

    loop {
	scene.advance();
	refresh();
	thread::sleep(time::Duration::from_millis(100));
    }

    endwin();
}

fn vprintw(mut y: i32, x: i32, string: &str, attr: attr_t) {
    attron(attr);
    for c in string.chars() {
	mvaddch(y,x,c as u32);
	y+=1;
    }
    attroff(attr);
}
