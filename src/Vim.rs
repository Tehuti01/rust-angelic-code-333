pub enum VimMotion {
    Left,
    Down,
    Up,
    Right,
    WordForward,
    WordBackward,
    LineStart,
    LineEnd,
    Top,
    Bottom,
}

pub enum VimOperator {
    Delete,
    Change,
    Yank,
}

pub struct VimState {
    pub cursor_pos: (usize, usize),
    pub mode: VimMode,
}

pub enum VimMode {
    Normal,
    Insert,
    Visual,
}

impl VimState {
    pub fn new() -> Self {
        Self {
            cursor_pos: (0, 0),
            mode: VimMode::Normal,
        }
    }

    pub fn apply_motion(&mut self, motion: VimMotion) {
        match motion {
            VimMotion::Left => if self.cursor_pos.0 > 0 { self.cursor_pos.0 -= 1 },
            VimMotion::Right => self.cursor_pos.0 += 1,
            VimMotion::Up => if self.cursor_pos.1 > 0 { self.cursor_pos.1 -= 1 },
            VimMotion::Down => self.cursor_pos.1 += 1,
            _ => {} // Remaining motions to be implemented
        }
    }
}
