#include <iostream>
#include <fstream>
#include <cstdint>
#include <set>

int part1()
{
	std::ifstream input;
	input.open( "input.txt" );
	if( !input.is_open() )
	{
		std::cerr << "Unable to find input.txt" << std::endl;
		return -1;
	}

	int64_t freq = 0;
	while( !input.eof() )
	{
		int32_t delta = 0;
		input >> delta;
		if( !input.good() )
		{
			break;
		}
		freq += delta;
	}
	std::cout << freq << std::endl;

	return 0;
}

int part2()
{
	std::ifstream input;
	input.open( "input.txt" );
	if( !input.is_open() )
	{
		std::cerr << "Unable to find input.txt" << std::endl;
		return -1;
	}

	std::set<int64_t> all_freqs;
	int64_t freq = 0;
	bool dup_found = false;
	while( !dup_found )
	{
		input.clear();
		input.seekg( input.beg, std::ios::beg );
		while( !input.eof() )
		{
			int32_t delta = 0;
			input >> delta;
			if( !input.good() )
			{
				break;
			}
			freq += delta;

			if( all_freqs.count( freq ) == 0 )
			{
				all_freqs.insert( freq );
			}
			else
			{
				dup_found = true;
				break;
			}
		}
	}
	
	std::cout << freq << std::endl;

	return 0;
}

int main()
{
	//part1();
	part2();
}
