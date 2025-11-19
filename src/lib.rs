use serde::Deserialize;

/// Indicate CSV row data
#[derive(Debug, Deserialize)]
pub struct Row {
    #[serde(rename = "CH1V")]
    pub press_1: f64,
    #[serde(rename = "CH2V")]
    pub press_2: f64,
    #[serde(skip)]
    _phantom_data_1: f64,
    #[serde(skip)]
    _phantom_data_2: f64,
}

pub struct Press {
    pub buf: Vec<f64>,
}

pub struct Voltage {
    pub buf: Vec<f64>,
}

impl Voltage {
    pub fn convert(self) -> Press {
        let buf = self
            .buf
            .into_iter()
            .map(|v| (v / 3.3) * 1023.0)
            .map(|ad| ad * 788.387e-6 - 0.1358)
            .collect();
        Press { buf }
    }
}

// ad_p1 * 788.387e-6 - 0.1358
// v / 3.3 * 1023
pub fn a(p1: Press, p2: Press) -> Option<f64> {
    p1.buf
        .iter()
        .zip(p2.buf)
        .map(|(p1, p2)| p1 - p2)
        .reduce(|a: f64, b| a.max(b))
}

impl Press {
    pub fn push(&mut self, t: f64) {
        self.buf.push(t)
    }
}

impl Voltage {
    pub fn new() -> Self {
        Self { buf: Vec::new() }
    }

    pub fn push(&mut self, t: f64) {
        self.buf.push(t)
    }
}
