mod completion;
mod editing;
mod navigation;

use super::Shell;
use crate::error;
use anyhow::anyhow;
use lazy_static::lazy_static;
use termion::event::Key;

type Result = anyhow::Result<KeyResult>;
type MatcherResult = Option<KeyPredicate>;
type KeyPredicate = Box<dyn (Fn(Key, &mut Shell) -> Result) + Send + Sync + 'static>;
type MatcherPredicate = Box<dyn (Fn(Key) -> MatcherResult) + Send + Sync + 'static>;

lazy_static! {
    static ref KEYS: Vec<MatcherPredicate> = {
        vec![
            // this cast turns the function into the proper type while also implicitly causing the rest to be cast as well
            box terminate_matcher as MatcherPredicate,
            box completion::matcher,
            box navigation::matcher,
            box editing::matcher,
        ]
    };
}

pub enum KeyResult {
    Ok,
    Skip,
    Stop,
}

pub fn get(key: Key) -> anyhow::Result<KeyPredicate> {
    KEYS.iter()
        .find_map(|matcher| matcher(key))
        .ok_or_else(|| anyhow!("no key handler for key {:?}", key))
}

fn terminate_matcher(key: Key) -> MatcherResult {
    if let Key::Ctrl('c') = key {
        Some(box terminate)
    } else {
        None
    }
}

fn terminate(_key: Key, _shell: &mut Shell) -> Result {
    Err(error::ShellError::Abort.into())
}
