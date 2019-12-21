use rp_core::log::*;

#[derive(Debug)]
pub struct Completions {
    commands: Vec<&'static str>,
}

impl Completions {
    pub fn new(commands: Vec<&'static str>) -> Self {
        debug!("Completions: commands: {:?}", commands);
        Completions { commands }
    }

    pub fn get(&self, input: String) -> anyhow::Result<Vec<&'static str>> {
        Ok(self
            .commands
            .iter()
            .filter_map(|comp| {
                if comp.starts_with(&input) {
                    Some(*comp)
                } else {
                    None
                }
            })
            .collect())
    }
}
