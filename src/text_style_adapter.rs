use std::fmt;
use std::io;
use itertools::Itertools;

use inquire::ui::*;
use bevy::ecs::{entity::Entity, system::{Commands, Resource}};
use bevy::text::TextStyle;
use bevy::ui::{Style, FlexDirection, node_bundles::{TextBundle, NodeBundle}};
// use bevy::prelude::*;
use bevy::prelude::{BuildChildren, Color as BevyColor};
use bevy::utils::default;
use bevy::hierarchy::ChildBuilder;

#[derive(Resource, Debug, Default)]
pub struct BevySettings {
    pub style: TextStyle,
}

#[derive(Debug, Clone, Default)]
pub struct StyledStringWriter {
    pub style: StyleSheet,
    pub strings: Vec<Styled<String>>,
    pub state: RendererState,
    pub(crate) cursor_pos: Option<CursorPos>,
    pub(crate) cursor_pos_save: Option<CursorPos>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct CursorPos {
    index: usize,
    len: usize
}

impl StyledStringWriter {
    pub fn clear(&mut self) {
        self.state = RendererState::default();
        self.cursor_pos = None;
        self.cursor_pos_save = None;
    }

    fn get_cursor_pos(&mut self) -> CursorPos {
        if self.strings.len() == 0 {
            self.strings.push(Styled { content: String::new(), style: self.style });
        }
        match &self.cursor_pos {
            None => CursorPos { index: self.strings.len() - 1, len: self.strings.last().unwrap().content.chars().count() },
            Some(c) => c.clone()
        }
    }

    fn set_cursor_pos(&mut self, cursor_pos: CursorPos) {
        // if cursor_pos.len < 0 {
        //     cursor_pos.index -= 1;
        //     cursor_pos.len += self.strings[cursor_pos.index].s.chars().count();
        // }
        self.cursor_pos = Some(cursor_pos);
    }

    pub(crate) fn drain_with_styled_cursor(&mut self, color: Color) -> Vec<Styled<String>> {
        let cursor_pos = self.get_cursor_pos();
        let mut strings = std::mem::take(&mut self.strings);
        let styled_string = std::mem::replace(&mut strings[cursor_pos.index], Styled::new(String::new()));

        // eprintln!("cursor {:?} str len {}", cursor_pos, styled_string.s.len());
        let _ = strings.splice(cursor_pos.index..cursor_pos.index + 1, cursorify(styled_string, cursor_pos.len, color));
        strings
    }

    fn render(
        &mut self,
        commands: &mut Commands,
        settings: &BevySettings,
        column: Entity,
    ) {
        // -> io::Result<()>
        let white = Color::Grey;

        let strings = if self.state.cursor_visible {
            self.drain_with_styled_cursor(white)
        } else {
            std::mem::take(&mut self.strings)
        };

        commands.entity(column).with_children(|column| {
            let mut next_line_count: Option<usize> = None;
            let mut line_count: usize = 0;
            let lines = strings
                .into_iter()
                .flat_map(|mut s| {
                    let mut a = vec![];
                    let mut b = None;
                    if s.content.contains('\n') {
                        let str = std::mem::take(&mut s.content);
                        a.extend(str.split_inclusive('\n').map(move |line| Styled {
                            content: line.to_string(),
                            ..s.clone()
                        }));
                    } else {
                        b = Some(s);
                    }
                    a.into_iter().chain(b)
                })
                .group_by(|x| {
                    if let Some(x) = next_line_count.take() {
                        line_count = x;
                    }
                    if x.content.chars().last().map(|c| c == '\n').unwrap_or(false) {
                        next_line_count = Some(line_count + 1);
                    }
                    line_count
                });

            // let mut line_num = 0;
            for (_key, line) in &lines {
                // let style: TextStyleParams = settings.style.clone().into();
                column
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Row,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        // if out.state.cursor_visible && line_num == out.state.cursor_pos[1] {
                        //     text_style::bevy::render_iter(
                        //         parent,
                        //         &style,
                        //         cursorify_iter(line, out.state.cursor_pos[0], white),
                        //     );
                        // } else {
                        render_iter(parent, &settings.style, line);
                        // }
                    });
                // line_num += 1;
            }
        });
    }
}

