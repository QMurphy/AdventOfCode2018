use std::fs::File;
use std::io::Read;

static INPUT_FILE: &str = "../input/day5.txt";

pub fn part1() -> Result<u32, ()>
{
    let mut input_file = File::open( INPUT_FILE ).expect( "Unable to open input file" );
    let mut queue: Vec<u8> = Vec::new();
    let mut stack: Vec<u8> = Vec::new();
    input_file.read_to_end( &mut queue ).expect( "Unable to read input file" );
    
    if !queue.is_ascii()
    {
        return Err( () );
    }
    
    if *queue.last().unwrap() == ( '\n' as u8 )
    {
        let final_element = queue.len() - 1;
        queue.remove( final_element );
    }

    // Treat polymer as a queue, push onto a stack
    // If the top two values in the stack are same type but opposite polarity, pop twice
    
    for c in queue.iter()
    {
        stack.push( *c );
        if stack.len() >= 2
        {
            let c1 = stack[ stack.len() - 2 ];
            let c2 = stack[ stack.len() - 1 ];
            if equal_type_opposite_polarity( c1, c2 )
            {
                stack.pop();
                stack.pop();
            }
        }
    }

    return Ok( stack.len() as u32 );
}

pub fn part2() -> Result<u32, ()>
{
    let mut input_file = File::open( INPUT_FILE ).expect( "Unable to open input file" );
    let mut queue: Vec<u8> = Vec::new();
    input_file.read_to_end( &mut queue ).expect( "Unable to read input file" );
    
    if !queue.is_ascii()
    {
        return Err( () );
    }
    
    if *queue.last().unwrap() == ( '\n' as u8 )
    {
        let final_element = queue.len() - 1;
        queue.remove( final_element );
    }

    let mut smallest_chain: u32 = u32::max_value();
    let mut smallest_letter: char = 'a';
    for l in 0..26
    {
        let banned_letter = l + 'a' as u8;
        let mut stack: Vec<u8> = Vec::new();
        for c in queue.iter()
        {
            if ( *c ).to_ascii_lowercase() != banned_letter
            {
                stack.push( *c );
                if stack.len() >= 2
                {
                    let c1 = stack[ stack.len() - 2 ];
                    let c2 = stack[ stack.len() - 1 ];
                    if equal_type_opposite_polarity( c1, c2 )
                    {
                        stack.pop();
                        stack.pop();
                    }
                }
            }
        }
        
        if smallest_chain > stack.len() as u32
        {
            smallest_chain = stack.len() as u32;
            smallest_letter = banned_letter as char;
        }
    }
    println!( "Removing {}", smallest_letter );

    return Ok( smallest_chain as u32 );
}

fn equal_type_opposite_polarity
    (
    c1: u8,
    c2: u8
    ) -> bool
{
    let equal_type = c1.to_ascii_lowercase() == c2.to_ascii_lowercase();
    let opposite_polarity = ( c1.is_ascii_lowercase() && c2.is_ascii_uppercase() ) || ( c1.is_ascii_uppercase() && c2.is_ascii_lowercase() );
    return equal_type && opposite_polarity;
}
