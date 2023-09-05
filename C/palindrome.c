// Link: https://www.codewars.com/kata/57a1fd2ce298a731b20006a4/c

#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <string.h>

bool is_palindrome(const char *str_in) {
	int length = strlen(str_in);
	for(int i = 0; i < length; i++) {
		if(tolower(str_in[i]) != tolower(str_in[length - 1 - i])) {
			return false;
		}
	}
	return true;
}

// // Best Practice
// bool is_palindrome(const char *s) {
// 	int l = strlen(s);
// 	for(int i = 0; i < l; i++)
// 		if(tolower(s[i]) != tolower(s[--l]))
// 			return false;
// 	return true;
// }
