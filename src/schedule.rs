use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullResponse {
    #[serde(flatten)]
    pub regions: HashMap<String, RegionWrapper>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegionWrapper {
    pub dsos: HashMap<String, DsoWrapper>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DsoWrapper {
    pub groups: ProbableOutageSchedules, // your existing struct
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ScheduleType {
    Planned,
    Probable
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OutageSchedules {
    #[serde(rename = "1.1")]
    pub one_one: OutageGroup,
    
    #[serde(rename = "1.2")]
    pub one_two: OutageGroup,

    #[serde(rename = "2.1")]
    pub two_one: OutageGroup,

    #[serde(rename = "2.2")]
    pub two_two: OutageGroup,

    #[serde(rename = "3.1")]
    pub three_one: OutageGroup,

    #[serde(rename = "3.2")]
    pub three_two: OutageGroup,

    #[serde(rename = "4.1")]
    pub four_one: OutageGroup,

    #[serde(rename = "4.2")]
    pub four_two: OutageGroup,

    #[serde(rename = "5.1")]
    pub five_one: OutageGroup,

    #[serde(rename = "5.2")]
    pub five_two: OutageGroup,
    
    #[serde(rename = "6.1")]
    pub six_one: OutageGroup,

    #[serde(rename = "6.2")]
    pub six_two: OutageGroup
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProbableOutageSchedules {
    #[serde(rename = "1.1")]
    pub one_one: ProbableOutageGroup,
    
    #[serde(rename = "1.2")]
    pub one_two: ProbableOutageGroup,

    #[serde(rename = "2.1")]
    pub two_one: ProbableOutageGroup,

    #[serde(rename = "2.2")]
    pub two_two: ProbableOutageGroup,

    #[serde(rename = "3.1")]
    pub three_one: ProbableOutageGroup,

    #[serde(rename = "3.2")]
    pub three_two: ProbableOutageGroup,

    #[serde(rename = "4.1")]
    pub four_one: ProbableOutageGroup,

    #[serde(rename = "4.2")]
    pub four_two: ProbableOutageGroup,

    #[serde(rename = "5.1")]
    pub five_one: ProbableOutageGroup,

    #[serde(rename = "5.2")]
    pub five_two: ProbableOutageGroup,
    
    #[serde(rename = "6.1")]
    pub six_one: ProbableOutageGroup,

    #[serde(rename = "6.2")]
    pub six_two: ProbableOutageGroup
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OutageGroup {
    pub today: OutageSchedule,
    pub tomorrow: OutageSchedule,
    pub updated_on: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ProbableOutageGroup {
    pub slots: HashMap<String, Vec<Slot>>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OutageSchedule {
    pub slots: Vec<Slot>
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Slot {
    pub start: u64,
    pub end: u64,

    #[serde(rename = "type")]
    pub kind: SlotType,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum SlotType {
    NotPlanned,
    Definite,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Region {
    pub has_cities: bool,

    pub dsos: Vec<Dsos>,

    pub id: u32,
    pub value: String
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Dsos {
    pub id: u32,
    pub name: String,
}
