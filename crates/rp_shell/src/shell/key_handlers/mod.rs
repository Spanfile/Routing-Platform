mod completion;
mod editing;
mod navigation;

use super::Shell;
use crate::error;
use anyhow::anyhow;
use lazy_static::lazy_static;
use std::collections::HashMap;
use termion::event::Key;

type Result = anyhow::Result<KeyResult>;

lazy_static! {
    static ref KEYS: HashMap<Key, Box<dyn (Fn(Key, &mut Shell) -> Result) + Send + Sync + 'static>> = {
        let mut m = HashMap::new();
        m.insert(
            Key::Ctrl('c'),
            // this cast turns the function into the proper type while also implicitly causing the rest to be cast as well
            box terminate as Box<dyn (Fn(Key, &mut Shell) -> Result) + Send + Sync + 'static>,
        );
        m.insert(Key::Char('\t'), box completion::tab);
        m.insert(Key::Left, box navigation::left);
        m.insert(Key::Right, box navigation::right);
        m.insert(Key::Up, box navigation::up);
        m.insert(Key::Down, box navigation::down);
        m.insert(Key::Backspace, box editing::backspace);
        m.insert(Key::Delete, box editing::delete);
        m.insert(Key::Char('\n'), box editing::enter);
        // m.insert(Key::Char(), box editing::key);
        m
    };
}

pub enum KeyResult {
    Ok,
    Skip,
    Stop,
}

pub fn get(
    key: Key,
) -> anyhow::Result<&'static (dyn (Fn(Key, &mut Shell) -> Result) + Send + Sync + 'static)> {
    Ok(KEYS
        .get(&key)
        .ok_or(anyhow!("no handler for key {:?}", key))?)
}

fn terminate(_key: Key, shell: &mut Shell) -> Result {
    shell.suspend_raw_mode()?;
    Err(error::ShellError::Abort.into())
}
