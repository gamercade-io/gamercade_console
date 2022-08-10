#[derive(Clone, PartialEq, Debug)]
pub(crate) enum EnvelopePhase {
    Attack,
    Decay,
    Sustain,
    Release,
    Off,
}
