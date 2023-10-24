#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Idle,
    Run(u16),
}

#[derive(Debug, Clone)]
pub struct ModeManager {
    mode: Mode,
    id: u16,
}

impl ModeManager {
    pub fn new() -> Self {
        Self {
            mode: Mode::Idle,
            id: 0,
        }
    }

    pub fn is_idle(&self) -> bool {
        matches!(self.mode, Mode::Idle)
    }

    pub fn match_run_id(&self, id: u16) -> bool {
        self.mode == Mode::Run(id)
    }

    pub fn to_run(&mut self) -> u16 {
        self.id += 1;
        self.mode = Mode::Run(self.id);
        self.id
    }

    pub fn to_idle(&mut self) {
        self.mode = Mode::Idle;
    }

    pub fn try_run(&mut self) -> Option<u16> {
        if self.is_idle() {
            Some(self.to_run())
        } else {
            None
        }
    }
}