pub fn render<'a>(
    parent: &mut ChildBuilder<'_>,
    o: &TextStyle,
    s: impl Into<Styled<String>>,
) {
    parent.spawn(with_style_string(s.into(), o));
}

pub fn render_iter<'a, I, Iter, S>(
    parent: &mut ChildBuilder<'_>,
    o: &TextStyle,
    iter: I,
) where
    I: IntoIterator<Item = S, IntoIter = Iter>,
    Iter: Iterator<Item = S>,
    S: Into<Styled<String>>,
{
    iter.into_iter().for_each(|b| render(parent, o, b));
}

// I originally had this function too:
//
// fn with_style_str<'a>(s: StyledStr<'a>, text_style_params: &TextStyleParams) -> TextBundle;
//
// but TextBundle requires a String, so Into<StyledString> seemed more explicit.
fn with_style_string(
    s: impl Into<Styled<String>>,
    text_style: &TextStyle,
) -> TextBundle {
    let s = s.into();
    let bundle = TextBundle::from_section(
        s.content,
        s.style.fg.map(|fg| TextStyle {
            color: from_color(fg),
            ..text_style.clone()
        }).unwrap_or(text_style.clone())
    );
    let bg: Option<BevyColor> = s.style.bg.map(from_color);
    match bg {
        None => bundle,
        Some(color) => bundle.with_background_color(color),
    }
}

fn from_color(color: Color) -> BevyColor {
    use inquire::ui::Color::*;

    let (r, g, b) = match color {
        Black => (0, 0, 0),
        DarkRed => (170, 0, 0),
        DarkGreen => (0, 170, 0),
        DarkYellow => (170, 85, 0),
        DarkBlue => (0, 0, 170),
        DarkMagenta => (170, 0, 170),
        DarkCyan => (0, 170, 170),
        DarkWhite => (170, 170, 170),
        LightBlack => (85, 85, 85),
        LightRed => (255, 85, 85),
        LightGreen => (85, 255, 85),
        LightYellow => (255, 255, 85),
        LightBlue => (85, 85, 255),
        LightMagenta => (255, 85, 255),
        LightCyan => (85, 255, 255),
        LightWhite => (255, 255, 255),
        Rgb { r, g, b } => (r, g, b),
        _ => todo!(),
    };
    BevyColor::rgb_u8(r, g, b)
}

// fn no_cursorify(
//     cs: Styled<String>,
//     i: usize,
//     cursor_color: text_style::Color,
// ) -> impl Iterator<Item = Styled<String>> {
//     std::iter::once(cs)
// }

/// Splits Styled<String> into possibly three pieces: (left string portion, the
/// cursor, right string portion). The character index `i`'s range is not the
/// usual _[0, N)_ where _N_ is the character count; it is _[0,N]_ inclusive so
/// that a cursor may be specified essentially at the end of the strin g.
fn cursorify(
    cs: Styled<String>,
    i: usize,
    cursor_color: Color,
) -> impl Iterator<Item = Styled<String>> {
    let (string, style) = (cs.content, cs.style);
    assert!(i <= string.chars().count(),
            "i {} <= str.chars().count() {}", i, string.chars().count());
    let (mut input, right) = match string.char_indices().nth(i + 1) {
        Some((byte_index, _char)) => {
            let (l, r) = string.split_at(byte_index);
            (l.to_owned(),Some(Styled { content: r.to_owned(), style }))
        },
        None => {
            let mut s = string;
            if s.chars().count() == i {
                s.push(' ');
            }
            (s, None)
        }
    };
    let cursor = Some(
        Styled {
            content: input
                .pop()
            // Newline is not printed. So use a space if necessary.
            // .map(|c| if c == '\n' { ' ' } else { c })
            // .unwrap()//_or(' ')
                .expect("Could not get cursor position")
                .to_string(),
            style: style.with_bg(cursor_color)
        }
    );
    let left = Some(Styled { content: input, style });
    left.into_iter().chain(cursor.into_iter().chain(right))
}

