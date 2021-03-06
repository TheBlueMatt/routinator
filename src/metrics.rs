//! Monitoring metrics.

use std::sync::Arc;
use chrono::{DateTime, Utc};
use rpki::tal::TalInfo;


//------------ Metrics -------------------------------------------------------

#[derive(Clone, Debug)]
pub struct Metrics {
    /// Time when these metrics have been collected.
    time: DateTime<Utc>,

    /// Per-TAL metrics.
    tals: Vec<TalMetrics>,
}

impl Metrics {
    pub fn new() -> Self {
        Metrics {
            time: Utc::now(),
            tals: Vec::new()
        }
    }

    pub fn push_tal(&mut self, tal: TalMetrics) {
        self.tals.push(tal)
    }

    pub fn timestamp(&self) -> i64 {
        self.time.timestamp()
    }

    pub fn tals(&self) -> &[TalMetrics] {
        &self.tals
    }

    pub fn log(self) {
        info!("Summary:");
        for tal in self.tals {
            info!(
                "{}: {} valid ROAs, {} VRPs.",
                tal.tal.name(), tal.roas, tal.vrps
            )
        }
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}


//------------ TalMetrics ----------------------------------------------------

#[derive(Clone, Debug)]
pub struct TalMetrics {
    /// The TAL.
    pub tal: Arc<TalInfo>,

    /// Number of ROAs.
    pub roas: u32,

    /// Number of VRPs.
    pub vrps: u32,
}

impl TalMetrics {
    pub fn new(tal: Arc<TalInfo>) -> Self {
        TalMetrics {
            tal,
            roas: 0,
            vrps: 0
        }
    }
}

