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

use base64::{engine::GeneralPurpose, Engine};
const BASE64ENGINE: GeneralPurpose = base64::engine::general_purpose::STANDARD;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InstrumentKind {
    Sampler,
    FMSynth,
    Wavetable,
}

pub(crate) fn de_audio_data<'de, D>(deserializer: D) -> Result<Box<[i16]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let bytes = if deserializer.is_human_readable() {
        let text: String = serde::Deserialize::deserialize(deserializer)?;
        BASE64ENGINE
            .decode(text)
            .map_err(serde::de::Error::custom)?
    } else {
        serde::Deserialize::deserialize(deserializer)?
    };
    let bytes: Vec<i16> = bytes
        .chunks_exact(2)
        .map(|slice| *bytemuck::from_bytes(slice))
        .collect();
    Ok(bytes.into_boxed_slice())
}

pub(crate) fn ser_audio_data<S>(data: &[i16], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let data: Vec<u8> = data
        .iter()
        .flat_map(|x| bytemuck::cast::<_, [u8; 2]>(*x))
        .collect();
    if serializer.is_human_readable() {
        let data = BASE64ENGINE.encode(data);
        serializer.serialize_str(&data)
    } else {
        serializer.serialize_bytes(&data)
    }
}