#[derive(Debug, Default, Clone)]
pub struct RendererState {
    // pub(crate) draw_time: DrawTime,
    pub(crate) cursor_visible: bool,
    pub(crate) newline_count: u16,
}

impl std::io::Write for StyledStringWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = std::str::from_utf8(buf).expect("Not a utf8 string");
        let ss = match self.strings.pop() {
            None => Styled { content: s.to_string(), style: self.style },
            Some(mut text) => {
                if text.style == self.style {
                    text.content.push_str(s);
                    text
                } else {
                    self.strings.push(text);
                    Styled { content: s.to_string(), style: self.style }
                }
            }
        };
        self.strings.push(ss);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl std::fmt::Write for StyledStringWriter {
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        let ss = match self.strings.pop() {
            None => Styled { content: s.to_string(), style: self.style },
            Some(mut text) => {
                if text.style == self.style {
                    text.content.push_str(s);
                    text
                } else {
                    self.strings.push(text);
                    Styled { content: s.to_string(), style: self.style }
                }
            }
        };
        self.strings.push(ss);
        Ok(())
    }
}

// impl Renderer for StyledStringWriter {
//     fn draw_time(&self) -> DrawTime {
//         self.state.draw_time
//     }

//     fn newline_count(&mut self) -> &mut u16 {
//         &mut self.state.newline_count
//     }

//     fn update_draw_time(&mut self) {
//         self.state.draw_time = match self.state.draw_time {
//             DrawTime::First => DrawTime::Update,
//             _ => DrawTime::Last,
//         }
//     }

//     fn set_foreground(&mut self, color: Color) -> io::Result<()> {
//         let style = self.style.get_or_insert(Style::default());
//         style.fg = Some(color);
//         Ok(())
//     }

//     fn set_background(&mut self, color: Color) -> io::Result<()> {
//         let style = self.style.get_or_insert(Style::default());
//         style.bg = Some(color);
//         Ok(())
//     }

//     fn reset_color(&mut self) -> io::Result<()> {
//         let style = self.style.get_or_insert(Style::default());
//         style.fg = None;
//         style.bg = None;
//         Ok(())
//     }

//     fn pre_prompt(&mut self) -> io::Result<()> {
//         Ok(())
//     }

//     fn post_prompt(&mut self) -> io::Result<()> {
//         Ok(())
//     }

//     /// Utility function for line input.
//     /// Set initial position based on the position after drawing.
//     fn move_cursor(&mut self, [x, _y]: [usize; 2]) -> io::Result<()> {
//         if self.state.draw_time == DrawTime::Last {
//             return Ok(());
//         }
//         let mut c = self.get_cursor_pos();
//         c.len += x;
//         self.set_cursor_pos(c);

//         // self.state.cursor_pos[0] += x;
//         // self.state.cursor_pos[1] += y;
//         Ok(())
//     }

//     fn save_cursor(&mut self) -> io::Result<()> {
//         self.cursor_pos_save = Some(self.get_cursor_pos());
//         Ok(())
//     }

//     fn restore_cursor(&mut self) -> io::Result<()> {
//         self.cursor_pos = self.cursor_pos_save.clone();
//         Ok(())
//     }

//     fn hide_cursor(&mut self) -> io::Result<()> {
//         self.state.cursor_visible = false;
//         Ok(())
//     }

//     fn show_cursor(&mut self) -> io::Result<()> {
//         self.state.cursor_visible = true;
//         Ok(())
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::*;
//     use std::io::Write;
//     use text_style::{self, AnsiColor, Styled<String>};
//     #[test]

//         let mut w = StyledStringWriter::default();
//         let v = w.drain_with_styled_cursor(AnsiColor::White.dark());
//         assert_eq!(v.len(), 2);
//     }

