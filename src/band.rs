// Tim Lobner

use chrono::NaiveDateTime;

/// struct to combine a bands information
pub struct Band{
    pub name: String,
    pub start_dt: NaiveDateTime, 
    pub end_dt: NaiveDateTime, 
    pub stage: String 
}
