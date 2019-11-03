use super::{KeyResult, MatcherResult, Result, Shell};
use anyhow::anyhow;
use termion::{clear, cursor, event::Key};

pub fn matcher(key: Key) -> MatcherResult {
    match key {
        Key::Backspace => Some(box backspace),
        Key::Delete => Some(box delete),
        Key::Char('\n') => Some(box enter),
        Key::Char(_) => Some(box any_char),
        _ => None,
    }
}

fn backspace(_key: Key, shell: &mut Shell) -> Result {
    if shell.cursor_location > 0 {
        shell.buffer.remove(shell.cursor_location - 1);
        let (cursor_x, cursor_y) = shell.cursor_pos()?;

        shell.write(format_args!(
            "{}{}",
            cursor::Goto(cursor_x - shell.cursor_location as u16, cursor_y),
            clear::UntilNewline
        ))?;

        for b in shell.buffer.iter() {
            shell.write(format_args!("{}", b))?;
        }

        shell.write(format_args!("{}", cursor::Goto(cursor_x - 1, cursor_y)))?;

        shell.cursor_location -= 1;
    }

    Ok(KeyResult::Ok)
}

fn delete(_key: Key, shell: &mut Shell) -> Result {
    if shell.cursor_location < shell.buffer.len() {
        shell.buffer.remove(shell.cursor_location);
        let (cursor_x, cursor_y) = shell.cursor_pos()?;

        shell.write(format_args!("{}", clear::UntilNewline))?;

        for b in shell.buffer.iter() {
            shell.write(format_args!("{}", b))?;
        }

        shell.write(format_args!("{}", cursor::Goto(cursor_x, cursor_y)))?;
    }

    Ok(KeyResult::Ok)
}

fn enter(_key: Key, shell: &mut Shell) -> Result {
    shell.write(format_args!("\n\r"))?;
    Ok(KeyResult::Stop)
}

fn any_char(key: Key, shell: &mut Shell) -> Result {
    if let Key::Char(c) = key {
        shell.write(format_args!("{}", c))?;

        if shell.cursor_location == shell.buffer.len() {
            shell.buffer.push(c);
        } else {
            shell.buffer.insert(shell.cursor_location, c);

            for b in shell.buffer[shell.cursor_location + 1..].iter() {
                shell.write(format_args!("{}", b))?;
            }

            shell.write(format_args!(
                "{}",
                cursor::Left((shell.buffer.len() - shell.cursor_location - 1) as u16)
            ))?;
        }

        shell.cursor_location += 1;

        Ok(KeyResult::Ok)
    } else {
        Err(anyhow!("unexpected key input for key function 'key'"))
    }
}
