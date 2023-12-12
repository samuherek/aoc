

struct SeedSoil {
    seed: u32,
    soil: u32,
}

struct SoilFertilizer {
    soil: u32,
    fertilizer: u32,
}

struct FertilizerWater {
    fertilizer: u32,
    water: u32,
}

struct WaterLight {
    water: u32,
    light: u32
}

struct LightTemp {
    light: u32,
    temp: u32,
}

struct TempHumidity {
    temp: u32,
    humidity: u32,
}

struct HumidityLocation {
    humidity: u32,
    location: u32,
}

pub fn part1_solve(data: &String) -> usize {
    let (seeds, db) = data.split_once("\n").unwrap_or(("", ""));
    let seed_ids = seeds.split_once(":").map_or(Vec::new(), |(_, nums)| {
        return nums.trim()
            .split_whitespace()
            .filter_map(|x| x.trim().parse::<u32>().ok())
            .collect::<Vec<_>>();
    });
    //let mut _seed_to_soil = Vec::new();
    //let mut _soil_to_fert = Vec::new();
    //let mut _fet_to_water = Vec::new();
    //let mut _water_to_light = Vec::new();
    //let mut _light_to_temp = Vec::new();
    //let mut _temp_to_humidity = Vec::new();
    //let mut _humidity_to_loc = Vec::new();
     
    println!("seeds:: {:?}", seed_ids);

    return 0;
}
