extern crate ndarray;

use std::str::FromStr;
use std::cmp::{min, max};
use std::num::ParseIntError;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use self::ndarray::Array2;

static INPUT_FILE: &str = "../input/day3.txt";

pub fn part1() -> u32
{
    let input_file = File::open( INPUT_FILE ).expect( "Unable to open input file" );
    let reader = BufReader::new( input_file );

    let mut claims: Vec<Rect> = Vec::new();
    let mut width = 0;
    let mut height = 0;
    for line in reader.lines()
    {
        let claim = line.unwrap().parse::<Rect>().unwrap();
        width = max( width, claim.x + claim.width );
        height = max( height, claim.y + claim.height );
        claims.push( claim );
    }

    let mut fabric = Array2::<u8>::zeros( (height as usize, width as usize) );
    for claim in claims
    {
        for y in claim.y..claim.y+claim.height
        {
            for x in claim.x..claim.x+claim.width
            {
                if fabric[[y as usize, x as usize]] <= 1
                {
                    fabric[[y as usize, x as usize]] += 1;
                }
            }
        }
    }

    let mut total = 0;
    for y in 0..height
    {
        for x in 0..width
        {
            if fabric[[y as usize, x as usize]] == 2
            {
                total += 1;
            }
        }
    }
    
    return total;
}

pub fn part2() -> Result< u32, () >
{
    let input_file = File::open( INPUT_FILE ).expect( "Unable to open input file" );
    let reader = BufReader::new( input_file );

    let mut claims: Vec<Rect> = Vec::new();
    for line in reader.lines()
    {
        let claim = line.unwrap().parse::<Rect>().unwrap();
        claims.push( claim );
    }

    // Naive brute force, could also build an array so we don't recheck failed IDs later on
    for i in 0..claims.len()
    {
        let mut intersection = false;
        for j in 0..claims.len()
        {
            if i == j
            {
                continue;
            }

            if claims[i].intersects( &claims[j] )
            {
                intersection = true;
                break;
            }
        }

        if !intersection
        {
            return Ok( ( i + 1 ) as u32 );
        }
    }

    return Err( () );
}

#[derive(Debug, PartialEq)]
struct Rect {
    x: i32,
    y: i32,
    width: i32,
    height: i32
}

impl Rect
{
    fn new
        (
        x: i32,
        y: i32,
        width: i32,
        height: i32
        ) -> Rect
    {
        return Rect { x: x, y: y, width: width, height: height };
    }

    fn area( &self ) -> i32
    {
        return self.width * self.height;
    }

    fn intersection( &self, other: &Rect ) -> Rect
    {
        let mut ret = Rect::new( 0, 0, 0, 0 );

        ret.x = max( self.x, other.x );
        ret.y = max( self.y, other.y );
        ret.width = min( self.x + self.width, other.x + other.width) - ret.x;
        ret.height = min( self.y + self.height, other.y + other.height) - ret.y;

        if ret.width <= 0 || ret.height <= 0
        {
            ret.x = 0;
            ret.y = 0;
            ret.width = 0;
            ret.height = 0;
        }

        return ret;
    }

    fn intersects( &self, other: &Rect ) -> bool
    {
        return self.intersection( other ).area() > 0;
    }
}

impl FromStr for Rect
{
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tmp: Vec<&str> = s.split( '@' )
                                 .last()
                                 .unwrap()
                                 .split( ':' )
                                 .collect();

        let pts: Vec<&str> = tmp[0].split( ',' ).collect();
        let size: Vec<&str> = tmp[1].split( 'x' ).collect();

        let x = pts[0].trim().parse::<i32>()?;
        let y = pts[1].trim().parse::<i32>()?;
        let w = size[0].trim().parse::<i32>()?;
        let h = size[1].trim().parse::<i32>()?;

        return Ok( Rect::new( x, y, w, h ) );
    }
}
