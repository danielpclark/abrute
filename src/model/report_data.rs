// Copyright 2017-2018 Daniel P. Clark & other abrute Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use std::time::SystemTime;
extern crate serde;
use self::serde::ser::{Serialize, SerializeStruct, Serializer};
use std::ops::Deref;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use {ITERATIONS, SUCCESS};

#[derive(Clone)]
pub struct ReportData {
    pub cores: u8,
    pub chunk: usize,
    pub cluster: Option<(usize, usize)>,
    pub character_set: String,
    pub start_time: SystemTime,
    pub start_at: String,
    pub adjacent_limit: Option<u8>,
    pub five_min_progress: Arc<Mutex<(usize, String)>>,
}

impl Serialize for ReportData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut struct_fields = 9;
        if self.cluster.is_some() {
            struct_fields += 2;
        }
        if self.adjacent_limit.is_some() {
            struct_fields += 1;
        }

        let mut state = serializer.serialize_struct("ReportData", struct_fields)?;
        state.serialize_field("cores", &self.cores)?;
        state.serialize_field("chunk", &self.chunk)?;
        if let Some((node, cluster_size)) = self.cluster {
            state.serialize_field("cluster_node", &node)?;
            state.serialize_field("cluster_size", &cluster_size)?;
        }
        state.serialize_field("character_set", &self.character_set)?;
        state.serialize_field("start_time", &self.start_time)?;
        state.serialize_field("start_at", &self.start_at)?;
        if let Some(adj) = self.adjacent_limit {
            state.serialize_field("adjacent_limit", &adj)?;
        }
        state.serialize_field("iterations", &ITERATIONS.load(Ordering::SeqCst))?;
        let getter = self.five_min_progress.lock().unwrap();
        let &(five_min_iters, ref last_string) = getter.deref();
        state.serialize_field("last_five_minute_iterations", &five_min_iters)?;
        state.serialize_field("last_attempt", &last_string)?;
        state.serialize_field("success", &SUCCESS.load(Ordering::SeqCst))?;
        state.end()
    }
}
