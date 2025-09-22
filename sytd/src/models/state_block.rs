use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StateBlock {
    Idle,
    InProgress,
    Paused,
    Finished,
}
