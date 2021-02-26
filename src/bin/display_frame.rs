use kyria_oled::Frame;
use std::env::args;
use std::io::{stdin, stdout, Write};
use termion::event::{Event, Key, MouseEvent};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let filenames: Vec<String> = args().skip(1).collect();
    let stdout = stdout();
    let stdout = &mut stdout.into_raw_mode().unwrap();
    let stdin = stdin();
    let mut evt = stdin.events();

    let mut current_idx = 0;

    loop {
        write!(
            stdout,
            "Displaying frame {}/{} ({})\r\n",
            current_idx + 1,
            filenames.len(),
            filenames[current_idx]
        )
        .unwrap();

        let frame = Frame::create_from_file(&filenames[current_idx]).unwrap();
        write!(
            stdout,
            "{}{}",
            format!("{}", frame).replace("\n", "\n\r"),
            termion::cursor::Up(frame.height() as u16 + 1),
        );

        loop {
            match evt.next().unwrap().unwrap() {
                Event::Key(Key::Left) => {
                    current_idx = (current_idx + filenames.len() - 1) % filenames.len();
                    break;
                }
                Event::Key(Key::Right) => {
                    current_idx = (current_idx + 1) % filenames.len();
                    break;
                }
                Event::Key(Key::Char('q'))
                | Event::Key(Key::Ctrl('c'))
                | Event::Key(Key::Ctrl('d')) => return,
                _ => (),
            }
        }
    }
}
