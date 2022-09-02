mod fm;
mod index_interpolator;
mod instrument_data_definition;
mod sampler;
mod wavetable;

pub use fm::*;
pub use index_interpolator::*;
pub use instrument_data_definition::*;
pub use sampler::*;
pub use wavetable::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstrumentKind {
    Sampler,
    FMSynth,
    Wavetable,
}

pub(crate) fn de_audio_data<'de, D, T>(deserializer: D) -> Result<Box<[T]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    if deserializer.is_human_readable() {
        let text: String = serde::Deserialize::deserialize(deserializer)?;
        let bytes = base64::decode(&text).map_err(serde::de::Error::custom)?;
        let bytes: Vec<T> = unsafe { std::mem::transmute(bytes) };
        Ok(bytes.into_boxed_slice())
    } else {
        let bytes: Vec<u8> = serde::Deserialize::deserialize(deserializer)?;
        let bytes: Vec<T> = unsafe { std::mem::transmute(bytes) };
        Ok(bytes.into_boxed_slice())
    }
}

pub(crate) fn ser_audio_data<S, T>(data: &[T], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let data: &[u8] = unsafe { std::mem::transmute(data) };
    if serializer.is_human_readable() {
        let data = base64::encode(data);
        serializer.serialize_str(&data)
    } else {
        serializer.serialize_bytes(data)
    }
}
