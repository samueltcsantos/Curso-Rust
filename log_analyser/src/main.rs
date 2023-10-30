use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::io::Error;
use std::env;

#[derive(Clone)]
struct LogEvent {
    date: String,
    hour: String,
    username: String,
    state: String
}

impl LogEvent {

    fn new(log_line: &str) -> Option<LogEvent> {

        let parts: Vec<&str> = log_line.split_whitespace().collect();

        if parts.len() != 4 {
            return None;
        }

        Some(LogEvent {
            date: String::from(parts[0]),
            hour: String::from(parts[1]),
            username: String::from(parts[2]),
            state: String::from(parts[3])
        })

    }

    pub fn get_hour(&self) -> String {
        format!("{}", &self.hour)
    }

}

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

pub fn convert_seconds(seconds: i64) -> (i64, i64, i64, i64) {
    let days = seconds / (24 * 3600);
    let mut remain_seconds = seconds % (24 * 3600);
    let hours = remain_seconds / 3600;
    remain_seconds %= 3600;
    let minutes = remain_seconds / 60;
    remain_seconds %= 60;
    (days, hours, minutes, remain_seconds)
}

fn get_current_directory() -> Result<PathBuf, Error> {
    env::current_dir()
}

fn read_file(file_name: &str) -> Result<BufReader<File>, Error> {

    match get_current_directory() {

        Ok(current_dir) => {
    
            println!("current working directory: {:?}", current_dir);
            
            let file_path = format!("{}/logs/{}", current_dir.to_string_lossy(), file_name);
           
            let file = File::open(file_path)?;

        
            let reader = BufReader::new(file);

            Ok(reader)
        },
        Err(err) => Err(err)
    }

}

fn parse_server_log(file_name: &String) -> Result<Vec<LogEvent>, Error> {

    match read_file(file_name) {

        Ok(reader) => {

            let mut log_events: Vec<LogEvent> = Vec::new();

            for (line_number, line_result) in reader.lines().enumerate() {
        
                let line = line_result?;

                if let Some(log_event) = LogEvent::new(&line) {
                    log_events.push(log_event);
                }
                else {
                    println!("Invalid log line {}: {} ", line_number, line);
                }
             
            }
            
            Ok(log_events)
        },
        Err(err) => Err(err),
        
    }

}


fn filter_by_user(log_events: Vec<LogEvent>) -> HashMap<String, Vec<LogEvent>>{

    let mut users: HashMap<String, Vec<LogEvent>> = HashMap::new();

    for i in 0..log_events.len() {
        let event = &log_events[i];

        let username_key = &event.username;

        match users.get_mut(username_key) {

            Some(vec_events) => { 
                vec_events.push(log_events[i].clone());
            },
            _ => {
                let mut vec_events: Vec<LogEvent> = Vec::new();
                vec_events.push(log_events[i].clone());
                users.insert(event.username.clone(), vec_events);
            }

        }
    }

    users
}

//
// start_time: hh:mm:ss
// end_time: hh:mm:ss
// 
fn elapsed_time(start_time: String, end_time: String) -> i64 {

    let mut elapsed_time: i64 = 0;
    let mut start: i64 = 0;
    let mut end: i64 = 0;

    match time_to_seconds(&start_time) {

        Ok(seconds) => {
            start = seconds as i64;
        },
        Err(err) => {
            println!("Err parsing time : {:?} ", err);
        }
    }

    match time_to_seconds(&end_time) {

        Ok(seconds) => {
            end = seconds as i64;
        },
        Err(err) => {
            println!("Err parsing time : {:?} ", err);
        }
    }
    elapsed_time = end - start;

    elapsed_time
}


fn check_connection_time(log_events: &Vec<LogEvent>) -> i64 {

    let mut total_time: i64 = 0;
    let mut index = 1;

    while index < log_events.len() {
        let previous_event = &log_events[index-1];
        let current_event =  &log_events[index];

        let start_time = previous_event.get_hour();
        let end_time = current_event.get_hour();

        let elapsed_duration = elapsed_time(start_time, end_time);

        //let (days, hours, minutes, seconds) = convert_seconds(elapsed_duration);
        //println!("{}d {}h {}m {}s", days, hours, minutes, seconds);

        total_time += elapsed_duration;

        index += 2;
    }

    total_time
}


fn main() {

    let file_name = String::from("server.log");

    match parse_server_log(&file_name) {
        Ok(log_events) => {

            let users = filter_by_user(log_events);

            for (key, value) in users.iter() {
                //println!("user: {}, events number: {} ", key, value.len());
                let total_time = check_connection_time(value);
                let (days, hours, minutes, seconds) = convert_seconds(total_time);
                println!("{} :: {}d {}h {}m {}s", key ,days, hours, minutes, seconds);
            }

        },
        Err(err) => {
            println!("error analysing log file {:?}", err)
        }
    }

    
}
