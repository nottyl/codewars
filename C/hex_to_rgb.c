#include <ctype.h>
#include <math.h>
#include <stdio.h>
#include <stdlib.h>

typedef struct {
	int r, g, b;
} rgb;

int to_decimal(char _hex, int digit) {
	int decimal;
    char hex;
    if (isalpha(_hex)){
        hex = toupper(_hex);
    }
    else{
        hex = atoi(&_hex);
    }

	switch(hex) {
		case 'A':
			decimal = 10 * (int)pow(16, digit);
			break;
		case 'B':
			decimal = 11 * (int)pow(16, digit);
			break;
		case 'C':
			decimal = 12 * (int)pow(16, digit);
			break;
		case 'D':
			decimal = 13 * (int)pow(16, digit);
			break;
		case 'E':
			decimal = 14 * (int)pow(16, digit);
			break;
		case 'F':
			decimal = 15 * (int)pow(16, digit);
			break;
        case '#':
            break;
		default:
			decimal = hex * (int)pow(16, digit);
			break;
	}
	return decimal;
}

rgb hex_str_to_rgb(const char *hex_str) {
	rgb result;
	result.r = to_decimal(hex_str[1], 1) + to_decimal(hex_str[2], 0);
	result.g = to_decimal(hex_str[3], 1) + to_decimal(hex_str[4], 0);
	result.b = to_decimal(hex_str[5], 1) + to_decimal(hex_str[6], 0);
	return result;
}

int main(){
    rgb result = hex_str_to_rgb("#Fa3456");
    printf("%d, %d, %d", result.r, result.g, result.b);
    return 0;
}

// Best Practice. I am an insane person
// typedef struct { int r, g, b; } rgb;
//
// rgb hex_str_to_rgb(const char *hex_str) {
//   int r, g, b;
//   sscanf(hex_str, "#%2x%2x%2x", &r, &g, &b);
//   return (rgb){r, g, b};
// }
