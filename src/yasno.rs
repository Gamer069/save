use crate::{conf::Conf, schedule::{self, FullResponse, OutageGroup, OutageSchedules, ProbableOutageGroup, Region}, util};

// const PLANNED_OUTAGES_URL: &str = "https://app.yasno.ua/api/blackout-service/public/shutdowns/regions/25/dsos/902/planned-outages";
const PLANNED_OUTAGES_URL: &str = "https://app.yasno.ua/api/blackout-service/public/shutdowns/regions/{region}/dsos/{dsos}/planned-outages";
const PROBABLE_OUTAGES_URL: &str = "https://app.yasno.ua/api/blackout-service/public/shutdowns/probable-outages?regionId={region}&dsoId={dsos}";
const REGIONS_URL: &str = "https://app.yasno.ua/api/blackout-service/public/shutdowns/addresses/v2/regions";

// REGION ID, DSOS ID
pub fn region(region: String, dsos: String, regions: Vec<Region>) -> (u32, u32) {
    let region = regions.iter().find(|x| x.value == region).unwrap_or_else(|| {
        eprintln!("Region doesn't exist");
        std::process::exit(-1);
    });

    let dsos = region.dsos.iter().find(|x| x.name == dsos).unwrap_or_else(|| {
        eprintln!("Dsos doesn't exist");
        std::process::exit(-1);
    });

    (region.id, dsos.id)
}

pub fn regions(conf: &Conf) -> Vec<schedule::Region> {
    let regions_url = conf
        .overrides
        .as_ref()
        .and_then(|o| o.planned_outages_url.as_deref())
        .unwrap_or(REGIONS_URL);

    let regions_res = reqwest::blocking::get(regions_url);
    let regions_json = regions_res
        .unwrap_or_else(|e| {
            eprintln!("Failed to get regions json: {e}");
            std::process::exit(-1);
        })
        .text()
        .unwrap_or_else(|e| {
            eprintln!("Didn't get text response, err: {e}");
            std::process::exit(-1);
        });
    
    let regions: Vec<schedule::Region> = serde_json::from_str(regions_json.as_str()).expect("Failed to parse regions");

    regions
}

pub fn planned_outages(conf: &Conf) -> OutageGroup {
    let mut planned_outages_url = conf
        .overrides
        .as_ref()
        .and_then(|o| o.planned_outages_url.as_deref())
        .unwrap_or(PLANNED_OUTAGES_URL);

    let region = region(conf.geo.region.clone(), conf.geo.dsos.clone(), regions(conf));
    let binding = planned_outages_url.replace("{region}", region.0.to_string().as_str()).replace("{dsos}", region.1.to_string().as_str());
    planned_outages_url = &binding;

    let outages_res = reqwest::blocking::get(planned_outages_url);
    let outages_json = outages_res
        .unwrap_or_else(|e| {
            eprintln!("Failed to get outages json: {e}");
            std::process::exit(-1);
        })
        .text()
        .unwrap_or_else(|e| {
            eprintln!("Didn't get text response, err: {e}");
            std::process::exit(-1);
        });
    
    let schedules: OutageSchedules = serde_json::from_str(outages_json.as_str()).expect("Failed to parse outage schedule");

    util::outage_group_from_str(conf.geo.group.clone(), schedules)
}

pub fn probable_outages(conf: &Conf) -> ProbableOutageGroup {
    let mut probable_outages_url = conf
        .overrides
        .as_ref()
        .and_then(|o| o.probable_outages_url.as_deref())
        .unwrap_or(PROBABLE_OUTAGES_URL);

    let region = region(conf.geo.region.clone(), conf.geo.dsos.clone(), regions(conf));
    let binding = probable_outages_url.replace("{region}", region.0.to_string().as_str()).replace("{dsos}", region.1.to_string().as_str());
    probable_outages_url = &binding;

    let outages_res = reqwest::blocking::get(probable_outages_url);
    let outages_json = outages_res
        .unwrap_or_else(|e| {
            eprintln!("Failed to get outages json: {e}");
            std::process::exit(-1);
        })
        .text()
        .unwrap_or_else(|e| {
            eprintln!("Didn't get text response, err: {e}");
            std::process::exit(-1);
        });

    let schedules_wrapper: FullResponse = serde_json::from_str(&outages_json).expect("Failed to parse probable outage schedule");

    let region_wrapper = schedules_wrapper
        .regions
        .get(&region.0.to_string())
        .unwrap_or_else(|| {
            eprintln!("Region {} not found in outage schedule", region.0);
            std::process::exit(-1);
        });

    let dso_wrapper = region_wrapper
        .dsos
        .get(&region.1.to_string())
        .unwrap_or_else(|| {
            eprintln!("DSOS {} not found in outage schedule", region.1);
            std::process::exit(-1);
        });

    util::probable_outage_group_from_str(conf.geo.group.clone(), dso_wrapper.groups.clone())
}
