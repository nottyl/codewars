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
