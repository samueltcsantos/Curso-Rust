
//
// Input :: "hh:mm:ss"
// Output :: N(s)
// 
pub fn time_to_seconds(time_str: &str) -> Result<u64, &'static str>{

    let  time_parts: Vec<&str> = time_str.split(":").collect();

    if time_parts.len() != 3 {
        return Err("Invalid time format. Please use hh:mm:ss!");
    }

    let hours = time_parts[0].parse::<u64>().unwrap_or(0);
    let minutes = time_parts[1].parse::<u64>().unwrap_or(0);
    let seconds = time_parts[2].parse::<u64>().unwrap_or(0);

    if hours > 23 || minutes > 59 || seconds > 59 {
        return Err("Invalid times values!");
    }

    Ok(hours * 3600 + minutes * 60 + seconds)
}

fn main() {
    let time_str = "00:01:10";

    match time_to_seconds(time_str) {
        Ok (seconds) => {
            println!("{} has {} s", time_str, seconds );
        },
        Err(err) => {
            println!("{} ", err);
        }
    }
}
