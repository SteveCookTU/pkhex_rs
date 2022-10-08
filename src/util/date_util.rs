use rand::Rng;
use std::ops::{Add, Sub};
use std::time::Duration;
use time::macros::datetime;
use time::util::days_in_year_month;
use time::{Date, Month, PrimitiveDateTime, Time};

const SECONDS_PER_DAY: usize = 60 * 60 * 24;

pub fn is_date_valid(year: i32, month: u8, day: u8) -> bool {
    !(year <= 0
        || year > 9999
        || month == 0
        || month > 12
        || day == 0
        || day > days_in_year_month(year as i32, Month::try_from(month).unwrap()))
}

pub fn epoch_2000() -> PrimitiveDateTime {
    PrimitiveDateTime::new(
        Date::from_calendar_date(2000, Month::January, 1).unwrap(),
        Time::MIDNIGHT,
    )
}

pub fn get_seconds_from_2000(date: Date, time: Time) -> usize {
    let mut seconds = date.sub(epoch_2000().date()).whole_seconds() as usize;
    seconds -= seconds % SECONDS_PER_DAY;
    seconds += time.sub(epoch_2000().time()).whole_seconds() as usize;
    seconds
}

pub fn get_date_time_2000(seconds: usize, date: &mut Date, time: &mut Time) {
    *date = epoch_2000().date().add(Duration::from_secs(seconds as u64));
    *time = epoch_2000().time().add(Duration::from_secs(seconds as u64));
}

pub fn convert_date_value_to_string(value: usize, seconds_bias: Option<usize>) -> String {
    let mut sb = String::new();
    if value >= SECONDS_PER_DAY {
        sb = format!("{}{}d ", sb, value / SECONDS_PER_DAY);
    }
    let zero: PrimitiveDateTime = datetime!(0000 - 01 - 01 0:00);
    sb = format!(
        "{}{}",
        sb,
        zero.add(Duration::from_secs(value as u64)).time()
    );
    if let Some(seconds_bias) = seconds_bias {
        sb = format!(
            "{}\nDate: {}",
            sb,
            epoch_2000()
                .add(Duration::from_secs((value + seconds_bias) as u64))
                .date()
        );
    }
    sb
}

pub fn get_random_date_within(
    start: PrimitiveDateTime,
    end: PrimitiveDateTime,
) -> PrimitiveDateTime {
    let mut r = rand::thread_rng();
    let delta = start - end;
    let bias = r.gen_range(0..(delta.whole_days() + 1)) as u64;
    start.add(Duration::from_secs(bias * SECONDS_PER_DAY as u64))
}
