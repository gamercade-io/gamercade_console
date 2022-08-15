use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::{array, thread};

use crossbeam_channel::{Receiver, Sender};
use rodio::OutputStream;

use crate::{
    initialize_globals, BgmState, ChainPlayback, InstrumentInstance, Sfx, SfxPlayback, SfxState,
    SongId, SongPlayback, SoundRom, SoundRomInstance, TickerState, TrackerState,
    PHRASE_STEPS_PER_BEAT, SFX_CHANNELS, SONG_TRACK_CHANNELS,
};

pub struct SoundEngine {
    pub rom: Arc<SoundRomInstance>,
    sender: Sender<TickerCommand>,
    _stream: OutputStream,
}

impl SoundEngine {
    /// Generates a new SoundEngine
    pub fn new(rom: &SoundRom) -> Self {
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
    pub fn play_sfx(&self, sfx: Option<Sfx>, channel: usize) {
        self.sender
            .try_send(TickerCommand::PlaySfx {
                index: channel,
                sfx,
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

    /// Sets the sfx audio to the passed in state. Useful to
    /// reset to a specific point in time
    pub fn set_to_sfx_state(&self, sfx_state: SfxState, index: usize) {
        self.sender
            .try_send(TickerCommand::SfxState { index, sfx_state })
            .unwrap();
    }
}

impl Drop for SoundEngine {
    fn drop(&mut self) {
        self.sender.try_send(TickerCommand::Shutdown).unwrap();
    }
}

enum TickerCommand {
    // Play
    PlayBgm(Option<SongId>),
    PlaySfx { index: usize, sfx: Option<Sfx> },

    // Rollback
    TrackerState(Box<TrackerState>),
    BgmState(Box<BgmState>),
    SfxState { index: usize, sfx_state: SfxState },

    // Shutdown
    Shutdown,
}

struct TickerRunner {
    rom: Arc<SoundRomInstance>,
    receiver: Receiver<TickerCommand>,
    bgm: SongPlayback,
    sfx: [SfxPlayback; SFX_CHANNELS],
    ticker_set: TickerSet,
}

#[derive(Debug)]
pub(crate) struct Ticker {
    remaining: AtomicI64,
    reset: AtomicI64,
}

impl Ticker {
    pub fn as_state(&self) -> TickerState {
        TickerState {
            remaining: self.remaining.load(Ordering::Relaxed),
            reset: self.reset.load(Ordering::Relaxed),
        }
    }

    pub fn write_from_state(&self, state: &TickerState) {
        self.remaining.store(state.remaining, Ordering::Relaxed);
        self.reset.store(state.reset, Ordering::Relaxed);
    }
}

struct TickerSet {
    set: [Arc<Ticker>; SFX_CHANNELS + 1],
}

impl Default for TickerSet {
    fn default() -> Self {
        Self {
            set: array::from_fn(|_| {
                Arc::new(Ticker {
                    remaining: AtomicI64::new(i64::MAX),
                    reset: AtomicI64::new(i64::MAX),
                })
            }),
        }
    }
}

impl TickerSet {
    fn tick(&self, micros: i64) -> [bool; SFX_CHANNELS + 1] {
        let mut iter = self.set.iter();

        array::from_fn(|_| {
            let pair = iter.next().unwrap();
            let current = pair.remaining.load(Ordering::Relaxed);
            let current = current - micros;

            if current <= 0 {
                pair.remaining
                    .store(pair.reset.load(Ordering::Relaxed), Ordering::Relaxed);
                true
            } else {
                pair.remaining.store(current, Ordering::Relaxed);
                false
            }
        })
    }

    fn reset_ticker_for_bpm(&self, index: usize, bpm: f32) {
        let tick_time = 60.0 / bpm / PHRASE_STEPS_PER_BEAT as f32;
        let tick_time = Duration::from_secs_f32(tick_time).as_nanos() as i64;
        self.set[index]
            .remaining
            .store(tick_time, Ordering::Relaxed);
        self.set[index].reset.store(tick_time, Ordering::Relaxed);
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
                    TickerCommand::PlaySfx { index, sfx } => self.handle_play_sfx(sfx, index),
                    TickerCommand::TrackerState(tracker) => self.handle_tracker_state(&tracker),
                    TickerCommand::BgmState(state) => self.handle_bgm_state(&state),
                    TickerCommand::SfxState { index, sfx_state } => {
                        self.handle_sfx_state(&sfx_state, index)
                    }
                    TickerCommand::Shutdown => break 'ticker,
                }
            }

            // Find out which tracker targets need to be updated
            let now = Instant::now();
            let elapsed = now.duration_since(prev).as_nanos() as i64;

            let mut tick_iter = self.ticker_set.tick(elapsed).into_iter();

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
                        sfx.chain_playback.update_tracker();
                    }
                });

            prev = now;
        }
    }

    fn handle_play_bgm(&mut self, song: Option<SongId>) {
        self.bgm.set_song_id(song);

        if let Some(song) = song {
            self.ticker_set.reset_ticker_for_bpm(0, self.rom[song].bpm)
        }
    }

    fn handle_play_sfx(&mut self, sfx: Option<Sfx>, index: usize) {
        if let Some(playback) = self.sfx.get_mut(index) {
            if let Some(sfx) = sfx {
                self.ticker_set.reset_ticker_for_bpm(index + 1, sfx.bpm)
            } else {
                playback.chain_playback.set_chain_id(None);
            }
        } else {
            println!("Tried to play sound on an invalid channel");
        }
    }

    fn handle_tracker_state(&mut self, state: &TrackerState) {
        self.bgm.set_from_song_state(&state.bgm);
        self.sfx
            .iter_mut()
            .zip(state.sfx.iter())
            .for_each(|(playback, state)| playback.chain_playback.set_from_chain_state(state))
    }

    fn handle_bgm_state(&mut self, bgm_state: &BgmState) {
        self.bgm.set_from_song_state(bgm_state);
    }

    fn handle_sfx_state(&mut self, sfx_state: &SfxState, index: usize) {
        if let Some(playback) = self.sfx.get_mut(index) {
            playback.set_from_sfx_state(sfx_state)
        }
    }
}

fn spawn_ticker_runner(rom: &Arc<SoundRomInstance>) -> (Sender<TickerCommand>, OutputStream) {
    let (sender, receiver) = crossbeam_channel::bounded((SFX_CHANNELS + SONG_TRACK_CHANNELS) * 2);
    let (stream, stream_handle) = OutputStream::try_default().unwrap();

    let ticker_set = TickerSet::default();
    let mut ticker_iter = ticker_set.set.iter();

    // Prepare the Bgm Chains
    let build_bgm_chains = array::from_fn(|_| {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        let instance = InstrumentInstance::no_sound(receiver);

        stream_handle.play_raw(instance).unwrap();
        ChainPlayback::new(None, sender, rom)
    });

    let sfx = array::from_fn(|_| {
        let (sender, receiver) = crossbeam_channel::bounded(1);
        let instance = InstrumentInstance::no_sound(receiver);

        stream_handle.play_raw(instance).unwrap();
        SfxPlayback {
            chain_playback: ChainPlayback::new(None, sender, rom),
            ticker: ticker_iter.next().unwrap().clone(),
        }
    });

    let runner = TickerRunner {
        bgm: SongPlayback::new(None, build_bgm_chains, rom, ticker_iter.next().unwrap()),
        sfx,
        rom: rom.clone(),
        receiver,
        ticker_set,
    };

    thread::spawn(|| runner.run());

    (sender, stream)
}
