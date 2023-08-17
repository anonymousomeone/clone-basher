use super::image::Image;

// locations of where we will scan for all the channels
// xywh (ofc)
// hardcoded (for now?!?!?)
const GREEN: [u8; 4] = [10, 10, 10, 10];
const RED: [u8; 4] = [10, 10, 10, 10];
const YELLOW: [u8; 4] = [10, 10, 10, 10];
const BLUE: [u8; 4] = [10, 10, 10, 10];
const ORANGE: [u8; 4] = [10, 10, 10, 10];

pub enum Channels {
    Green,
    Red,
    Yellow,
    Blue,
    Orange
}


pub fn should_press(area: Image, channel: Channels) -> bool {
    false
}

pub fn get_green() {

}