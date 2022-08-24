use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum FMWaveform {
    Sine,
    InverseSine,
    HalfSine,
    InverseHalfSine,
    AlternatingSine,
    InverseAlternatingSine,
    CamelSine,
    InveseCamelSine,
}
