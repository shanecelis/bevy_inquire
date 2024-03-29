use std::fmt::Display;
use std::io::Result;
use std::io::Write;
use inquire::error::InquireResult;
use inquire::InquireError;
use inquire::terminal::{Terminal, TerminalSize};
use inquire::ui::{Key, Styled, KeyModifiers, InputReader};
mod text_style_adapter;

use text_style_adapter::StyledStringWriter;
use bevy::prelude::*;

#[derive(Component)]
pub struct BevyTerminal {
    size: TerminalSize,
    writer: StyledStringWriter,
}

impl Default for BevyTerminal {
    fn default() -> Self {
        Self {
            size: TerminalSize::new(24, 80),// { width: 80, height: 24 },
            writer: StyledStringWriter::default()
        }
    }
}

pub struct BevyInput {
    keys: Vec<Key>,
}

pub fn from_input(input: &ButtonInput<KeyCode>) -> KeyModifiers {
    let mut mods = KeyModifiers::empty();
    if input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]) {
        mods |= KeyModifiers::SHIFT;
    }
    if input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]) {
        mods |= KeyModifiers::CONTROL;
    }
    if input.any_pressed([KeyCode::AltLeft, KeyCode::AltRight]) {
        mods |= KeyModifiers::ALT;
    }
    if input.any_pressed([KeyCode::SuperLeft, KeyCode::SuperRight]) {
        mods |= KeyModifiers::SUPER;
    }
    mods
}

impl InputReader for BevyInput {
    fn read_key(&mut self) -> InquireResult<Key> {
        self.keys.pop().ok_or(InquireError::OperationCanceled)
    }
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
