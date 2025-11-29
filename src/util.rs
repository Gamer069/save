use crate::schedule::{OutageGroup, OutageSchedules, ProbableOutageGroup, ProbableOutageSchedules};

pub fn outage_group_from_str(group: String, schedules: OutageSchedules) -> OutageGroup {
    match group.as_str() {
        "1.1" => {
            schedules.one_one
        },
        "1.2" => {
            schedules.one_two
        },
        "2.1" => {
            schedules.two_one
        },
        "2.2" => {
            schedules.two_two
        },
        "3.1" => {
            schedules.three_one
        },
        "3.2" => {
            schedules.three_two
        },
        "4.1" => {
            schedules.four_one
        },
        "4.2" => {
            schedules.four_two
        },
        "5.1" => {
            schedules.five_one
        },
        "5.2" => {
            schedules.five_two
        },
        "6.1" => {
            schedules.six_one
        },
        "6.2" => {
            schedules.six_two
        },
        _ => {
            eprintln!("Invalid group {}", group.as_str());
            std::process::exit(-1);
        }
    }
}

pub fn probable_outage_group_from_str(group: String, schedules: ProbableOutageSchedules) -> ProbableOutageGroup {
    match group.as_str() {
        "1.1" => {
            schedules.one_one
        },
        "1.2" => {
            schedules.one_two
        },
        "2.1" => {
            schedules.two_one
        },
        "2.2" => {
            schedules.two_two
        },
        "3.1" => {
            schedules.three_one
        },
        "3.2" => {
            schedules.three_two
        },
        "4.1" => {
            schedules.four_one
        },
        "4.2" => {
            schedules.four_two
        },
        "5.1" => {
            schedules.five_one
        },
        "5.2" => {
            schedules.five_two
        },
        "6.1" => {
            schedules.six_one
        },
        "6.2" => {
            schedules.six_two
        },
        _ => {
            eprintln!("Invalid group {}", group.as_str());
            std::process::exit(-1);
        }
    }
}
