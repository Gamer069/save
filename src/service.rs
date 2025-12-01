use std::time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{conf::Conf, schedule::{OutageGroup, ProbableOutageGroup, SlotType}};

pub fn service(schedule: &OutageGroup, probable_schedule: Option<ProbableOutageGroup>, conf: &Conf) {
    let overrides = conf.overrides.as_ref();
    let loop_delay = overrides.and_then(|o| o.loop_delay).unwrap_or(Duration::from_secs(1));

    let time_before_save = overrides
        .and_then(|o| o.time_before_save)
        .unwrap_or(Duration::from_mins(15));


    if conf.save.test {
        save(&conf, time_before_save);
        std::process::exit(0);
    }

    loop {
        let time = SystemTime::now();

        for slot in &schedule.today.slots {
            if slot.kind != SlotType::Definite {
                continue;
            }

            let start = UNIX_EPOCH + Duration::from_mins(slot.start);
            let end = UNIX_EPOCH + Duration::from_mins(slot.end);

            let time_to_cmp = time + time_before_save;

            if time_to_cmp >= start && time_to_cmp <= end {
                save(&conf, time_before_save);
            }
        }

        if let Some(ref probable_schedule) = probable_schedule {
            let probable_slot = probable_schedule.slots.values().flat_map(|v| v.iter().cloned()).collect::<Vec<_>>();
            for slot in probable_slot {
                if slot.kind != SlotType::Definite {
                    continue;
                }

                let start = UNIX_EPOCH + Duration::from_mins(slot.start);
                let end = UNIX_EPOCH + Duration::from_mins(slot.end);

                let time_to_cmp = time + time_before_save;

                if time_to_cmp >= start && time_to_cmp <= end {
                    save(&conf, time_before_save);
                }
            }
        }

        std::thread::sleep(loop_delay);
    }
}

pub fn save(conf: &Conf, time_before_save: Duration) {
    for preset in &conf.save.preset {
        preset.save(time_before_save);
    }
}
