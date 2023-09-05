// Link: https://www.codewars.com/kata/526dbd6c8c0eb53254000110

#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

bool alphanumeric(const char *string) {
    if (string == NULL){
        return false;
    }
	int len = strlen(string);
	for(int i = 0; i < len; i++) {
		if(!isalnum(string[i])){
            return false;
        }
	}
    return true;
}

// // Best Practice
//
// bool alphanumeric(const char* string) {
//   if (*string == '\0') return false;
//   while (*string) if (!isalnum(*string++)) return false;
//   return true;
// }
