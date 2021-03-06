use std::collections::{BTreeMap, HashMap};

use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::state::State;
use crate::task::Task;

/// The Message used to add a new command to the daemon.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Message {
    Add(AddMessage),
    Remove(Vec<usize>),
    Switch(SwitchMessage),
    Stash(Vec<usize>),
    Enqueue(EnqueueMessage),

    Start(StartMessage),
    Pause(PauseMessage),
    Kill(KillMessage),

    Send(SendMessage),
    EditRequest(usize),
    EditResponse(EditResponseMessage),
    Edit(EditMessage),
    Group(GroupMessage),

    Status,
    StatusResponse(State),
    Log(LogRequestMessage),
    LogResponse(BTreeMap<usize, TaskLogMessage>),
    Stream(String),
    StreamRequest(StreamRequestMessage),
    /// The boolean decides, whether the children should be get a SIGTERM as well.
    Reset(bool),
    Clean,
    DaemonShutdown,

    Success(String),
    Failure(String),

    Parallel(ParallelMessage),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AddMessage {
    pub command: String,
    pub path: String,
    pub envs: HashMap<String, String>,
    pub start_immediately: bool,
    pub stashed: bool,
    pub group: Option<String>,
    pub enqueue_at: Option<DateTime<Local>>,
    pub dependencies: Vec<usize>,
    pub ignore_aliases: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SwitchMessage {
    pub task_id_1: usize,
    pub task_id_2: usize,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EnqueueMessage {
    pub task_ids: Vec<usize>,
    pub enqueue_at: Option<DateTime<Local>>,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct StartMessage {
    pub task_ids: Vec<usize>,
    pub group: Option<String>,
    pub all: bool,
    pub children: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PauseMessage {
    pub task_ids: Vec<usize>,
    pub group: Option<String>,
    pub wait: bool,
    pub all: bool,
    pub children: bool,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct KillMessage {
    pub task_ids: Vec<usize>,
    pub group: Option<String>,
    pub default: bool,
    pub all: bool,
    pub children: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SendMessage {
    pub task_id: usize,
    pub input: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EditMessage {
    pub task_id: usize,
    pub command: String,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EditResponseMessage {
    pub task_id: usize,
    pub command: String,
    pub path: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GroupMessage {
    pub add: Option<String>,
    pub remove: Option<String>,
}

/// `err` decides, whether you should stream stderr or stdout.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StreamRequestMessage {
    pub task_id: Option<usize>,
    pub err: bool,
}

/// Request logs for specific tasks.
/// An empty task_id vector will return logs of all tasks.
/// If send_logs is false, the daemon won't send the logs
/// and the client will read logs from the local disk.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LogRequestMessage {
    pub task_ids: Vec<usize>,
    pub send_logs: bool,
}

/// Helper struct for sending tasks and their log output to the client.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TaskLogMessage {
    pub task: Task,
    pub stdout: Option<Vec<u8>>,
    pub stderr: Option<Vec<u8>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ParallelMessage {
    pub parallel_tasks: usize,
    pub group: Option<String>,
}

pub fn create_success_message<T: ToString>(text: T) -> Message {
    Message::Success(text.to_string())
}

pub fn create_failure_message<T: ToString>(text: T) -> Message {
    Message::Failure(text.to_string())
}
