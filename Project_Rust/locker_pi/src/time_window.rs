/*#[derive(Debug)]
pub enum Period { Am, Pm, }

impl Period {
    pub fn value(&self) -> String {
        match *self {
            Period::Am => "am".to_string(),
            Period::Pm => "pm".to_string(),
        }
    }
}*/

#[derive(Debug)]
pub struct Time {
    hour: u8,
    minute: u8,
    period: String,
}

#[derive(Debug)]
pub enum TimeError { 
    OutOfRange,
    NotANumber,
    NotAmOrPm,
    InvalidFormat,
}

impl Time {

    pub fn new(hour: u8, minute: u8, period: String) -> Time {
        Time { 
            hour,
            minute,
            period,
        }
    }

    
    pub fn new_validated(time: String) -> Result<Time, TimeError> {
        if size(&time) != 7 {
            return Err(TimeError::InvalidFormat);
        }

        // parse hour and minute
        let hr = time[..2].parse::<u8>();
        match hr {
            Err(_e) => return Err(TimeError::NotANumber),
            Ok(_e) => {},
        }
        let hr = hr.unwrap();

        let min = time[3..5].parse::<u8>();
        match min {
            Err(_e) => return Err(TimeError::NotANumber),
            Ok(_e) => {},
        }
        let min = min.unwrap();

        // check hour-minute separator
        let colon = time[2..3].to_string();
        if &colon != ":" {
            return Err(TimeError::InvalidFormat);
        }

        // check am/pm
        let period = time[5..].to_string();
        if &period != "am" || &period != "pm" {
            return Err(TimeError::NotAmOrPm);
        }

        // check hour/minute ranges
        if hr == 0 || hr > 12 || min >= 60 {
            return Err(TimeError::OutOfRange);
        }
        
        // return
        Ok( Time::new(hr, min, period) )
    }

    pub fn to_string(&self) -> String {
        format!("{}:{}{}", 
            self.hour, 
            self.minute, 
            self.period)
    }
}

fn size(string: &String) -> usize {
    string.chars().count()
}
