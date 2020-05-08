use super::coordinates::Coordinates;

#[derive(Debug)]
pub struct RawMaze {
    pub start: Coordinates,
    pub map: Vec<Vec<char>>,
    pub size: u8,
}

impl RawMaze {
    pub fn random_via_api(size: u8) -> RawMaze {
        let data = reqwest::get(&Self::generate_url(size))
            .unwrap()
            .json::<serde_json::Value>()
            .unwrap();
        let data: &serde_json::Map<String, serde_json::Value> = data.as_object().unwrap();
        let mut start = data["startingPosition"]
            .as_array()
            .unwrap()
            .iter()
            .map(|i| i.as_i64().unwrap() as u8);
        let start = Coordinates::from(start.next().unwrap(), start.next().unwrap());
        let map = data["map"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| {
                a.as_array()
                    .unwrap()
                    .iter()
                    .map(|s| s.as_str().unwrap().chars().nth(0).unwrap())
                    .collect()
            })
            .collect();
        RawMaze { map, size, start }
    }

    fn generate_url(size: u8) -> String {
        format!(
            "https://api.noopschallenge.com/mazebot/random?minSize={size}&maxSize={size}",
            size = size
        )
    }
}
