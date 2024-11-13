use crate::regex_pd::TAGS;

pub struct Filter {
    pub(crate) min: f32,
    pub(crate) max: f32,
    pub(crate) borderless: bool,
    pub(crate) include: Vec<String>,
    pub(crate) exclude: Vec<String>,
}

impl Filter {
    pub fn rarity(mut self, rarity: &String) -> Self {
        let r = parse_rarity(rarity.as_str()).unwrap();
        self.min = r.0;
        self.max = r.1;
        self
    }

    pub fn borderless(mut self, borderless: bool) -> Self {
        self.borderless = borderless;
        self
    }

    pub fn include(mut self, include: &String) -> Self {
        if !include.is_empty() {
            self.include = include
                .split(",")
                .map(|s| s.to_string().to_lowercase())
                .collect();
            ensure_tags_exist(&self.include);
        }
        self
    }

    pub fn exclude(mut self, exclude: &String) -> Self {
        if !exclude.is_empty() {
            self.exclude = exclude
                .split(",")
                .map(|s| s.to_string().to_lowercase())
                .collect();
            ensure_tags_exist(&self.exclude);
        }
        self
    }
}

impl Default for Filter {
    fn default() -> Self {
        Filter {
            min: 0.1,
            max: 1.0,
            borderless: true,
            include: vec![],
            exclude: vec![],
        }
    }
}

fn ensure_tags_exist(tags: &Vec<String>) {
    let invalid_tags = tags.iter().filter(|&t| !TAGS.contains(t)).collect::<Vec<&String>>();
    if !invalid_tags.is_empty() {
        eprintln!("Invalid tags: {:?}", invalid_tags);
        std::process::exit(1);
    }
}

pub fn parse_rarity(rarity: &str) -> anyhow::Result<(f32, f32)> {
    let parts: Vec<&str> = rarity.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid rarity format. \
        Format must be 'min:max', where min and max are decimal numbers seperated by a colon.");
    }
    let min = parts[0].parse::<f32>()?;
    let max = parts[1].parse::<f32>()?;
    if min < 0f32 || max > 1f32 {
        anyhow::bail!("Invalid rarity range. Range must be between 0 and 1 inclusive.");
    }
    Ok((min, max))
}
