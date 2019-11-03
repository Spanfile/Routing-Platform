use super::{KeyResult, MatcherResult, Result, Shell};
use crate::error;
use termion::event::Key;

pub fn matcher(key: Key) -> MatcherResult {
    if let Key::Char('\t') = key {
        Some(box tab)
    } else {
        None
    }
}

pub fn tab(_key: Key, shell: &mut Shell) -> Result {
    let completions = shell.completions.get(shell.buffer.iter().collect())?;
    match completions.len() {
        0 => return Ok(KeyResult::Skip),
        1 => {
            let completed = completions.first().unwrap();
            shell.buffer = completed.chars().collect();

            for b in shell.buffer[shell.cursor_location..].iter() {
                shell.write(format_args!("{}", b))?;
            }

            shell.cursor_location = shell.buffer.len();
        }
        _ => {
            return Err(error::ShellError::AmbiguousCompletion(completions).into());
        }
    };

    Ok(KeyResult::Ok)
}
