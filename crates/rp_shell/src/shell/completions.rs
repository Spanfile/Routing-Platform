#[derive(Debug)]
pub struct Completions {
    completions: Vec<&'static str>,
}

impl Completions {
    pub fn new() -> Self {
        Completions {
            completions: Vec::new(),
        }
    }

    pub fn get(&self, input: String) -> anyhow::Result<Vec<&'static str>> {
        Ok(vec!["asdasdasd", "kekekeke"])
    }
}
