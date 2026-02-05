use std::thread;
use std::time::Duration;
use core_graphics::event::{CGEvent, CGEventFlags, CGKeyCode, EventField};
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use core_graphics::event::CGEventTapLocation;
use std::sync::mpsc;
use core_graphics::display::CGWarpMouseCursorPosition;
use core_graphics::geometry::CGPoint;
#[derive(Debug, Clone)]


pub enum KeyJob {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
    Num0, Num1, Num2, Num3, Num4, Num5, Num6, Num7, Num8, Num9,
    Return,
    Equal,
    Tab,
    Space,
    Delete,
    Escape,
    Command,
    Shift,
    Control,
    Option,
    Minus,
    RightBracket,
    CapsLock,
    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12,
    LeftArrow,
    RightArrow,
    Semicolon,
    LeftBracket,
    UpArrow,
    DownArrow,
    Quote,
    Backslash,
    Comma,
    Slash,
    Period,
    Grave,
    RightShift,
    RightOption,
    RightControl,
    from_string(String),
}
impl KeyJob {
    fn keycode(&self) -> u16 {
        match self {
            KeyJob::from_string(KeyCodeAsString) => {
                match KeyCodeAsString.to_lowercase().as_str()
                {
                    "a" => 0,
                    "s" => 1,
                    "d" => 2,
                    "f" => 3,
                    "h" => 4,
                    "g" => 5,
                    "z" => 6,
                    "x" => 7,
                    "c" => 8,
                    "v" => 9,
                    "b" => 11,
                    "q" => 12,
                    "w" => 13,
                    "e" => 14,
                    "r" => 15,
                    "y" => 16,
                    "t" => 17,
                    "o" => 31,
                    "u" => 32,
                    "i" => 34,
                    "p" => 35,
                    "l" => 37,
                    "j" => 38,
                    "k" => 40,
                    "n" => 45,
                    "m" => 46,
                    "0" => 29,
                    "1" => 18,
                    "2" => 19,
                    "3" => 20,
                    "4" => 21,
                    "5" => 23,
                    "6" => 22,
                    "7" => 26,
                    "8" => 28,
                    "9" => 25,
                    "return" => 36,
                    "enter" => 36,
                    "tab" => 48,
                    "space" => 49,
                    "delete" => 51,
                    "escape" => 53,
                    "command" => 55,
                    "shift" => 56,
                    "capslock" => 57,
                    "option" => 58,
                    "control" => 59,
                    "rightshift" => 60,
                    "rightoption" => 61,
                    "rightcontrol" => 62,
                    "f1" => 122,
                    "f2" => 120,
                    "f3" => 99,
                    "f4" => 118,
                    "f5" => 96,
                    "f6" => 97,
                    "f7" => 98,
                    "f8" => 100,
                    "f9" => 101,
                    "f10" => 109,
                    "f11" => 103,
                    "f12" => 111,
                    "left" | "leftarrow" => 123,
                    "right" | "rightarrow" => 124,
                    "down" | "downarrow" => 125,
                    "up" | "uparrow" => 126,
                    _ => 999
                }
            },
            KeyJob::A => 0,
            KeyJob::S => 1,
            KeyJob::D => 2,
            KeyJob::F => 3,
            KeyJob::H => 4,
            KeyJob::G => 5,
            KeyJob::Z => 6,
            KeyJob::X => 7,
            KeyJob::C => 8,
            KeyJob::V => 9,
            KeyJob::B => 11,
            KeyJob::Q => 12,
            KeyJob::W => 13,
            KeyJob::E => 14,
            KeyJob::R => 15,
            KeyJob::Y => 16,
            KeyJob::T => 17,
            KeyJob::Num1 => 18,
            KeyJob::Num2 => 19,
            KeyJob::Num3 => 20,
            KeyJob::Num4 => 21,
            KeyJob::Num6 => 22,
            KeyJob::Num5 => 23,
            KeyJob::Equal => 24,
            KeyJob::Num9 => 25,
            KeyJob::Num7 => 26,
            KeyJob::Minus => 27,
            KeyJob::Num8 => 28,
            KeyJob::Num0 => 29,
            KeyJob::RightBracket => 30,
            KeyJob::O => 31,
            KeyJob::U => 32,
            KeyJob::LeftBracket => 33,
            KeyJob::I => 34,
            KeyJob::P => 35,
            KeyJob::Return => 36,
            KeyJob::L => 37,
            KeyJob::J => 38,
            KeyJob::Quote => 39,
            KeyJob::K => 40,
            KeyJob::Semicolon => 41,
            KeyJob::Backslash => 42,
            KeyJob::Comma => 43,
            KeyJob::Slash => 44,
            KeyJob::N => 45,
            KeyJob::M => 46,
            KeyJob::Period => 47,
            KeyJob::Tab => 48,
            KeyJob::Space => 49,
            KeyJob::Grave => 50,
            KeyJob::Delete => 51,
            KeyJob::Escape => 53,
            KeyJob::Command => 55,
            KeyJob::Shift => 56,
            KeyJob::CapsLock => 57,
            KeyJob::Option => 58,
            KeyJob::Control => 59,
            KeyJob::RightShift => 60,
            KeyJob::RightOption => 61,
            KeyJob::RightControl => 62,
            KeyJob::F1 => 122,
            KeyJob::F2 => 120,
            KeyJob::F3 => 99,
            KeyJob::F4 => 118,
            KeyJob::F5 => 96,
            KeyJob::F6 => 97,
            KeyJob::F7 => 98,
            KeyJob::F8 => 100,
            KeyJob::F9 => 101,
            KeyJob::F10 => 109,
            KeyJob::F11 => 103,
            KeyJob::F12 => 111,
            KeyJob::UpArrow => 126,
            KeyJob::DownArrow => 125,
            KeyJob::LeftArrow => 123,
            KeyJob::RightArrow => 124,
        }
    }
}

