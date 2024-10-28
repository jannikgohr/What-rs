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
        println!("Setting rarity filter: min={}, max={}", min, max);
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