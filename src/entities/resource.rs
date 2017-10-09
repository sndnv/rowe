#[derive(Debug)]
pub struct Level {
    pub max: u8,
    pub current: u8
}

#[derive(Debug)]
pub struct Resource {
    pub name: String,
    pub level: Level,
    pub replenish_time: Option<u16>
}