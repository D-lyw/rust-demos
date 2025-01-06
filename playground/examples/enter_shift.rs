use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyModifiers}, execute, terminal::{disable_raw_mode, enable_raw_mode}};
use ractor::factory::discard;
use std::io;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))?;

    loop {
        // 读取事件
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                println!("{:?}, {:?}", code, modifiers);
                // 检查是否按下了 Enter 键和 Shift 键
                if code == KeyCode::Enter && modifiers.contains(KeyModifiers::SHIFT) {
                    println!("Shift + Enter detected!");
                    // 在这里调用你的自定义函数
                    custom_function();
                }
            }
        }
    }
    disable_raw_mode()?;
}

fn custom_function() {
    println!("Custom function triggered!");
}
