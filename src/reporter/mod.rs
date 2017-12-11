extern crate serde;
use serde::ser::{Serialize, Serializer, SerializeStruct};
extern crate serde_json;
use serde_json::Error;
use std::sync::{Arc, Mutex};
use ::Digits;
mod ticker_tape;
mod spinner;
mod json;

mod prelude {
  pub use ticker_tape::*;
  pub use spinner::*;
  pub use json::*;
}

pub struct ReportData {
  pub cores: u8,
  pub chunk: usize,
  pub cluster: Option<(usize,usize)>,
  pub character_set: String,
  pub start_time: SystemTime,
  pub start_at: String,
  pub adjacent_limit: Option<u8>,
  pub iterations: &usize, // Atomic global
  pub five_min_progress: Arc<Mutex<(usize, String)>>, // Mutex try_lock
}

impl Serialize for ReportData {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer {

    let mut struct_fields = 8;
    if self.cluster.is_some() { struct_fields += 2; }
    if self.adjacent_limit.is_some() { struct_fields += 1; }

    let mut state = serializer.serialize_struct("ReportData", struct_fields)?;
    state.serialize_field("cores", &self.cores)?;
    state.serialize_field("chunk", &self.chunk)?;
    if let Some(node, cluster_size) = self.cluster {
      state.serialize_field("cluster_node", &node)?;
      state.serialize_field("cluster_size", &cluster_size)?;
    }
    state.serialize_field("character_set", &self.character_set)?;
    state.serialize_field("start_time", &self.start_time)?;
    state.serialize_field("start_at", &self.start_at)?;
    if let Some(adj) = self.adjacent_limit {
      state.serialize_field("adjacent_limit", &self.adjacent_limit)?;
    }
    state.serialize_field("iterations", &self.iterations.load(Ordering::SeqCst))?;
    let (five_min_iters, last_string) = self.five_min_progress.lock().unwrap();
    state.serialize_field("last_five_minute_iterations", &five_min_iters)?;
    state.serialize_field("last_attempt", &last_string)?;
    state.end()
  }
}

pub trait Reporter {
  fn report(data: ReportData);
}
