pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval { min: min, max: max }
    }

    // Open interval
    pub fn contains(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    // Closed interval
    pub fn surrounds(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
}

pub const UNIVERSE: Interval = Interval {
    min: -f64::INFINITY,
    max: f64::INFINITY,
};

pub const EMPTY: Interval = Interval {
    min: f64::INFINITY,
    max: -f64::INFINITY,
};

pub const POSITIVE_HALF_LINE: Interval = Interval {
    min: 0.,
    max: f64::INFINITY,
};

pub const NEGATIVE_HALF_LINE: Interval = Interval {
    min: -f64::INFINITY,
    max: 0.,
};
