use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{array, thread};

use crossbeam_channel::{Receiver, Sender};
use rodio::OutputStream;

use crate::{
    initialize_globals, BgmState, ChainId, ChainPlayback, ChainState, InstrumentInstance, SongId,
    SongPlayback, SoundRom, SoundRomInstance, TrackerState, PHRASE_STEPS_PER_BEAT, SFX_CHANNELS,
    SONG_TRACK_CHANNELS,
};

pub struct SoundEngine {
    pub rom: Arc<SoundRomInstance>,
    sender: Sender<TickerCommand>,
    _stream: OutputStream,
}

impl SoundEngine {
    /// Generates a new SoundEngine
    pub fn new(rom: SoundRom) -> Self {
        initialize_globals();

        // Build the RomInstance
        let rom = Arc::new(SoundRomInstance::new(rom));

        let (sender, _stream) = spawn_ticker_runner(&rom);

        Self {
            rom,
            sender,
            _stream,
        }
    }

    /// Plays the Bgm. If None is passed, instead will mute the bgm.
    pub fn play_bgm(&self, song: Option<SongId>) {
        self.sender.try_send(TickerCommand::PlayBgm(song)).unwrap();
    }

    /// Plays the SFX on the given channel. If None is passed,
    /// instead will mute the channel.
    pub fn play_sfx(&self, chain: Option<ChainId>, channel: usize) {
        self.sender
            .try_send(TickerCommand::PlaySfx {
                index: channel,
                chain,
            })
            .unwrap();
    }

    /// Sets the entire audio thread to the passed in state. Useful to
    /// reset all sounds to a specific point in time
    pub fn set_to_state(&self, full_state: TrackerState) {
        self.sender
            .try_send(TickerCommand::TrackerState(Box::new(full_state)))
            .unwrap();
    }

    /// Sets the bgm audio to the passed in state. Useful to
    /// reset to a specific point in time
    pub fn set_to_bgm_state(&self, bgm_state: BgmState) {
        self.sender
            .try_send(TickerCommand::BgmState(Box::new(bgm_state)))
            .unwrap();
    }

    /// Sets the chain audio to the passed in state. Useful to
    /// reset to a specific point in time
    pub fn set_to_chain_state(&self, chain_state: ChainState, index: usize) {
        self.sender
            .try_send(TickerCommand::ChainState { index, chain_state })
            .unwrap();
    }
}

impl Drop for SoundEngine {
    fn drop(&mut self) {
        self.sender.try_send(TickerCommand::Shutdown).unwrap();
    }
}

enum TickerCommand {
    PlayBgm(Option<SongId>),
    PlaySfx {
        index: usize,
        chain: Option<ChainId>,
    },
    TrackerState(Box<TrackerState>),
    BgmState(Box<BgmState>),
    ChainState {
        index: usize,
        chain_state: ChainState,
    },
    Shutdown,
}

struct TickerRunner {
    rom: Arc<SoundRomInstance>,
    receiver: Receiver<TickerCommand>,
    bgm: SongPlayback,
    sfx: [ChainPlayback; SFX_CHANNELS],
    ticker: Ticker,
}

struct Ticker {
    remaining: [i64; SFX_CHANNELS + 1],
    reset: [i64; SFX_CHANNELS + 1],
}

impl Default for Ticker {
    fn default() -> Self {
        Self {
            remaining: array::from_fn(|_| i64::MAX),
            reset: array::from_fn(|_| i64::MAX),
        }
    }
}

impl Ticker {
    fn tick(&mut self, micros: i64) -> [bool; SFX_CHANNELS + 1] {
        let mut iter = self.remaining.iter_mut().zip(self.reset.iter());

        array::from_fn(|_| {
            let pair = iter.next().unwrap();
            *pair.0 -= micros;
            if *pair.0 <= 0 {
                *pair.0 = *pair.1;
                true
            } else {
                false
            }
        })
    }
}

impl TickerRunner {
    fn run(mut self) {
        let mut prev = Instant::now();

        'ticker: loop {
            // Handle incoming messages
            while let Ok(msg) = self.receiver.try_recv() {
                match msg {
                    TickerCommand::PlayBgm(song) => self.handle_play_bgm(song),
                    TickerCommand::PlaySfx { index, chain } => self.handle_play_sfx(chain, index),
                    TickerCommand::TrackerState(tracker) => self.handle_tracker_state(&tracker),
                    TickerCommand::BgmState(state) => self.handle_bgm_state(&state),
                    TickerCommand::ChainState { index, chain_state } => {
                        self.handle_chain_state(&chain_state, index)
                    }
                    TickerCommand::Shutdown => break 'ticker,
                }
            }

            // Find out which tracker targets need to be updated
            let now = Instant::now();
            let elapsed = now.duration_since(prev).as_nanos() as i64;

            let mut tick_iter = self.ticker.tick(elapsed).into_iter();

            // BGM is always first
            if tick_iter.next().unwrap() {
                self.bgm.update_tracker();
            }

            // Run through the remaining ones, which correspond to the
            // sfx chains
            tick_iter
                .zip(self.sfx.iter_mut())
                .for_each(|(update, sfx)| {
                    if update {
                        sfx.update_tracker();
                    }
                });

            prev = now;
        }
    }

    fn handle_play_bgm(&mut self, song: Option<SongId>) {
        self.bgm.set_song_id(song);

        if let Some(song) = song {
            let tick_time = 60.0 / self.rom[song].bpm / PHRASE_STEPS_PER_BEAT as f32;
            let tick_time = Duration::from_secs_f32(tick_time).as_nanos() as i64;
            self.ticker.remaining[0] = tick_time;
            self.ticker.reset[0] = tick_time;
        }
    }

    fn handle_play_sfx(&mut self, chain: Option<ChainId>, index: usize) {
        if let Some(track) = self.sfx.get_mut(index) {
            // TODO: Update ticker rate
            // TODO: How to handle effects / playback?
            track.set_chain_id(chain);
        } else {
            println!("Tried to play sound on an invalid channel");
        }
    }

    fn handle_tracker_state(&mut self, state: &TrackerState) {
        self.bgm.set_from_song_state(&state.bgm);
        self.sfx
            .iter_mut()
            .zip(state.sfx.iter())
            .for_each(|(sfx, state)| sfx.set_from_chain_state(state))
    }

    fn handle_bgm_state(&mut self, bgm_state: &BgmState) {
        self.bgm.set_from_song_state(bgm_state);
    }

    fn handle_chain_state(&mut self, chain_state: &ChainState, index: usize) {
        if let Some(track) = self.sfx.get_mut(index) {
            track.set_from_chain_state(chain_state)
        }
    }
}

fn spawn_ticker_runner(rom: &Arc<SoundRomInstance>) -> (Sender<TickerCommand>, OutputStream) {
    let (sender, receiver) = crossbeam_channel::bounded((SFX_CHANNELS + SONG_TRACK_CHANNELS) * 2);
    let (stream, stream_handle) = OutputStream::try_default().unwrap();

    // Callback closure to initialize our chains as needed
    let build_chain = |_| {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        let instance = InstrumentInstance::no_sound(receiver);

        stream_handle.play_raw(instance).unwrap();
        ChainPlayback::new(None, sender, rom)
    };

    let runner = TickerRunner {
        bgm: SongPlayback::new(None, array::from_fn(build_chain), rom),
        sfx: array::from_fn(build_chain),
        rom: rom.clone(),
        receiver,
        ticker: Ticker::default(),
    };

    thread::spawn(|| runner.run());

    (sender, stream)
}
