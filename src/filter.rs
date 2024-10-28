pub struct Filter {
    pub(crate) min: f64,
    pub(crate) max: f64,
    pub(crate) borderless: bool
}

pub fn create_filter(
    rarity: Option<(f64, f64)>,
    borderless: bool,
    include: Option<&String>,
    exclude: Option<&String>,
) -> Filter {
    // TODO: Add include and exclude filter
    let mut filter: Filter = Filter { min: 0f64, max: 1f64, borderless };

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

pub fn parse_rarity(rarity: &str) -> anyhow::Result<(f64, f64)> {
    let parts: Vec<&str> = rarity.split(':').collect();
    if parts.len() != 2 {
        anyhow::bail!("Invalid rarity format. \
        Format must be 'min:max', where min and max are decimal numbers seperated by a colon.");
    }
    let min = parts[0].parse::<f64>()?;
    let max = parts[1].parse::<f64>()?;
    if min < 0f64 || max > 1f64 {
        anyhow::bail!("Invalid rarity range. Range must be between 0 and 1 inclusive.");
    }
    Ok((min, max))
}
