// Tim Lobner

use chrono::{DateTime, FixedOffset};

/// struct to combine a bands information
pub struct Band{
    pub name: String,
    pub start_dt: DateTime<FixedOffset>,
    pub end_dt: DateTime<FixedOffset>,
    pub stage: String 
}
