use std::fmt::Display;
use std::io::Result;
use std::io::Write;
use inquire::terminal::{Terminal, TerminalSize};
use inquire::ui::Styled;
mod text_style_adapter;

use text_style_adapter::StyledStringWriter;

struct BevyTerminal {
    size: TerminalSize,
    writer: StyledStringWriter,
}

impl Terminal for BevyTerminal {
    fn get_size(&self) -> Result<TerminalSize> {
        Ok(self.size)
    }

    fn write<T: Display>(&mut self, val: T) -> Result<()> {
        write!(self.writer, "{}", val)
    }

    fn write_styled<T: Display>(&mut self, val: &Styled<T>) -> Result<()> {
        write!(self.writer, "{}", val.content)
    }

    fn clear_line(&mut self) -> Result<()> { todo!() }
    fn clear_until_new_line(&mut self) -> Result<()> { todo!() }

    fn cursor_hide(&mut self) -> Result<()> {
        self.writer.state.cursor_visible = false;
        Ok(())
    }

    fn cursor_show(&mut self) -> Result<()> {
        self.writer.state.cursor_visible = true;
        Ok(())
    }
    fn cursor_up(&mut self, cnt: u16) -> Result<()> { todo!() }
    fn cursor_down(&mut self, cnt: u16) -> Result<()> { todo!() }
    fn cursor_left(&mut self, cnt: u16) -> Result<()> { todo!() }
    fn cursor_right(&mut self, cnt: u16) -> Result<()> { todo!() }
    fn cursor_move_to_column(&mut self, idx: u16) -> Result<()> { todo!() }

    fn flush(&mut self) -> Result<()> { todo!() }
}
