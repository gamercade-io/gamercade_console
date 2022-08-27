use gamercade_audio::{Chain, InstrumentDataDefinition, Phrase, Sfx, Song, SoundRom};
use gamercade_sound_engine::{InstrumentDefinition, InstrumentDefinitionKind, SoundRomInstance};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSoundData {
    songs: Vec<EditorAudioDataEntry<Song>>,
    chains: Vec<EditorAudioDataEntry<Chain>>,
    phrases: Vec<EditorAudioDataEntry<Phrase>>,
    instruments: Vec<EditorAudioDataEntry<InstrumentDataDefinition>>,
    sfx: Vec<EditorAudioDataEntry<Sfx>>,
}

impl Default for EditorSoundData {
    fn default() -> Self {
        let sound_rom = SoundRom::default();
        Self {
            songs: from_rom(&sound_rom.songs, "Song"),
            chains: from_rom(&sound_rom.chains, "Chains"),
            phrases: from_rom(&sound_rom.phrases, "Phrases"),
            instruments: from_rom(&sound_rom.instruments, "Instruments"),
            sfx: from_rom(&sound_rom.sfx, "Sfx"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EditorAudioDataEntry<T> {
    name: String,
    data: T,
}

fn extract_data<T: Clone>(target: &[EditorAudioDataEntry<T>]) -> Box<[T]> {
    target
        .iter()
        .map(|x| x.data.clone())
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

fn from_rom<T: Clone>(target: &[T], name: &str) -> Vec<EditorAudioDataEntry<T>> {
    target
        .iter()
        .enumerate()
        .map(|(index, item)| EditorAudioDataEntry {
            name: format!("{} {}", name, index),
            data: item.clone(),
        })
        .collect::<Vec<_>>()
}

impl From<&EditorSoundData> for SoundRom {
    fn from(data: &EditorSoundData) -> Self {
        Self {
            songs: extract_data(&data.songs),
            chains: extract_data(&data.chains),
            phrases: extract_data(&data.phrases),
            instruments: extract_data(&data.instruments),
            sfx: extract_data(&data.sfx),
        }
    }
}

impl From<&EditorSoundData> for SoundRomInstance {
    fn from(data: &EditorSoundData) -> Self {
        Self {
            songs: extract_data(&data.songs),
            chains: extract_data(&data.chains),
            phrases: extract_data(&data.phrases),
            instrument_bank: data
                .instruments
                .iter()
                .enumerate()
                .map(|(id, instrument)| InstrumentDefinition {
                    id,
                    kind: InstrumentDefinitionKind::from(instrument.data.clone()),
                })
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            sfx: extract_data(&data.sfx),
        }
    }
}
