use ncurses::attr_t;
use ncurses::attron;
use ncurses::attroff;
use ncurses::mvaddch;
use crate::requests::Report;

extern crate rand;
use rand::Rng;

// Scene struct
// Holds all data for the scene, including:
//   Streaks (light things up and hold messages)
//   Dimensions
//   MessageQueue (messages yet to be printed)
// Handles updating, can render

struct Scene {
    streaks: Vec<Streak>,  // holds all streaks in the scene
    width:   i32,          // width of the scene
    height:  i32,          // height of the scene
    queue:   Vec<Message>, // Messages yet to be printed
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
    pub fn new_with_queue(queue: &mut Vec<Message>, head_x: i32, length: i32, height: i32) -> Self {
	let mut rng = rand::thread_rng(); // TODO: fewer thread_rng()'s
	let mut inner_text = ColorString::with_capacity(height as usize); // prealloc
	let first_msg: ColorString = queue.pop().unwrap().into();
	let mut start: i32 = rng.gen_range(0, first_msg.len()+5) as i32 - first_msg.len() as i32; // TODO: make sure msg is long enough
	if start > 0 {
	    for _ in 0..start {
		// pad out top if required
		inner_text.push(ColorChar{data: ' ' as u32, attr: 0});
	    }
	}
	for i in (
	    if start < 0 {
		-start // cut off relevant portion of message if required
	    } else {
		0
	    }
	)..first_msg.len() as i32 {
	    inner_text.push(first_msg[i as usize]);
	}
	start += first_msg.len() as i32;
	loop {
	    let next_msg: ColorString = queue.pop().unwrap().into();
	    let r = rng.gen_range(1,5);
	    for _ in 0..r {
		inner_text.push(ColorChar{data: ' ' as u32, attr: 0});
	    }
	    start += r;
	    for i in 0..next_msg.len() as i32 {
		if i+start <= height {
		    inner_text.push(next_msg[i as usize]);
		} else {
		    break;
		}
	    }
	    start += next_msg.len() as i32;
	    if start >= height {
		break;
	    }
	}
	Streak{head_x, head_y: 0, length, inner_text}
    }
    pub fn finished(&self, screen_height: i32) -> bool {
	self.head_y-self.length >= screen_height
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
	Self{title: "NYI".to_string(), body: " not yet implimented".to_string(), color: color_good}
    }
    pub fn len(&self) -> usize {
	self.title.len()+self.body.len()
    }
}

impl From<Message> for ColorString {
    fn from(message: Message) -> ColorString {
	let mut ret_str = ColorString::with_capacity(message.len());
	for i in 0..message.title.len() {
	    ret_str.push(ColorChar{data: message.title.as_bytes()[i] as u32, attr: message.color}); // TODO: add bold
	}
	for i in 0..message.body.len() {
	    ret_str.push(ColorChar{data: message.body.as_bytes()[i] as u32, attr: message.color});
	}
	ret_str
	
    }
}
