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
struct Streak {
    head_x: i32, // horizontal coord
    head_y: i32, // Bottom of the streak
    length: i32, // length of streak
    inner_text: ColorString,    
}

impl Streak {
    // Takes a queue of messages, consuming when needed
    fn new_with_queue(queue: &mut Vec<Message>, head_x: i32, length: i32, height: i32) -> Self {
	let mut rng = rand::thread_rng(); // TODO: fewer thread_rng()'s
	let mut inner_text = ColorString::with_capacity(height as usize); // prealloc
	let first_msg: ColorString = queue.pop().unwrap().into();
	let mut start: i32 = rng.gen_range(0, first_msg.len()+5) as i32 - first_msg.len() as i32;
	for i in 0..first_msg.len() as i32 {
	    if i+start >= 0 {
		inner_text[(i+start) as usize] = first_msg[i as usize];
	    }
	}
	start += first_msg.len() as i32;
	loop {
	    let next_msg: ColorString = queue.pop().unwrap().into();
	    start += rng.gen_range(1,5);
	    for i in 0..next_msg.len() as i32 {
		if i+start <= height {
		    inner_text[(i+start) as usize] = next_msg[i as usize];
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
    fn finished(&self, screen_height: i32) -> bool {
	self.head_y-self.length >= screen_height
    }
    fn render(&self) { // print contents to screen
	for i in (self.head_y-self.length)..self.head_y {
	    attron(self.inner_text[i as usize].attr);
	    mvaddch(i,self.head_x,self.inner_text[i as usize].data);
	    attroff(self.inner_text[i as usize].attr);
	}
    }
    fn advance(&mut self) {
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
struct Message {
    title: String, // the bolded part: prints first
    body: String,  // non-bolded part: prints second
    color: attr_t, // color of the message
}

impl Message {
    fn new_from_report(report: Report, color_good: attr_t, color_bad: attr_t, color_neutral: attr_t) -> Self {
	Message{title: "NYI".to_string(), body: " not yet implimented".to_string(), color: color_good}
    }
    fn len(&self) -> usize {
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
	    ret_str.push(ColorChar{data: message.body.as_bytes()[i] as u32, attr: message.color}); // TODO: add bold
	}
	ret_str
	
    }
}
