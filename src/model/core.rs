pub struct Core {
    is_team_zero: bool,
    score: u32,
    goals: u32,
    assists: u32,
    saves: u32,
    shots: u32,
    demos_inflicted: u32,
    demos_taken: u32
}

impl Core {
    pub fn new() -> Self {
        Self {
            is_team_zero: false,
            score: 0,
            goals: 0,
            assists: 0,
            saves: 0,
            shots: 0,
            demos_inflicted: 0,
            demos_taken: 0
        }
    }
}