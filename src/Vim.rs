use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operator {
    Delete,
    Change,
    Yank,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FindType {
    FSmall,
    FCap,
    TSmall,
    TCap,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TextObjScope {
    Inner,
    Around,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VimMode {
    Insert { inserted_text: String },
    Normal { command: CommandState },
    Visual,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CommandState {
    Idle,
    Count { digits: String },
    Operator { op: Operator, count: u32 },
    OperatorCount { op: Operator, count: u32, digits: String },
    OperatorFind { op: Operator, count: u32, find: FindType },
    OperatorTextObj { op: Operator, count: u32, scope: TextObjScope },
    Find { find: FindType, count: u32 },
    G { count: u32 },
    OperatorG { op: Operator, count: u32 },
    Replace { count: u32 },
    Indent { direction: IndentDirection, count: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IndentDirection {
    In,
    Out,
}

pub struct PersistentState {
    pub last_change: Option<RecordedChange>,
    pub last_find: Option<(FindType, char)>,
    pub register: String,
    pub register_is_linewise: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordedChange {
    Insert { text: String },
    Operator { op: Operator, motion: String, count: u32 },
    OperatorTextObj { op: Operator, obj_type: char, scope: TextObjScope, count: u32 },
    OperatorFind { op: Operator, find: FindType, char: char, count: u32 },
    Replace { char: char, count: u32 },
    X { count: u32 },
    ToggleCase { count: u32 },
    Indent { direction: IndentDirection, count: u32 },
}

pub struct VimState {
    pub mode: VimMode,
    pub persistent: PersistentState,
    pub cursor_pos: (usize, usize),
}

impl VimState {
    pub fn new() -> Self {
        Self {
            mode: VimMode::Normal { command: CommandState::Idle },
            persistent: PersistentState {
                last_change: None,
                last_find: None,
                register: String::new(),
                register_is_linewise: false,
            },
            cursor_pos: (0, 0),
        }
    }

    pub fn handle_key(&mut self, key: char) {
        let (new_mode, new_command_state) = match &mut self.mode {
            VimMode::Normal { command } => {
                match command {
                    CommandState::Idle => {
                        match key {
                            'i' => (Some(VimMode::Insert { inserted_text: String::new() }), None),
                            'v' => (Some(VimMode::Visual), None),
                            'd' => (None, Some(CommandState::Operator { op: Operator::Delete, count: 1 })),
                            'c' => (None, Some(CommandState::Operator { op: Operator::Change, count: 1 })),
                            'y' => (None, Some(CommandState::Operator { op: Operator::Yank, count: 1 })),
                            'h' | 'j' | 'k' | 'l' => {
                                self.apply_simple_motion(key);
                                (None, None)
                            }
                            _ => (None, None)
                        }
                    }
                    _ => (None, None)
                }
            }
            VimMode::Insert { inserted_text } => {
                if key == '\x1b' { // Esc
                    (Some(VimMode::Normal { command: CommandState::Idle }), None)
                } else {
                    inserted_text.push(key);
                    (None, None)
                }
            }
            VimMode::Visual => {
                if key == '\x1b' {
                    (Some(VimMode::Normal { command: CommandState::Idle }), None)
                } else {
                    (None, None)
                }
            }
        };

        if let Some(mode) = new_mode {
            self.mode = mode;
        } else if let Some(state) = new_command_state {
            if let VimMode::Normal { command } = &mut self.mode {
                *command = state;
            }
        }
    }

    fn apply_simple_motion(&mut self, key: char) {
        match key {
            'h' => if self.cursor_pos.0 > 0 { self.cursor_pos.0 -= 1 },
            'l' => self.cursor_pos.0 += 1,
            'k' => if self.cursor_pos.1 > 0 { self.cursor_pos.1 -= 1 },
            'j' => self.cursor_pos.1 += 1,
            _ => {}
        }
    }
}
