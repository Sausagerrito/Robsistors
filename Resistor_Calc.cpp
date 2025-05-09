
#include <cmath>
#include <eng_format.hpp>
#include <iostream>
#include <string>





int three_band(std::string input) {

	bool check = true;

	double result = 0;
	int d1 = 0;
	int d2 = 0;
	int m = 0;
	switch (input[0]) {
	case 'K':case 'k':
		d1 = 0;
		break;
	case 'N':case 'n':
		d1 = 1;
		break;
	case 'R':case 'r':
		d1 = 2;
		break;
	case 'O':case 'o':
		d1 = 3;
		break;
	case 'Y':case 'y':
		d1 = 4;
		break;
	case 'G':case 'g':
		d1 = 5;
		break;
	case 'L':case 'l':
		d1 = 6;
		break;
	case 'V':case 'v':
		d1 = 7;
		break;
	case 'E':case 'e':
		d1 = 8;
		break;
	case 'W':case 'w':
		d1 = 9;
		break;
	default:
		std::cout << "ERROR: Band one input not supported" << std::endl;
		check = false;
	}

	switch (input[1]) {
	case 'K':case 'k':
		d2 = 0;
		break;
	case 'N':case 'n':
		d2 = 1;
		break;
	case 'R':case 'r':
		d2 = 2;
		break;
	case 'O':case 'o':
		d2 = 3;
		break;
	case 'Y':case 'y':
		d2 = 4;
		break;
	case 'G':case 'g':
		d2 = 5;
		break;
	case 'L':case 'l':
		d2 = 6;
		break;
	case 'V':case 'v':
		d2 = 7;
		break;
	case 'E':case 'e':
		d2 = 8;
		break;
	case 'W':case 'w':
		d2 = 9;
		break;
	default:
		std::cout << "ERROR: Band two input not supported" << std::endl;
		check = false;
	}

	switch (input[2]) {
	case 'K':case 'k':
		m = 0;
		break;
	case 'N':case 'n':
		m = 1;
		break;
	case 'R':case 'r':
		m = 2;
		break;
	case 'O':case 'o':
		m = 3;
		break;
	case 'Y':case 'y':
		m = 4;
		break;
	case 'G':case 'g':
		m = 5;
		break;
	case 'L':case 'l':
		m = 6;
		break;
	case 'V':case 'v':
		m = 7;
		break;
	case 'E':case 'e':
		m = 8;
		break;
	case 'W':case 'w':
		m = 9;
		break;
	case 'D':case 'd':
		m = -1;
		break;
	case 'S':case 's':
		m = -2;
		break;
	default:
		std::cout << "ERROR: Band three input not supported" << std::endl;
		check = false;
	}

	if (check == false)
		return 0;
	else;

	result = (d1 * 10 + d2) * pow(10, m);

	std::wcout.imbue(std::locale(""));
	std::cout << to_engineering_string(result, 4, false, "\xEA ") + "\n" << std::endl;

	return 0;
}

