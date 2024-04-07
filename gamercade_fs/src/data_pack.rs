use serde::{Deserialize, Serialize};

use base64::{engine::GeneralPurpose, Engine};
const BASE64ENGINE: GeneralPurpose = base64::engine::general_purpose::STANDARD;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct DataPack {
    #[serde(serialize_with = "ser_data_pack", deserialize_with = "de_data_pack")]
    pub data: Vec<u8>,
}

pub(crate) fn de_data_pack<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
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

    Ok(bytes)
}

pub(crate) fn ser_data_pack<S>(data: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let data: Vec<u8> = data.iter().flat_map(|x| x.to_be_bytes()).collect();
    if serializer.is_human_readable() {
        let data = BASE64ENGINE.encode(data);
        serializer.serialize_str(&data)
    } else {
        serializer.serialize_bytes(&data)
    }
}
