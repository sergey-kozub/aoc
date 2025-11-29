use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
enum GuardAction {
  BeginShift(u32),
  FallAsleep,
  WakeUp,
}

#[derive(Debug)]
struct LogEntry {
  timestamp: u32,
  action: GuardAction,
}

#[derive(Debug)]
struct GuardStats {
  minutes: u32,
  hist: Vec<u32>,
}

impl LogEntry {
  fn parse(re: &Regex, line: &str) -> LogEntry {
    let caps = re.captures(line).unwrap();
    let timestamp = caps[1].replace(|c: char| !c.is_ascii_digit(), "")
      .parse::<u32>().unwrap();
    let id = caps.get(3).and_then(|m| m.as_str().parse::<u32>().ok());
    let action = match caps[2].chars().nth(0).unwrap() {
      'G' => GuardAction::BeginShift(id.unwrap()),
      'f' => GuardAction::FallAsleep,
      'w' => GuardAction::WakeUp,
      _ => panic!(),
    };
    LogEntry { timestamp, action }
  }

  fn parse_all(text: &str) -> Vec<LogEntry> {
    let re = Regex::new(concat!(
      r"\[\d{4}-(\d{2}-\d{2} \d{2}:\d{2})\] ",
      r"(Guard #(\d+) begins shift|falls asleep|wakes up)"
    )).unwrap();
    let mut data: Vec<LogEntry> = text.lines()
      .map(|s| LogEntry::parse(&re, s)).collect();
    data.sort_by_key(|e| e.timestamp);
    data
  }

  fn process(log: &[LogEntry]) -> HashMap<u32, GuardStats> {
    let mut stats = HashMap::<u32, GuardStats>::new();
    let mut id = 0_u32;
    let mut start = 0_u32;
    for entry in log {
      let minute = entry.timestamp % 100;
      match entry.action {
        GuardAction::BeginShift(x) => id = x,
        GuardAction::FallAsleep => start = minute,
        GuardAction::WakeUp => {
          let e = stats.entry(id).or_insert_with(
            || GuardStats { minutes: 0, hist: vec![0; 60] });
          e.minutes += minute - start;
          for i in start..minute { e.hist[i as usize] += 1; }
        },
      }
    }
    stats
  }

  fn strategy_1(log: &[LogEntry]) -> u32 {
    let stats = LogEntry::process(log);
    stats.iter().max_by_key(|(_, v)| v.minutes).map(|(k, v)| {
      let max = v.hist.iter().max().unwrap();
      let pos = v.hist.iter().position(|&t| t == *max).unwrap();
      *k * pos as u32
    }).unwrap()
  }

  fn strategy_2(log: &[LogEntry]) -> u32 {
    let stats = LogEntry::process(log);
    stats.iter().map(|(k, v)| {
      let max = v.hist.iter().max().unwrap();
      let pos = v.hist.iter().position(|&t| t == *max).unwrap();
      (*k, pos as u32, *max)
    }).max_by_key(|x| x.2).map(|x| x.0 * x.1).unwrap()
  }
}

pub fn run(content: &str) {
  let data = LogEntry::parse_all(content);
  let res1 = LogEntry::strategy_1(&data);
  let res2 = LogEntry::strategy_2(&data);
  println!("{} {}", res1, res2);
}

#[cfg(test)]
mod tests {
  const TEST: &str = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

  #[test]
  fn small() {
    let data = super::LogEntry::parse_all(TEST);
    assert_eq!(super::LogEntry::strategy_1(&data), 240);
  }

  #[test]
  fn large() {
    let data = super::LogEntry::parse_all(TEST);
    assert_eq!(super::LogEntry::strategy_2(&data), 4455);
  }
}
