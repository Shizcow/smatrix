/*
 * Main.rs
 *
 * Handles setup and TUI
 */

use rand::prelude::*;
use std::{thread, time};
use ncurses::*;

static COLOR_PAIR_BACKGROUND: i16 = 1;
static COLOR_PAIR_GREEN     : i16 = 2;
static COLOR_PAIR_RED       : i16 = 3;

struct Streak {
    contents: String,
    x_head: i32,
    y_head: i32,
    color: attr_t
}

impl Streak {
    fn new(contents: String, x: i32, color: attr_t) -> Self {
	Self{y_head: 0-contents.len() as i32, x_head: x, contents: contents, color: color}
    }
    fn print(&self) {
	vprintw(self.y_head, self.x_head, &self.contents, self.color);
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

fn main() {
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
    bkgd(' ' as chtype | COLOR_PAIR(COLOR_PAIR_BACKGROUND) as chtype); // fill background

    let mut rng = rand::thread_rng();
    let mut streaks: Vec<Streak> = Vec::new();
    loop {
	for _ in 0..3 { // try to spawn
	    let target_x = rng.gen_range(0, screen_width-1);
	    let mut any = false;
	    for i in 0..streaks.len() { // couldn't figure out Iterator::any
		if streaks[i].x_head==target_x {
		    any = true;
		    break;
		}
	    }
	    if !any {
		streaks.push(Streak::new("1234567890".to_string(), target_x, COLOR_PAIR(COLOR_PAIR_RED)));
	    }
	}
	for streak in &mut streaks {
	    streak.update();
	}
	streaks.retain(|streak|!streak.finished(screen_height));
	refresh();
	thread::sleep(time::Duration::from_millis(100));
    }    

    //endwin();
}

fn vprintw(mut y: i32, x: i32, string: &str, attr: attr_t) {
    attron(attr);
    for c in string.chars() {
	mvaddch(y,x,c as u32);
	y+=1;
    }
    attroff(attr);
}
