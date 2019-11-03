use super::{KeyResult, MatcherResult, Result, Shell};
use termion::{clear, cursor, event::Key};

pub fn matcher(key: Key) -> MatcherResult {
    match key {
        Key::Left => Some(box left),
        Key::Right => Some(box right),
        Key::Up => Some(box up),
        Key::Down => Some(box down),
        _ => None,
    }
}

fn left(_key: Key, shell: &mut Shell) -> Result {
    if shell.cursor_location > 0 {
        shell.write(format_args!("{}", cursor::Left(1)))?;
        shell.cursor_location -= 1;
    }

    Ok(KeyResult::Ok)
}

fn right(_key: Key, shell: &mut Shell) -> Result {
    if shell.cursor_location < shell.buffer.len() {
        shell.write(format_args!("{}", cursor::Right(1)))?;
        shell.cursor_location += 1;
    }

    Ok(KeyResult::Ok)
}

fn up(_key: Key, shell: &mut Shell) -> Result {
    let (cursor_x, cursor_y) = shell.cursor_pos()?;

    if let Some(new_index) = match shell.history_index {
        Some(i) => {
            if i > 0 {
                shell.history_index = Some(i - 1);
                Some(i - 1)
            } else {
                Some(i)
            }
        }
        None => {
            if !shell.history.is_empty() {
                let i = shell.history.len() - 1;
                shell.history_index = Some(i);
                Some(i)
            } else {
                None
            }
        }
    } {
        shell.buffer = if let Some(entry) = shell.history.get(new_index) {
            entry.command.chars().collect()
        } else {
            return Ok(KeyResult::Skip);
        };

        shell.write(format_args!(
            "{}{}",
            cursor::Goto(cursor_x - shell.cursor_location as u16, cursor_y),
            clear::UntilNewline
        ))?;

        for b in shell.buffer.iter() {
            shell.write(format_args!("{}", b))?;
        }

        shell.cursor_location = shell.buffer.len();
    }

    Ok(KeyResult::Ok)
}

fn down(_key: Key, shell: &mut Shell) -> Result {
    let (cursor_x, cursor_y) = shell.cursor_pos()?;

    shell.write(format_args!(
        "{}{}",
        cursor::Goto(cursor_x - shell.cursor_location as u16, cursor_y),
        clear::UntilNewline
    ))?;

    if let Some(new_index) = match shell.history_index {
        Some(i) => {
            if i == shell.history.len() - 1 {
                shell.history_index = None;
            } else {
                shell.history_index = Some(i + 1);
            }
            shell.history_index
        }
        None => None,
    } {
        shell.buffer = if let Some(entry) = shell.history.get(new_index) {
            entry.command.chars().collect()
        } else {
            return Ok(KeyResult::Skip);
        };
    } else {
        shell.buffer.clear();
    }

    for b in shell.buffer.iter() {
        shell.write(format_args!("{}", b))?;
    }

    shell.cursor_location = shell.buffer.len();

    Ok(KeyResult::Ok)
}
