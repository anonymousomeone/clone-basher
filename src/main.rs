use std::path::Path;
use std::time::Duration;
use std::thread;

use rusty_duplication::capturer::model::Capturer;
use rusty_duplication::manager::Manager;
use rusty_duplication::utils::FrameInfoExt;

extern crate image;

use crate::modules::hook;
use crate::modules::image::Image;
use crate::modules::keyboard::press_and_release;
use crate::modules::Keys;

mod modules;

fn main() {
    println!("Hello, world!");
    let manager = Manager::default().unwrap();
    
    let mut capturer = manager.contexts[0].simple_capturer().unwrap();
    
    thread::sleep(Duration::from_millis(100));
    
    let mut image = Image::default();
    let mut image_num = 0;
    
    loop {
        let result = capturer.safe_capture();
        
        let info = match result {
            Ok(v) => v,
            Err(_e) => {
                thread::sleep(Duration::from_millis(20));
                continue;
            }
        };
        
        if !info.desktop_updated() {
            thread::sleep(Duration::from_millis(20));
            continue;
        }
        
        
        // caught in 4k 📸📸📸📸
        let data: &[u8] = capturer.buffer();
        //  ^^^^ BGRA8
        
        image.put_data(data.to_vec());
        image.update_width(1920);
        
        // crop to ch window
        let mut rect;
        
        unsafe {
            rect = hook::get_ch_coords();
        }
        
        // windows fuckery
        rect.left += 8;
        rect.top += 8;
        rect.right -= 8;
        rect.bottom -= 8;

        let width: u16 = (rect.right - rect.left).try_into().unwrap() ;
        let height: u16 = (rect.bottom - rect.top).try_into().unwrap();

        image.crop(
            rect.left.try_into().unwrap(),
            rect.top.try_into().unwrap(),
            width,
            height
        );

        // -- 


        let mut ndata: Vec<u8> = vec![0; image.get_data().len()];

        convert_between_bgra_and_rgba(&image.get_data(), width, height, &mut ndata);
        
        let name = String::from("./images/image") + &image_num.to_string() + ".png";

        image::save_buffer(&Path::new(&name), &ndata, width as u32, height as u32, image::ColorType::Rgba8).unwrap();
        // break;

        image_num += 1;
    }

}

fn convert_between_bgra_and_rgba(input: &[u8], pixel_width: u16, pixel_height: u16, output: &mut [u8]) {
    let mut offset = 0;

    for _ in 0..pixel_height {
        for _ in 0..pixel_width {
            output[offset] = input[offset + 2];
            output[offset + 1] = input[offset + 1];
            output[offset + 2] = input[offset];
            output[offset + 3] = input[offset + 3];

            offset += 4;
        }
    }
}