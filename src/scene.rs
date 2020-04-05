use ncurses::{attron, attroff, mvaddch};
use ncurses::{attr_t, A_BOLD};
use crate::requests::Report;
use std::collections::VecDeque;

extern crate rand;
use rand::Rng;

struct Column {
    streaks: Vec<Streak>
}

impl Column {
    fn new() -> Self {
	Self{streaks: Vec::new()}
    }
}

pub struct MessageQueue {
    data: VecDeque<Message>,
    closed: bool,       // recycle data?
}

impl MessageQueue {
    fn new(capacity: usize, closed: bool) -> Self {
	Self{data: VecDeque::with_capacity(capacity), closed}
    }
    fn pop(&mut self) -> Option<Message> {
	self.data.pop_front()
    }
    fn push(&mut self, message: Message) {
	self.data.push_back(message);
    }
    fn recycle(&mut self, message: Message) {
	if self.closed { // recycle to back of queue
	    self.data.push_back(message);
	}
    }
}

// Scene struct
// Holds all data for the scene, including:
//   Streaks (light things up and hold messages)
//   Dimensions
//   MessageQueue (messages yet to be printed)
// Handles updating, can render

pub struct Scene {
    columns:     Vec<Column>,  // holds all streaks in the scene
    height:      i32,          // height of the scene
    queue:       MessageQueue, // Messages yet to be printed
    background:  attr_t,       // used for derendering
    max_padding: i32,
}

impl Scene {
    pub fn new(width: i32, height: i32, max_padding: i32, background: attr_t, is_closed: bool) -> Self {
	let mut columns = Vec::new();
	for _ in 0..width {
	    columns.push(Column::new());
	}
	Self{columns, height, queue: MessageQueue::new(width as usize, is_closed), background, max_padding}
    }
    pub fn push(&mut self, message: Message){
	self.queue.push(message);
    }
    pub fn advance(&mut self){ // move all streaks, clean up dead ones, try to spawn new ones
	let mut rng = rand::thread_rng(); // TODO: fewer thread_rng()'s
	for (i, column) in self.columns.iter_mut().enumerate() {
	    for streak in &mut column.streaks { // advance all
		streak.derender(self.background);
		streak.advance();
	    }
	    let height = self.height; // always fighting with the borrow checker
	    column.streaks.retain(|streak| !streak.finished(height)); // clean up dead streaks

	    // now, try to spawn new streaks
	    if column.streaks.len()==0 || column.streaks.iter().all(|streak| streak.top_space() > 5) { // there's need to
		column.streaks.push(Streak::new_with_queue(&mut self.queue, i as i32, rng.gen_range(5, self.height*2), self.height, self.max_padding)); // add new streak, consuming from queue
		// TODO: better length requirements
	    }

	    
	    for streak in &mut column.streaks { // advance all
		streak.render(self.height);
	    }
	}
    }
}


// Streak struct
// Holds a streak's location&length
// Handles streak movement
// Can render all characters in a streak
pub struct Streak {
    head_x: i32, // horizontal coord
    head_y: i32, // Bottom of the streak
    length: i32, // length of streak
    inner_text: ColorString,    
}

