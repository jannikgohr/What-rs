pub struct Filter {
    pub(crate) min: f32,
    pub(crate) max: f32,
    pub(crate) borderless: bool
}

pub fn create_filter(
    rarity: Option<(f32, f32)>,
    borderless: bool,
    include: Option<&String>,
    exclude: Option<&String>,
) -> Filter {
    // TODO: Add include and exclude filter
    let mut filter: Filter = Filter { min: 0f32, max: 1f32, borderless };

    if let Some((min, max)) = rarity {
        filter.min = min;
        filter.max = max;
    }

    if let Some(tags) = include {
        println!("Including tags: {:?}", tags.split(',').collect::<Vec<&str>>());
    }

    if let Some(tags) = exclude {
        println!("Excluding tags: {:?}", tags.split(',').collect::<Vec<&str>>());
    }

    filter
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