#[derive(Debug)]
pub enum MouseJobPos
{
    Pos(f64,f64)
}
impl MouseJobPos
{
    fn OpenPos(&self) -> Vec<f64>
    {
        match self
        {
            MouseJobPos::Pos(x,y) => vec![*x,*y]
        }
    }
}

pub struct JanKeymap
{
    os:String,//Future Crossplatform
    latest_job:KeyJob,
    latest_sequance:String,
    latest_movement:Vec<f64>,
}
impl JanKeymap
{
    pub fn new() -> Self
    {
        Self{
            os:std::env::consts::OS.to_string(),
            latest_job:KeyJob::A,
            latest_sequance:String::from(""),
            latest_movement:Vec::new()
        }
    }
    fn SimulateKeyboardPress(&mut self) -> bool{
        let latest_job = self.latest_job.clone();
        let key = latest_job.keycode();
        if key == 999 {return false;}
        let Job = thread::spawn(move ||{
            let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
            let event = CGEvent::new_keyboard_event(source.clone(), key, true).unwrap();
            
            event.post(CGEventTapLocation::HID);
            let event = CGEvent::new_keyboard_event(source.clone(), key, false).unwrap();
            event.post(CGEventTapLocation::HID);
        });
        //Job.join().unwrap();
        return true;
    }
    pub fn KeyboardEvent(&mut self,SelectedKey:KeyJob) -> Option<bool>
    {
        self.latest_job = SelectedKey;
        if self.SimulateKeyboardPress() == false{
            return None;
        }
        Some(true)
    }
    pub fn KeyboardStringSequance(&mut self,sequance:String)
    {
        let _ = sequance.chars().for_each(|c| {    
            self.latest_job = KeyJob::from_string(c.to_string());
            self.SimulateKeyboardPress(); 
        });
    }
    pub fn MouseMovement(&mut self,pos_sequance:MouseJobPos)
    {
        self.latest_movement = match pos_sequance{
            MouseJobPos::Pos(x,y) => vec![x,y]
        };
        unsafe{
            let latest_movement_cln = self.latest_movement.clone();
            thread::spawn(move || {
                CGWarpMouseCursorPosition(CGPoint::new(latest_movement_cln[0],latest_movement_cln[1]));
            });
        }
    }
}