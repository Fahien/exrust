use chrono::{prelude::*, LocalResult};

fn main() {
    let duration = std::time::Duration::default();
    println!("Default duration: {:?}", duration);

    let a = std::time::Duration::new(1, 2);
    println!("One sec + one ns duration: {:?}", a);
    println!("Double (one sec + one ns) duration: {:?}", a + a);
    println!("Double (one sec + one ns) duration: {:?}", 2 * a);

    let now = chrono::Local::now();
    println!("{}", now);
    let utc_now = chrono::Utc::now();
    println!("{}", utc_now);
    let new_time = Utc
        .ymd(2021, 02, 17)
        .and_hms(22, 50, 1)
        .with_nanosecond(1)
        .unwrap();
    println!("{}", new_time);

    let single_utc = LocalResult::Single(new_time);
    let single_lcl = LocalResult::Single(now);
    println!("{:?}", single_utc);
    println!("{:?}", single_lcl);

    println!(
        "{}y {}m {}d - {}:{}:{}.{} {:?}",
        now.year(),
        now.month(),
        now.day(),
        now.hour(),
        now.minute(),
        now.second(),
        now.nanosecond(),
        now.timezone()
    );
    let parsed_dt: DateTime<Utc> = "2014-11-28 12:00:09 UTC".parse().unwrap();
    println!("{}", parsed_dt);
}