impl Streak {
    // Takes a queue of messages, consuming when needed
    pub fn new_with_queue(queue: &mut MessageQueue, head_x: i32, length: i32, screen_height: i32, max_padding: i32) -> Self {
	let mut rng = rand::thread_rng(); // TODO: fewer thread_rng()'s
	let mut inner_text = ColorString::with_capacity(screen_height as usize); // prealloc
	let first_msg_check = queue.pop();
	if let None = first_msg_check {
	    for _ in 0..screen_height {
		inner_text.push(ColorChar{data: ' ' as u32, attr: 0}); // pad out top if required
	    }
	    return Streak{head_x, head_y: 0, length, inner_text}; // nothing to do!
	}
	let first_msg = first_msg_check.unwrap();
	{
	    let first_string: ColorString = (&first_msg).into();
	    let mut start: i32 = rng.gen_range(0, first_string.len()+max_padding as usize) as i32 - first_string.len() as i32 + 1; // make sure there's at least one char printed, space up to max_padding is allowed at top
	    if start > screen_height {
		start = screen_height; // don't overflow
	    }
	    if start > 0 {
		for _ in 0..start {
		    inner_text.push(ColorChar{data: ' ' as u32, attr: 0}); // pad out top if required
		}
	    }
	    for i in (
		if start < 0 {
		    -start // cut off relevant portion of message if required
		} else {
		    0
		}
	    )..(screen_height.min(first_string.len() as i32)) {
		inner_text.push(first_string[i as usize]);
		if inner_text.len() as i32 >= screen_height {
		    queue.recycle(first_msg);
		    return Streak{head_x, head_y: 0, length, inner_text}; // if first message is too long
		}
	    }
	}
	queue.recycle(first_msg);
	loop {
	    let r: i32 = if max_padding > 0 {
		rng.gen_range(1,max_padding)
	    } else {
		0 // if padding is forced to 0, never pad ever
	    };
	    if inner_text.len() as i32+r >= screen_height { // terminate early
		for _ in 0..(screen_height as usize-inner_text.len()) {
		    inner_text.push(ColorChar{data: ' ' as u32, attr: 0}); // fill remaining
		}
		break; // streak is full
	    } else { // still need more content to fill
		for _ in 0..r {
		    inner_text.push(ColorChar{data: ' ' as u32, attr: 0});
		}
	    }

	    
	    let next_msg_check = queue.pop();
	    if let None = next_msg_check {
		for _ in inner_text.len() as i32..screen_height {
		    inner_text.push(ColorChar{data: ' ' as u32, attr: 0}); // pad out top if required
		}
		return Streak{head_x, head_y: 0, length, inner_text}; // nothing to do!
	    }
	    let next_msg = next_msg_check.unwrap();
	    {
		let next_string: ColorString = (&next_msg).into();
		
		if inner_text.len()+next_string.len() >= screen_height as usize { // terminate early
		    for i in 0..(screen_height as usize-inner_text.len()) {
			inner_text.push(next_string[i as usize]); // fill remaining
		    }
		    queue.recycle(next_msg);
		    break; // streak is full
		} else {
		    for i in 0..next_string.len() {
			inner_text.push(next_string[i as usize]); // print full string, move on
		    }
		}
	    }
	    queue.recycle(next_msg);
	}
	Streak{head_x, head_y: 0, length, inner_text}
    }
    pub fn render(&self, screen_height: i32) { // print contents to screen
	for i in (self.head_y-self.length-1)..self.head_y {
	    if i >= 0 && i < screen_height {
		attron(self.inner_text[i as usize].attr);
		mvaddch(i,self.head_x,self.inner_text[i as usize].data);
		attroff(self.inner_text[i as usize].attr);
	    }
	}
    }
    pub fn derender(&self, attr: attr_t) { // removes first char, makes streak look like it's moving down
	attron(attr);
	mvaddch(self.head_y-self.length-1, self.head_x, ' ' as u32);
	attron(attr);
    }
    pub fn advance(&mut self) {
	self.head_y+=1;
    }
    pub fn finished(&self, screen_height: i32) -> bool { // can this streak be safely deleted?
	self.head_y-self.length >= screen_height
    }
    pub fn top_space(&self) -> i32 { // how much unallocated space at the top of the screen?
	self.head_y-self.length+1
    }
}

type ColorString = Vec<ColorChar>;

#[derive(Copy, Clone)]
struct ColorChar {
    data: u32,
    attr: attr_t
}

// Message struct
// Holds basic info about a message
pub struct Message {
    title: String, // the bolded part: prints first
    body: String,  // non-bolded part: prints second
    color: attr_t, // color of the message
}

impl Message {
    pub fn new(title: String, body: String, color: attr_t) -> Self {
	Self{title, body, color}
    }
    pub fn new_from_report(report: Report, color_good: attr_t, color_bad: attr_t, color_neutral: attr_t) -> Self {
	Self{title: report.ticker, body: format!("{:.2}$ {:.2}%", report.change.abs(), &report.change_percent.abs()), color: if report.change > 0.0 {color_good} else if report.change < 0.0 {color_bad} else {color_neutral}}
    }
    pub fn len(&self) -> usize {
	self.title.len()+self.body.len()
    }
}

impl From<&Message> for ColorString {
    fn from(message: &Message) -> ColorString {
	let mut ret_str = ColorString::with_capacity(message.len());
	for i in 0..message.title.len() {
	    ret_str.push(ColorChar{data: message.title.as_bytes()[i] as u32, attr: message.color | A_BOLD()});
	}
	for i in 0..message.body.len() {
	    ret_str.push(ColorChar{data: message.body.as_bytes()[i] as u32, attr: message.color});
	}
	ret_str
    }
}
