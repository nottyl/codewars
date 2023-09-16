// Link: https://www.codewars.com/kata/513e08acc600c94f01000001/train/c
#include <stdio.h>
#include <criterion/criterion.h>
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

static void do_test (int r, int g, int b, const char expected[6 + 1]);

Test(RGBToHexConversion, BasicTests)
{
	do_test(  0,   0,   0, "000000");
	do_test(  1,   2,   3, "010203");
	do_test(255, 255, 255, "FFFFFF");
	do_test(254, 253, 252, "FEFDFC");
	do_test(-20, 275, 125, "00FF7D");
}

extern void rgb (int r, int g, int b, char hex[6 + 1]);

static void do_test (int r, int g, int b, const char expected[6 + 1])
{
	char actual[6 + 1] = "@@@@@@@";
	rgb(r, g, b, actual);

	cr_assert_str_eq(actual, expected,
		"r = %d, g = %d, b = %d\nexpected \"%s\", but got \"%s\"",
		r, g, b, expected, actual
	);
}
