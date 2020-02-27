#pragma once

__constant ushort DIGITS[10] = {
    0b111101101101111, // 0
    0b001001001001001, // 1
    0b111001111100111, // 2
    0b111001111001111, // 3
    0b101101111001001, // 4
    0b111100111001111, // 5
    0b111100111101111, // 6
    0b111001001001001, // 7
    0b111101111101111, // 8
    0b111101111001111  // 9
};

bool print_number(uint number, int2 shift, int scale) {
    //bool minus = number < 0;
    //number = abs(number);
    shift.x = -shift.x;
    shift.y = -shift.y;
    if (shift.x < 0 || shift.y < 0) {
        return false;
    }
    shift /= scale;
    if (shift.y >= 5) {
        return false;
    }
    int pos = 0;
    while (shift.x > 3 && number > 0) {
        shift.x -= 4;
        number /= 10;
        pos += 1;
    }
    if (shift.x >= 3) {
        return false;
    }
    if (pos != 0 && number == 0) {
        return false;
    }
    return (DIGITS[number % 10] >> (shift.x + shift.y*3)) & 1;
}


// Sample usage
/*
	uint num = ...; // number to print
	int2 pp = (int2)(idx % width, idx / width); // pixel position
	int ts = 4; // text size
	if (print_number(num, pp - (int2)(width - 1 - ts, 6*ts), ts)) {
		vstore4((uchar4)((uchar3)(0), 0xff), idx, image);
		return;
	}
*/