int four_band(std::string input) {
	
	bool check = true;

	double result = 0;
	int d1 = 0;
	int d2 = 0;
	int m = 0;
	double t = 0;
	switch (input[0]) {
		case 'K':case 'k':
			d1 = 0;
			break;
		case 'N':case 'n':
			d1 = 1;
			break;
		case 'R':case 'r':
			d1 = 2;
			break;
		case 'O':case 'o':
			d1 = 3;
			break;
		case 'Y':case 'y':
			d1 = 4;
			break;
		case 'G':case 'g':
			d1 = 5;
			break;
		case 'L':case 'l':
			d1 = 6;
			break;
		case 'V':case 'v':
			d1 = 7;
			break;
		case 'E':case 'e':
			d1 = 8;
			break;
		case 'W':case 'w':
			d1 = 9;
			break;
		default:
			std::cout << "ERROR: Band one input not supported" << std::endl;
			check = false;
		}

	switch (input[1]) {
		case 'K':case 'k':
			d2 = 0;
			break;
		case 'N':case 'n':
			d2 = 1;
			break;
		case 'R':case 'r':
			d2 = 2;
			break;
		case 'O':case 'o':
			d2 = 3;
			break;
		case 'Y':case 'y':
			d2 = 4;
			break;
		case 'G':case 'g':
			d2 = 5;
			break;
		case 'L':case 'l':
			d2 = 6;
			break;
		case 'V':case 'v':
			d2 = 7;
			break;
		case 'E':case 'e':
			d2 = 8;
			break;
		case 'W':case 'w':
			d2 = 9;
			break;
		default:
			std::cout << "ERROR: Band two input not supported" << std::endl;
			check = false;
		}

	switch (input[2]) {
		case 'K':case 'k':
			m = 0;
			break;
		case 'N':case 'n':
			m = 1;
			break;
		case 'R':case 'r':
			m = 2;
			break;
		case 'O':case 'o':
			m = 3;
			break;
		case 'Y':case 'y':
			m = 4;
			break;
		case 'G':case 'g':
			m = 5;
			break;
		case 'L':case 'l':
			m = 6;
			break;
		case 'V':case 'v':
			m = 7;
			break;
		case 'E':case 'e':
			m = 8;
			break;
		case 'W':case 'w':
			m = 9;
			break;
		case 'D':case 'd':
			m = -1;
			break;
		case 'S':case 's':
			m = -2;
			break;
		default:
			std::cout << "ERROR: Band three input not supported" << std::endl;
			check = false;
		}

	switch (input[3]) {
		case 'N':case 'n':
			t = 0.01;
			break;
		case 'R':case 'r':
			t = 0.02;
			break;
		case 'O':case 'o':
			t = 0.03;
			break;
		case 'Y':case 'y':
			t = 0.04;
			break;
		case 'G':case 'g':
			t = 0.005;
			break;
		case 'L':case 'l':
			t = 0.0025;
			break;
		case 'V':case 'v':
			t = 0.001;
			break;
		case 'E':case 'e':
			t = 0.0005;
			break;
		case 'D':case 'd':
			t = 0.05;
			break;
		case 'S':case 's':
			t = 0.10;
			break;
		default:
			std::cout << "ERROR: Band four input not supported" << std::endl;
			check = false;
		}

	if (check == false)
		return 0;
	else;

	result = (d1 * 10 + d2) * pow(10, m);

	std::wcout.imbue(std::locale(""));
	std::cout << to_engineering_string(result, 4, false, "\xEA \xF1 ");
	std::cout << to_engineering_string(t * result, 4, false, "\xEA") + "\n" << std::endl;
		
	

	return 0;
}

