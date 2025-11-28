use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutageGroup {
    pub today: OutageSchedule,
    pub tomorrow: OutageSchedule,
    pub updated_on: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OutageSchedule {
    pub slots: Vec<Slot>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Slot {
    pub start: u64,
    pub end: u64,

    #[serde(rename = "type")]
    pub kind: SlotType,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum SlotType {
    NotPlanned,
    Definite,
}
