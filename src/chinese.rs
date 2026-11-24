// Chinese calendar and astronomical calculations adapted from ICU 76.1
// astro.cpp and chnsecal.cpp. Copyright Unicode, Inc. and others.
// See UNICODE-LICENSE.txt. The approximations intentionally match PHP's ICU.

use crate::{Date, Error, calendar::date};
use chrono::Datelike;
use std::f64::consts::{PI, TAU};

const DAY_MS: f64 = 86_400_000.0;
const CHINA_OFFSET: f64 = 28_800_000.0;
const SYNODIC_MONTH: f64 = 29.530588853;

fn normalize(angle: f64) -> f64 {
    angle - TAU * (angle / TAU).floor()
}

fn signed_angle(angle: f64) -> f64 {
    normalize(angle + PI) - PI
}

fn sun(time: f64) -> (f64, f64) {
    let day = (time + 210_866_760_000_000.0) / DAY_MS - 2_447_891.5;
    let mean = normalize(
        normalize(TAU / 365.242191 * day) + 279.403303_f64.to_radians()
            - 282.768422_f64.to_radians(),
    );
    let mut eccentric = mean;
    loop {
        let delta = eccentric - 0.016713 * eccentric.sin() - mean;
        eccentric -= delta / (1.0 - 0.016713 * eccentric.cos());
        if delta.abs() <= 1e-5 {
            break;
        }
    }
    let anomaly =
        2.0 * ((eccentric / 2.0).tan() * ((1.0_f64 + 0.016713) / (1.0 - 0.016713)).sqrt()).atan();
    (normalize(anomaly + 282.768422_f64.to_radians()), mean)
}

fn moon_age(time: f64) -> f64 {
    let (solar, solar_anomaly) = sun(time);
    let day = (time + 210_866_760_000_000.0) / DAY_MS - 2_447_891.5;
    let mean_longitude = normalize((13.1763966 * day).to_radians() + 318.351648_f64.to_radians());
    let mut mean_anomaly =
        normalize(mean_longitude - (0.1114041 * day).to_radians() - 36.340410_f64.to_radians());
    let evection = 1.2739_f64.to_radians() * (2.0 * (mean_longitude - solar) - mean_anomaly).sin();
    let annual = 0.1858_f64.to_radians() * solar_anomaly.sin();
    mean_anomaly += evection - annual - 0.3700_f64.to_radians() * solar_anomaly.sin();
    let mut longitude = mean_longitude + evection + 6.2886_f64.to_radians() * mean_anomaly.sin()
        - annual
        + 0.2140_f64.to_radians() * (2.0 * mean_anomaly).sin();
    longitude += 0.6583_f64.to_radians() * (2.0 * (longitude - solar)).sin();
    let node = normalize(318.510107_f64.to_radians() - (0.0529539 * day).to_radians())
        - 0.16_f64.to_radians() * solar_anomaly.sin();
    normalize(
        ((longitude - node).sin() * 5.145366_f64.to_radians().cos())
            .atan2((longitude - node).cos())
            + node
            - solar,
    )
}

fn time_of_angle(mut start: f64, lunar: bool, next: bool) -> Result<f64, Error> {
    let period = if lunar { SYNODIC_MONTH } else { 365.242191 };
    let desired = if lunar { 0.0 } else { 3.0 * PI / 2.0 };
    let angle_at = |time| if lunar { moon_age(time) } else { sun(time).0 };
    for _ in 0..16 {
        let mut previous_angle = angle_at(start);
        let mut delta = (normalize(desired - previous_angle) + if next { 0.0 } else { -TAU })
            * (period * DAY_MS)
            / TAU;
        let mut previous_delta = delta;
        let mut time = start + delta.ceil();
        for _ in 0..64 {
            let angle = angle_at(time);
            let factor = (delta / signed_angle(angle - previous_angle)).abs();
            delta = signed_angle(desired - angle) * factor;
            if delta.abs() > previous_delta.abs() {
                break;
            }
            previous_delta = delta;
            previous_angle = angle;
            time += delta.ceil();
            if delta.abs() <= 60_000.0 {
                return Ok(time);
            }
        }
        start += (period * DAY_MS / 8.0).ceil() * if next { 1.0 } else { -1.0 };
    }
    Err(Error::Calculation(
        "Chinese astronomical convergence".into(),
    ))
}

fn new_moon(day: i32, next: bool) -> Result<i32, Error> {
    Ok(
        ((time_of_angle(f64::from(day) * DAY_MS - CHINA_OFFSET, true, next)? + CHINA_OFFSET)
            / DAY_MS)
            .floor() as i32,
    )
}

fn solstice(year: i32) -> Result<i32, Error> {
    let day = date(year, 12, 1)?.num_days_from_ce() - 719_163;
    Ok(
        ((time_of_angle(f64::from(day) * DAY_MS - CHINA_OFFSET, false, true)? + CHINA_OFFSET)
            / DAY_MS)
            .floor() as i32,
    )
}

fn months_between(first: i32, last: i32) -> i32 {
    (f64::from(last - first) / SYNODIC_MONTH).round() as i32
}

fn solar_term(day: i32) -> i32 {
    let term = ((6.0 * sun(f64::from(day) * DAY_MS - CHINA_OFFSET).0 / PI) as i32 + 2) % 12;
    if term < 1 { term + 12 } else { term }
}

fn no_major_term(moon: i32) -> Result<bool, Error> {
    Ok(solar_term(moon) == solar_term(new_moon(moon + 25, true)?))
}

fn leap_month_between(first: i32, mut last: i32) -> Result<bool, Error> {
    while last >= first {
        if no_major_term(last)? {
            return Ok(true);
        }
        last = new_moon(last - 25, false)?;
    }
    Ok(false)
}

fn new_year(year: i32) -> Result<i32, Error> {
    let first = new_moon(solstice(year - 1)? + 1, true)?;
    let second = new_moon(first + 25, true)?;
    let last = new_moon(solstice(year)? + 1, false)?;
    if months_between(first, last) == 12 && (no_major_term(first)? || no_major_term(second)?) {
        return new_moon(second + 25, true);
    }
    Ok(second)
}

pub(crate) fn to_gregorian(year: i32, month: i32, day: i32) -> Result<Date, Error> {
    let mut moon = new_moon(new_year(year)? + (month - 1) * 29, true)?;
    let gregorian_year = Date::from_num_days_from_ce_opt(moon + 719_163)
        .ok_or(Error::UnsupportedYear)?
        .year();
    let mut after = solstice(gregorian_year)?;
    let before = if moon < after {
        solstice(gregorian_year - 1)?
    } else {
        let before = after;
        after = solstice(gregorian_year + 1)?;
        before
    };
    let first = new_moon(before + 1, true)?;
    let last = new_moon(after + 1, false)?;
    let this = new_moon(moon + 1, false)?;
    let has_leap_month = months_between(first, last) == 12;
    let mut calculated_month = months_between(first, this);
    if has_leap_month && leap_month_between(first, this)? {
        calculated_month -= 1;
    }
    if calculated_month < 1 {
        calculated_month += 12;
    }
    let is_leap = has_leap_month
        && no_major_term(this)?
        && !leap_month_between(first, new_moon(this - 25, false)?)?;
    if calculated_month != month || is_leap {
        moon = new_moon(moon + 25, true)?;
    }
    Date::from_num_days_from_ce_opt(moon + 719_163 + day - 1).ok_or(Error::UnsupportedYear)
}