int five_band(std::string input) {

	bool check = true;

	double result = 0;
	int d1 = 0;
	int d2 = 0;
	int d3 = 0;
	int m = 0;
	double t = 0;
	switch (input[0]) {
	case 'K':case 'k':
		d1 = 0;
		break;
	case 'N':case 'n':
		d1 = 1;
		break;
	case 'R':case 'r':
		d1 = 2;
		break;
	case 'O':case 'o':
		d1 = 3;
		break;
	case 'Y':case 'y':
		d1 = 4;
		break;
	case 'G':case 'g':
		d1 = 5;
		break;
	case 'L':case 'l':
		d1 = 6;
		break;
	case 'V':case 'v':
		d1 = 7;
		break;
	case 'E':case 'e':
		d1 = 8;
		break;
	case 'W':case 'w':
		d1 = 9;
		break;
	default:
		std::cout << "ERROR: Band one input not supported" << std::endl;
		check = false;
	}

	switch (input[1]) {
	case 'K':case 'k':
		d2 = 0;
		break;
	case 'N':case 'n':
		d2 = 1;
		break;
	case 'R':case 'r':
		d2 = 2;
		break;
	case 'O':case 'o':
		d2 = 3;
		break;
	case 'Y':case 'y':
		d2 = 4;
		break;
	case 'G':case 'g':
		d2 = 5;
		break;
	case 'L':case 'l':
		d2 = 6;
		break;
	case 'V':case 'v':
		d2 = 7;
		break;
	case 'E':case 'e':
		d2 = 8;
		break;
	case 'W':case 'w':
		d2 = 9;
		break;
	default:
		std::cout << "ERROR: Band two input not supported" << std::endl;
		check = false;
	}

	switch (input[2]) {
	case 'K':case 'k':
		d3 = 0;
		break;
	case 'N':case 'n':
		d3 = 1;
		break;
	case 'R':case 'r':
		d3 = 2;
		break;
	case 'O':case 'o':
		d3 = 3;
		break;
	case 'Y':case 'y':
		d3 = 4;
		break;
	case 'G':case 'g':
		d3 = 5;
		break;
	case 'L':case 'l':
		d3 = 6;
		break;
	case 'V':case 'v':
		d3 = 7;
		break;
	case 'E':case 'e':
		d3 = 8;
		break;
	case 'W':case 'w':
		d3 = 9;
		break;
	default:
		std::cout << "ERROR: Band three input not supported" << std::endl;
		check = false;
	}

	switch (input[3]) {
	case 'K':case 'k':
		m = 0;
		break;
	case 'N':case 'n':
		m = 1;
		break;
	case 'R':case 'r':
		m = 2;
		break;
	case 'O':case 'o':
		m = 3;
		break;
	case 'Y':case 'y':
		m = 4;
		break;
	case 'G':case 'g':
		m = 5;
		break;
	case 'L':case 'l':
		m = 6;
		break;
	case 'V':case 'v':
		m = 7;
		break;
	case 'E':case 'e':
		m = 8;
		break;
	case 'W':case 'w':
		m = 9;
		break;
	case 'D':case 'd':
		m = -1;
		break;
	case 'S':case 's':
		m = -2;
		break;
	default:
		std::cout << "ERROR: Band four input not supported" << std::endl;
		check = false;
	}

	switch (input[4]) {
	case 'N':case 'n':
		t = 0.01;
		break;
	case 'R':case 'r':
		t = 0.02;
		break;
	case 'O':case 'o':
		t = 0.03;
		break;
	case 'Y':case 'y':
		t = 0.04;
		break;
	case 'G':case 'g':
		t = 0.005;
		break;
	case 'L':case 'l':
		t = 0.0025;
		break;
	case 'V':case 'v':
		t = 0.001;
		break;
	case 'E':case 'e':
		t = 0.0005;
		break;
	case 'D':case 'd':
		t = 0.05;
		break;
	case 'S':case 's':
		t = 0.10;
		break;
	default:
		std::cout << "ERROR: Band five input not supported" << std::endl;
		check = false;
	}

	if (check == false)
		return 0;
	else;

	result = (d1 * 100 + d2 * 10 + d3) * pow(10, m);

	std::wcout.imbue(std::locale(""));
	std::cout << to_engineering_string(result, 4, false, "\xEA \xF1 ");
	std::cout << to_engineering_string(t * result, 4, false, "\xEA") + "\n" << std::endl;



	return 0;
}

