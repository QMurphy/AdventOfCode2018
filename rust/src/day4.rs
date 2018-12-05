extern crate chrono;

use std::str::FromStr;
use std::ops::Range;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use self::chrono::NaiveDateTime;
use self::chrono::Timelike;

static INPUT_FILE: &str = "../input/day4.txt";

pub fn part1() -> u32
{
    let input_file = File::open( INPUT_FILE ).expect( "Unable to open input file" );
    let reader = BufReader::new( input_file );

    // Build list of events
    let mut events: Vec<GuardEvent> = Vec::new();
    for line in reader.lines()
    {
        let event = line.unwrap().parse::<GuardEvent>().unwrap();
        events.push( event );
    }

    // Sort list of events
    events.sort();

    // Build a hashmap where the key is the guard and the key is a vec of ranges that they're asleep
    let mut guard_sleep_sched: HashMap<u16, Vec<Range<u16>>> = HashMap::new();
    let mut guard_on_duty: u16 = 0;
    let mut sleep_start: u16 = 0;
    for event in events
    {
        match event.event
        {
            GuardEventType::BEGIN( id ) => guard_on_duty = id,
            GuardEventType::SLEEP => sleep_start = event.timestamp.time().minute() as u16,
            GuardEventType::WAKE => {
                let sleep_end = event.timestamp.time().minute() as u16;
                let sleep_range = Range { start: sleep_start, end: sleep_end };
                
                /* This fails the borrow checker
                match guard_sleep_sched.get_mut( &guard_on_duty )
                {
                    Some( v ) => v.push( sleep_range ),
                    None => {
                        let mut v: Vec<Range<u16>> = Vec::new();
                        v.push( sleep_range );
                        guard_sleep_sched.insert( guard_on_duty, v );
                    }
                }
                */

                if guard_sleep_sched.contains_key( &guard_on_duty )
                {
                    let mut v = guard_sleep_sched.get_mut( &guard_on_duty ).unwrap();
                    v.push( sleep_range );
                }
                else
                {
                    let mut v: Vec<Range<u16>> = Vec::new();
                    v.push( sleep_range );
                    guard_sleep_sched.insert( guard_on_duty, v );
                }
            }
        }
    }
    
    // Find the guard who sleeps the most
    let mut sleepiest_guard = 0;
    let mut min_total = 0;
    for ( guard, naps ) in &guard_sleep_sched
    {
        // Sum their sleep
        let mut nap_sum = 0;
        for nap in naps
        {
            nap_sum += nap.end - nap.start;
        }

        // Check if we need to update the max
        if min_total < nap_sum
        {
            min_total = nap_sum;
            sleepiest_guard = *guard;
        }
    }
    println!( "Guard {} slept for {} min total", sleepiest_guard, min_total );

    // Find the most common minute asleep
    let naps = guard_sleep_sched.get( &sleepiest_guard ).unwrap();
    let mut min_counter: [u32; 60] = [0; 60];
    for nap in naps
    {
        for min in nap.start..nap.end
        {
            min_counter[min as usize] += 1;
        }
    }
    let min_mode = min_counter.iter().enumerate().max_by( |x, y| (x.1).cmp(y.1) ).unwrap().0;
    println!( "Most common minute asleep was {}", min_mode );

    return sleepiest_guard as u32 * min_mode as u32;
}

pub fn part2() -> u32
{
    let input_file = File::open( INPUT_FILE ).expect( "Unable to open input file" );
    let reader = BufReader::new( input_file );

    // Build list of events
    let mut events: Vec<GuardEvent> = Vec::new();
    for line in reader.lines()
    {
        let event = line.unwrap().parse::<GuardEvent>().unwrap();
        events.push( event );
    }

    // Sort list of events
    events.sort();

    // Build a hashmap where the key is the guard and the key is a vec of ranges that they're asleep
    let mut guard_sleep_sched: HashMap<u16, Vec<Range<u16>>> = HashMap::new();
    let mut guard_on_duty: u16 = 0;
    let mut sleep_start: u16 = 0;
    for event in events
    {
        match event.event
        {
            GuardEventType::BEGIN( id ) => guard_on_duty = id,
            GuardEventType::SLEEP => sleep_start = event.timestamp.time().minute() as u16,
            GuardEventType::WAKE => {
                let sleep_end = event.timestamp.time().minute() as u16;
                let sleep_range = Range { start: sleep_start, end: sleep_end };
                
                /* This fails the borrow checker
                match guard_sleep_sched.get_mut( &guard_on_duty )
                {
                    Some( v ) => v.push( sleep_range ),
                    None => {
                        let mut v: Vec<Range<u16>> = Vec::new();
                        v.push( sleep_range );
                        guard_sleep_sched.insert( guard_on_duty, v );
                    }
                }
                */

                if guard_sleep_sched.contains_key( &guard_on_duty )
                {
                    let mut v = guard_sleep_sched.get_mut( &guard_on_duty ).unwrap();
                    v.push( sleep_range );
                }
                else
                {
                    let mut v: Vec<Range<u16>> = Vec::new();
                    v.push( sleep_range );
                    guard_sleep_sched.insert( guard_on_duty, v );
                }
            }
        }
    }
    
    // Find the guard who sleeps the most
    let mut sleepiest_guard = 0;
    let mut highest_sleep_min = 0;
    let mut highest_sleep_count = 0;
    for ( guard, naps ) in &guard_sleep_sched
    {
        let mut min_counter: [u32; 60] = [0; 60];
        for nap in naps
        {
            for min in nap.start..nap.end
            {
                min_counter[min as usize] += 1;
            }
        }
        let min_mode = min_counter.iter().enumerate().max_by( |x, y| (x.1).cmp(y.1) ).unwrap();
        
        if highest_sleep_count < *min_mode.1
        {
            highest_sleep_min = min_mode.0;
            highest_sleep_count = *min_mode.1;
            sleepiest_guard = *guard;
        }
    }
    println!( "Guard {} was asleep at min {} a total of {} times", sleepiest_guard, highest_sleep_min, highest_sleep_count );

    return sleepiest_guard as u32 * highest_sleep_min as u32;
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum GuardEventType
{
    BEGIN( u16 ),
    SLEEP,
    WAKE
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct GuardEvent
{
    timestamp: NaiveDateTime,
    event: GuardEventType
}

impl GuardEvent
{
    fn new
        (
        timestamp: NaiveDateTime,
        event: GuardEventType
        ) -> Self
    {
        return Self { timestamp: timestamp, event: event };
    }
}

impl FromStr for GuardEvent
{
    type Err = chrono::format::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.trim_start_matches( '[' ).split(']');

        let timestamp_str = iter.next().unwrap().trim();
        let timestamp = NaiveDateTime::parse_from_str( &timestamp_str, "%F %R" ).expect( "Unable to parse timestamp" );
        
        let event_str = iter.next().unwrap().trim();
        let event: GuardEventType;
        if event_str.starts_with( "wakes up" )
        {
            event = GuardEventType::WAKE;
        }
        else if event_str.starts_with( "falls asleep" )
        {
            event = GuardEventType::SLEEP;
        }
        else
        {
            // Parse "Guard #3251 begins shift"
            let guard_id = event_str.split( ' ' ).nth( 1 ).unwrap().trim_start_matches( '#' ).parse::<u16>().unwrap();
            event = GuardEventType::BEGIN( guard_id );
        }

        return Ok( GuardEvent::new( timestamp, event ) );
    }
}
