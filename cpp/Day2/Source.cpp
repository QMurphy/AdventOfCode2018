#include <iostream>
#include <fstream>
#include <cstdint>
#include <vector>

#define INPUT_FILE "../../input/day2.txt"

int part1()
{
	std::ifstream input;
	input.open( INPUT_FILE );
	if( !input.is_open() )
	{
		std::cerr << "Unable to find input.txt" << std::endl;
		return -1;
	}

	char id[32];
	uint32_t num_doubles = 0;
	uint32_t num_triples = 0;
	while( !input.eof() )
	{
		input.getline( id, 32 );
		if( !input.good() )
		{
			break;
		}

		uint8_t letter_cnts[26] = { 0 };
		uint8_t id_len = std::strlen( id );
		for( uint8_t i = 0; i < id_len; i++ )
		{
			letter_cnts[id[i] - 'a']++;
		}

		bool has_double = false;
		bool has_triple = false;
		for( uint8_t i = 0; i < 26 && ( !has_double || !has_triple ); i++ )
		{
			if( letter_cnts[i] == 2 )
			{
				has_double = true;
			}
			else if( letter_cnts[i] == 3 )
			{
				has_triple = true;
			}
		}

		if( has_double )
		{
			num_doubles++;
		}
		if( has_triple )
		{
			num_triples++;
		}
	}

	std::cout << num_doubles * num_triples << std::endl;
	
	return 0;
}

uint32_t hamming_distance
	(
	std::string str1,
	std::string str2
	);

int part2()
{
	std::ifstream input;
	input.open( INPUT_FILE );
	if( !input.is_open() )
	{
		std::cerr << "Unable to find input.txt" << std::endl;
		return -1;
	}

	char id[32];
	std::vector<std::string> ids;
	while( !input.eof() )
	{
		input.getline( id, 32 );
		if( !input.good() )
		{
			break;
		}

		ids.push_back( std::string( id ) );
	}

	for( size_t i = 0; i < ids.size() - 1; i++ )
	{
		for( size_t j = i + 1; j < ids.size(); j++ )
		{
			if( hamming_distance( ids[i], ids[j] ) == 1 )
			{
				for( size_t k = 0; k < ids[i].length(); k++ )
				{
					if( ids[i][k] == ids[j][k] )
					{
						std::cout << ids[i][k];
					}
				}
				std::cout << std::endl;
				return 0;
			}
		}
	}

	return -1;
}

int main()
{
	//part1();
	part2();
}

uint32_t hamming_distance
	(
	std::string str1,
	std::string str2
	)
{
	if( str1.length() != str2.length() )
	{
		return std::numeric_limits<uint32_t>::max();
	}

	uint32_t count = 0;
	for( size_t i = 0; i < str1.length(); i++ )
	{
		if( str1[i] != str2[i] )
		{
			count++;
		}
	}
	return count;
}