int six_band(std::string input) {

	bool check = true;

	double result = 0;
	int d1 = 0;
	int d2 = 0;
	int d3 = 0;
	int m = 0;
	double t = 0;
	int c = 0;
	switch (input[0]) {
	case 'K':case 'k':
		d1 = 0;
		break;
	case 'N':case 'n':
		d1 = 1;
		break;
	case 'R':case 'r':
		d1 = 2;
		break;
	case 'O':case 'o':
		d1 = 3;
		break;
	case 'Y':case 'y':
		d1 = 4;
		break;
	case 'G':case 'g':
		d1 = 5;
		break;
	case 'L':case 'l':
		d1 = 6;
		break;
	case 'V':case 'v':
		d1 = 7;
		break;
	case 'E':case 'e':
		d1 = 8;
		break;
	case 'W':case 'w':
		d1 = 9;
		break;
	default:
		std::cout << "ERROR: Band one input not supported" << std::endl;
		check = false;
	}

	switch (input[1]) {
	case 'K':case 'k':
		d2 = 0;
		break;
	case 'N':case 'n':
		d2 = 1;
		break;
	case 'R':case 'r':
		d2 = 2;
		break;
	case 'O':case 'o':
		d2 = 3;
		break;
	case 'Y':case 'y':
		d2 = 4;
		break;
	case 'G':case 'g':
		d2 = 5;
		break;
	case 'L':case 'l':
		d2 = 6;
		break;
	case 'V':case 'v':
		d2 = 7;
		break;
	case 'E':case 'e':
		d2 = 8;
		break;
	case 'W':case 'w':
		d2 = 9;
		break;
	default:
		std::cout << "ERROR: Band two input not supported" << std::endl;
		check = false;
	}

	switch (input[2]) {
	case 'K':case 'k':
		d3 = 0;
		break;
	case 'N':case 'n':
		d3 = 1;
		break;
	case 'R':case 'r':
		d3 = 2;
		break;
	case 'O':case 'o':
		d3 = 3;
		break;
	case 'Y':case 'y':
		d3 = 4;
		break;
	case 'G':case 'g':
		d3 = 5;
		break;
	case 'L':case 'l':
		d3 = 6;
		break;
	case 'V':case 'v':
		d3 = 7;
		break;
	case 'E':case 'e':
		d3 = 8;
		break;
	case 'W':case 'w':
		d3 = 9;
		break;
	default:
		std::cout << "ERROR: Band three input not supported" << std::endl;
		check = false;
	}

	switch (input[3]) {
	case 'K':case 'k':
		m = 0;
		break;
	case 'N':case 'n':
		m = 1;
		break;
	case 'R':case 'r':
		m = 2;
		break;
	case 'O':case 'o':
		m = 3;
		break;
	case 'Y':case 'y':
		m = 4;
		break;
	case 'G':case 'g':
		m = 5;
		break;
	case 'L':case 'l':
		m = 6;
		break;
	case 'V':case 'v':
		m = 7;
		break;
	case 'E':case 'e':
		m = 8;
		break;
	case 'W':case 'w':
		m = 9;
		break;
	case 'D':case 'd':
		m = -1;
		break;
	case 'S':case 's':
		m = -2;
		break;
	default:
		std::cout << "ERROR: Band four input not supported" << std::endl;
		check = false;
	}

	switch (input[4]) {
	case 'N':case 'n':
		t = 0.01;
		break;
	case 'R':case 'r':
		t = 0.02;
		break;
	case 'O':case 'o':
		t = 0.03;
		break;
	case 'Y':case 'y':
		t = 0.04;
		break;
	case 'G':case 'g':
		t = 0.005;
		break;
	case 'L':case 'l':
		t = 0.0025;
		break;
	case 'V':case 'v':
		t = 0.001;
		break;
	case 'E':case 'e':
		t = 0.0005;
		break;
	case 'D':case 'd':
		t = 0.05;
		break;
	case 'S':case 's':
		t = 0.10;
		break;
	default:
		std::cout << "ERROR: Band five input not supported" << std::endl;
		check = false;
	}

	switch (input[5]) {
	case 'K':case 'k':
		c = 250;
		break;
	case 'N':case 'n':
		c = 100;
		break;
	case 'R':case 'r':
		c = 50;
		break;
	case 'O':case 'o':
		c = 15;
		break;
	case 'Y':case 'y':
		c = 25;
		break;
	case 'G':case 'g':
		c = 20;
		break;
	case 'L':case 'l':
		c = 10;
		break;
	case 'V':case 'v':
		c = 5;
		break;
	case 'E':case 'e':
		c = 1;
		break;
	default:
		std::cout << "ERROR: Band six input not supported" << std::endl;
		check = false;
	}

	if (check == false)
		return 0;
	else;

	result = (d1 * 100 + d2 * 10 + d3) * pow(10, m);

	std::wcout.imbue(std::locale(""));
	std::cout << to_engineering_string(result, 4, false, "\xEA \xF1 ");
	std::cout << to_engineering_string(t * result, 4, false, "\xEA   ");
	printf("%d ppm/\xF8\C\n",c);



	return 0;
}

int main()
{
	std::cout << "blac[K] brow[N] [R]ed [O]range [Y]ellow [G]reen b[L]ue [V]iolet gr[E]y [W]hite gol[D] [S]ilver\n" << std::endl;

	std::cout << "Color Code:" << std::endl;
	while (true) {
		std::string input;

		std::cin >> input;
		int size = input.size();

		switch (size) {
		case 3:
			three_band(input);
			break;
		case 4:
			four_band(input);
			break;
		case 5:
			five_band(input);
			break;
		case 6:
			six_band(input);
			break;
		default:
			std::cout << "ERROR: Input length not supported" << std::endl;
			break;
			}

		}
	return 0;
}
