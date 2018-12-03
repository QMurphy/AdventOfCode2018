use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

static INPUT_FILE: &str = "../input/day2.txt";

pub fn part1() -> i32
{
    let input_file = File::open( INPUT_FILE ).expect( "Unable to open input file" );
    let reader = BufReader::new( input_file );

    let mut num_doubles = 0;
    let mut num_triples = 0;

    for line in reader.lines()
    {
        let id = line.unwrap();
        let mut counts = [ 0; 26 ];
        for letter in id.into_bytes()
        {
            counts[( letter - ( 'a' as u8 ) ) as usize] += 1;
        }

        let mut has_double = false;
        let mut has_triple = false;
        for count in counts.iter()
        {
            if *count == 2
            {
                has_double = true;
            }
            else if *count == 3
            {
                has_triple = true;
            }

            if has_double && has_triple
            {
                break;
            }
        }

        if has_double
        {
            num_doubles += 1;
        }
        if has_triple
        {
            num_triples += 1;
        }
    }
    
    return num_doubles * num_triples;
}

pub fn part2() -> Result< String, () >
{
    let input_file = File::open( INPUT_FILE ).expect( "Unable to open input file" );
    let reader = BufReader::new( input_file );

    let mut ids : Vec<String> = Vec::new();
    for line in reader.lines()
    {
        ids.push( line.unwrap() );
    }

    for i in 0..( ids.len() - 1 )
    {
        for j in ( i + 1 )..ids.len()
        {
            match hamming_distance( &ids[i], &ids[j] )
            {
                Ok( dist ) =>
                {
                    if dist == 1
                    {
                        return Ok( get_common_chars( &ids[i], &ids[j] ) );
                    }
                },
                Err( () ) => panic!( "IDs don't have matching length!" )
            }
        }
    }

    return Err( () );
}

fn hamming_distance
    (
    str1 : &str,
    str2 : &str
    ) -> Result<u32, ()>
{
    let mut dist = 0;
    if str1.len() != str2.len()
    {
        return Err( () );
    }

    for letters in str1.chars().zip( str2.chars() )
    {
        if letters.0 != letters.1
        {
            dist += 1;
        }
    }
    return Ok( dist );
}

fn get_common_chars
    (
    str1 : &str,
    str2 : &str
    ) -> String
{
    let mut common : String = String::new();
    if str1.len() != str2.len()
    {
        return common;
    }

    for letters in str1.chars().zip( str2.chars() )
    {
        if letters.0 == letters.1
        {
            common.push( letters.0 );
        }
    }
    return common;
}
