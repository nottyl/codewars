// Link: https://www.codewars.com/kata/54c27a33fb7da0db0100040e

#include <math.h>
#include <stdbool.h>
#include <stdio.h>

bool is_square(int n) {
	double square = sqrt((double) n);
	if(square >= 0 && square - (int) square == 0) {
		return true;
	}
	return false;
}

// // Best Practice
// bool is_square(int n) {
// 	int s = sqrt(n);
// 	return n == s * s;
// }