//     #[test]
//     fn test_cursorify2() -> std::io::Result<()> {
//         let mut w = StyledStringWriter::default();
//         write!(w, "what the fuck")?;
//         w.set_foreground(AnsiColor::Black.light())?;
//         write!(w, "huh")?;
//         let v = w.drain_with_styled_cursor(AnsiColor::White.dark());
//         assert_eq!(v.len(), 3);
//         Ok(())
//     }

//     #[test]
//     fn test_cursorify3() {
//         let s = Styled<String>::new(" ".into(), None);
//         let v: Vec<_> = cursorify(s, 0, AnsiColor::White.dark()).collect();
//         assert_eq!(v.len(), 2);
//         assert_eq!(&v[0].s, "");
//         assert_eq!(v[0].style, None);
//         assert_eq!(&v[1].s, " ");
//         assert_ne!(v[1].style, None);
//     }

//     #[test]
//     fn test_cursorify5() {
//         let s = Styled<String>::new("a".into(), None);
//         let v: Vec<_> = cursorify(s, 1, AnsiColor::White.dark()).collect();
//         assert_eq!(v.len(), 2);
//         assert_eq!(&v[0].s, "a");
//         assert_eq!(v[0].style, None);
//         assert_eq!(&v[1].s, " ");
//         assert_ne!(v[1].style, None);
//     }

//     #[test]
//     fn test_cursorify4() {
//         let s = Styled<String>::new("".into(), None);
//         let v: Vec<_> = cursorify(s, 0, AnsiColor::White.dark()).collect();
//         assert_eq!(v.len(), 2);
//         assert_eq!(v[0].style, None);
//         assert_ne!(v[1].style, None);
//     }

//     mod unicode {
//         use super::*;
//         use std::io::Write;
//         use text_style::{self, AnsiColor, Styled<String>};
//         #[test]
//         fn test_cursorify() {
//             let mut w = StyledStringWriter::default();
//             let v = w.drain_with_styled_cursor(AnsiColor::White.dark());
//             assert_eq!(v.len(), 2);
//         }

//         #[test]
//         fn test_cursorify2() -> std::io::Result<()> {
//             let mut w = StyledStringWriter::default();
//             write!(w, "▣what the fuck")?;
//             w.set_foreground(AnsiColor::Black.light())?;
//             write!(w, "▣huh")?;
//             let v = w.drain_with_styled_cursor(AnsiColor::White.dark());
//             assert_eq!(v.len(), 3);
//             Ok(())
//         }

//         #[test]
//         fn test_cursorify3() {
//             let s = Styled<String>::new("▣".into(), None);
//             let v: Vec<_> = cursorify(s, 0, AnsiColor::White.dark()).collect();
//             assert_eq!(v.len(), 2);
//             assert_eq!(&v[0].s, "");
//             assert_eq!(v[0].style, None);
//             assert_eq!(&v[1].s, "▣");
//             assert_ne!(v[1].style, None);
//         }

//         #[test]
//         fn test_unicode_cursorify5() {
//             let s = Styled<String>::new("▣".into(), None);
//             let v: Vec<Styled<String>> = cursorify(s, 1, AnsiColor::White.dark()).collect();
//             assert_eq!(v.len(), 2);
//             // assert_eq!(v[0].s.len(), 0);
//             // assert_eq!(&v[0].s, "");
//             assert_eq!(&v[0].s, "▣");
//             assert_eq!(v[0].style, None);
//             assert_eq!(&v[1].s, " ");
//             assert_ne!(v[1].style, None);
//         }

//         #[test]
//         fn test_cursorify4() {
//             let s = Styled<String>::new("".into(), None);
//             let v: Vec<_> = cursorify(s, 0, AnsiColor::White.dark()).collect();
//             assert_eq!(v.len(), 2);
//             assert_eq!(v[0].style, None);
//             assert_ne!(v[1].style, None);
//         }
//     }
// }
