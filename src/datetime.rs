#[derive(Debug, PartialEq, Clone)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub microsecond: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Offset {
    pub sign: i8, // 1 for +, -1 for -
    pub hour: u8,
    pub minute: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Datetime {
    OffsetDateTime {
        date: Date,
        time: Time,
        offset: Offset,
    },
    LocalDateTime {
        date: Date,
        time: Time,
    },
    LocalDate(Date),
    LocalTime(Time),
}

impl Datetime {
    /// Hand-rolled RFC 3339 (TOML) datetime parser.
    /// This avoids the overhead of Regex used in the Python version.
    pub fn parse(s: &str) -> Option<Self> {
        let bytes = s.as_bytes();

        // Try parsing as Local Time first
        if bytes.len() >= 8 && bytes[2] == b':' {
            if let Some(time) = parse_time(bytes) {
                return Some(Datetime::LocalTime(time));
            }
        }

        // Must start with a Date (YYYY-MM-DD)
        if bytes.len() < 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return None;
        }

        let date = Date {
            year: parse_digits(&bytes[0..4])? as u16,
            month: parse_digits(&bytes[5..7])? as u8,
            day: parse_digits(&bytes[8..10])? as u8,
        };

        if bytes.len() == 10 {
            return Some(Datetime::LocalDate(date));
        }

        let sep = bytes[10];
        if sep != b'T' && sep != b't' && sep != b' ' {
            return None;
        }

        // Parse time part
        let rem = &bytes[11..];
        if rem.len() < 8 {
            return None;
        }

        let mut time_end = 8;
        if rem.len() > 8 && rem[8] == b'.' {
            time_end += 1;
            while time_end < rem.len() && rem[time_end].is_ascii_digit() {
                time_end += 1;
            }
        }

        let time = parse_time(&rem[..time_end])?;
        let after_time = &rem[time_end..];

        if after_time.is_empty() {
            return Some(Datetime::LocalDateTime { date, time });
        }

        // Parse Offset
        if after_time[0] == b'Z' || after_time[0] == b'z' {
            return Some(Datetime::OffsetDateTime {
                date,
                time,
                offset: Offset {
                    sign: 1,
                    hour: 0,
                    minute: 0,
                },
            });
        }

        if after_time.len() == 6
            && (after_time[0] == b'+' || after_time[0] == b'-')
            && after_time[3] == b':'
        {
            let sign = if after_time[0] == b'+' { 1 } else { -1 };
            let hour = parse_digits(&after_time[1..3])? as u8;
            let minute = parse_digits(&after_time[4..6])? as u8;
            return Some(Datetime::OffsetDateTime {
                date,
                time,
                offset: Offset { sign, hour, minute },
            });
        }

        None
    }
}

fn parse_time(bytes: &[u8]) -> Option<Time> {
    if bytes.len() < 8 || bytes[2] != b':' || bytes[5] != b':' {
        return None;
    }
    let hour = parse_digits(&bytes[0..2])? as u8;
    let minute = parse_digits(&bytes[3..5])? as u8;
    let second = parse_digits(&bytes[6..8])? as u8;

    let mut microsecond = 0;
    if bytes.len() > 9 && bytes[8] == b'.' {
        let frac = &bytes[9..];
        let mut val = 0;
        let mut multiplier = 100_000;
        for (i, &b) in frac.iter().enumerate() {
            if i >= 6 {
                break;
            } // limit to microseconds
            val += (b - b'0') as u32 * multiplier;
            multiplier /= 10;
        }
        microsecond = val;
    }

    Some(Time {
        hour,
        minute,
        second,
        microsecond,
    })
}

fn parse_digits(bytes: &[u8]) -> Option<u32> {
    let mut acc = 0;
    for &b in bytes {
        if b.is_ascii_digit() {
            acc = acc * 10 + (b - b'0') as u32;
        } else {
            return None;
        }
    }
    Some(acc)
}
