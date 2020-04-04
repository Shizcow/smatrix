use ncurses::attr_t;
use crate::requests::Report;

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
    message_holders: Vec<MessageHolder>, // all the messages in this streak
}

// MessageHolder struct
// Holds a message and it's location, translating to only nessicary data
// No movement, just location
struct MessageHolder {
    message: Message, // contents
    head_pos: i32, // position of the head of the message
}

impl MessageHolder {
    fn new_from_message(message: Message, head_pos: i32) -> Self {
	MessageHolder{message, head_pos}
    }
    fn len(&self) -> usize {
	self.message.len()
    }
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
