#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum EnvelopePhase {
    Attack,
    Decay,
    Sustain,
    Release,
    Off,
}

impl EnvelopePhase {
    pub fn next_phase(self) -> Self {
        match self {
            EnvelopePhase::Attack => EnvelopePhase::Decay,
            EnvelopePhase::Decay => EnvelopePhase::Sustain,
            EnvelopePhase::Sustain => EnvelopePhase::Release,
            EnvelopePhase::Release => EnvelopePhase::Off,
            EnvelopePhase::Off => EnvelopePhase::Off,
        }
    }
}
