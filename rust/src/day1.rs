use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashSet;

static INPUT_FILE: &str = "../input/day1.txt";

pub fn part1() -> i32
{
    let input_file = File::open( INPUT_FILE ).expect( "Unable to open day1.txt" );
    let reader = BufReader::new( input_file );
    
    let mut freq: i32 = 0;
    for line in reader.lines()
    {
        let delta = line.unwrap().parse::<i32>().unwrap();
        freq = freq + delta;
    }
    
    return freq;
}

pub fn part2() -> i32
{
    let mut all_freqs = HashSet::new();
    let mut freq: i32 = 0;
    all_freqs.insert( freq ); // Initial frequency
    let mut dup_found = false;

    while !dup_found
    {
        let input_file = File::open( INPUT_FILE ).expect( "Unable to open day1.txt" );
        let reader = BufReader::new( input_file );

        for line in reader.lines()
        {
            let delta = line.unwrap().parse::<i32>().unwrap();
            freq = freq + delta;
            if all_freqs.contains( &freq )
            {
                dup_found = true;
                break;
            }
            else
            {
                all_freqs.insert( freq );
            }
        }
    }
    
    return freq;
}
