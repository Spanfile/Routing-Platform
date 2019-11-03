#[derive(Debug)]
pub struct Completions {
    completions: Vec<&'static str>,
}

impl Completions {
    pub fn new(completions: Vec<&'static str>) -> Self {
        rp_log::debug!("Completions: {:?}", completions);
        Completions { completions }
    }

    pub fn get(&self, input: String) -> anyhow::Result<Vec<&'static str>> {
        Ok(self
            .completions
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
