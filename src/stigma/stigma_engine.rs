#[derive(Debug, Clone, Copy)]
pub struct StigmaState {
    pub weight: f64,
    pub momentum: f64,
    pub memory: f64,
    pub decay: f64,
    pub drift: f64,
}

impl Default for StigmaState {
    fn default() -> Self {
        Self::new()
    }
}

impl StigmaState {
    pub fn new() -> Self {
        Self {
            weight: 0.0,
            momentum: 0.0,
            memory: 0.0,
            decay: 0.001,
            drift: 0.0,
        }
    }

    pub fn step(&mut self, ndb_score: f64, dt: f64) {
        let ndb_clamped = ndb_score.clamp(0.0, 1.0);
        let safe_dt = if dt > 0.0 { dt } else { 1.0 };

        self.weight += 0.0001 * ndb_clamped * safe_dt;
        self.momentum = 0.9 * self.momentum + 0.1 * (self.weight * ndb_clamped);
        self.memory = (self.memory + self.momentum * safe_dt) * (1.0 - self.decay);
        self.drift += (ndb_clamped - self.drift) * 0.01 * safe_dt;
    }
}
