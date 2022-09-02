use gamercade_audio::{InstrumentId, NoteId, INSTRUMENTS_MAX_COUNT, TOTAL_NOTES_COUNT};

pub(crate) enum TrackerEditCommand {
    EditRow(TrackerEditRowCommand),
    EditEntry(TrackerEditEntryCommand),
}

pub(crate) enum TrackerEditRowCommand {
    Insert,
    Delete,
}

pub(crate) enum TrackerEditEntryCommand {
    Add(usize),
    Sub(usize),
}

pub(crate) trait HandleTrackerEditEntryCommand {
    fn handle_command(&mut self, command: TrackerEditEntryCommand) {
        match command {
            TrackerEditEntryCommand::Add(amount) => self.add(amount),
            TrackerEditEntryCommand::Sub(amount) => self.sub(amount),
        }
    }

    fn add(&mut self, amount: usize);
    fn sub(&mut self, amount: usize);
}

impl HandleTrackerEditEntryCommand for NoteId {
    fn add(&mut self, amount: usize) {
        self.0 = (TOTAL_NOTES_COUNT - 1).min(self.0 + amount)
    }

    fn sub(&mut self, amount: usize) {
        self.0 = self.0.saturating_sub(amount)
    }
}

impl HandleTrackerEditEntryCommand for InstrumentId {
    fn add(&mut self, amount: usize) {
        self.0 = (INSTRUMENTS_MAX_COUNT - 1).min(self.0 + amount)
    }

    fn sub(&mut self, amount: usize) {
        self.0 = self.0.saturating_sub(amount)
    }
}

impl HandleTrackerEditEntryCommand for u8 {
    fn add(&mut self, amount: usize) {
        *self = self.saturating_add(amount as u8)
    }

    fn sub(&mut self, amount: usize) {
        *self = self.saturating_sub(amount as u8)
    }
}
