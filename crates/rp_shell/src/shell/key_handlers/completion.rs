use super::{KeyResult, Result, Shell};
use crate::error;
use termion::event::Key;

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
        }
        _ => {
            shell.suspend_raw_mode()?;
            return Err(error::ShellError::AmbiguousCompletion(completions).into());
        }
    };

    Ok(KeyResult::Ok)
}
