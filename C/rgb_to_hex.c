// Link: https://www.codewars.com/kata/513e08acc600c94f01000001/train/c
#include <stdio.h>
#include <string.h>

#define CHECK_BOUNDARY(color) \
	if(color < 0) {           \
		color = 0;            \
	}                         \
	else if(color > 255) {    \
		color = 255;          \
	}


void rgb(int r, int g, int b, char hex[6 + 1]) {
	CHECK_BOUNDARY(r);
	CHECK_BOUNDARY(g);
	CHECK_BOUNDARY(b);
	sprintf(hex, "%02X%02X%02X", r, g, b);
}

// // Best Practice:
// unsigned char wrap(int a) {
// 	if(a > 255) return 255;
// 	else if(a < 0)
// 		return 0;
// 	else
// 		return a;
// }
// int rgb(int r, int g, int b, char *output) {
// 	sprintf(output, "%02X%02X%02X\0", wrap(r), wrap(g), wrap(b));
// 	return 0;
// }
