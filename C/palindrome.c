#include <stdbool.h>
#include <string.h>
#include <stdio.h>
#include <ctype.h>

bool is_palindrome(const char *str_in) {
    int length = strlen(str_in);
    for(int i=0; i<length; i++){
        if(tolower(str_in[i]) != tolower(str_in[length - 1 - i])){
            return false;
        }
    }
    return true;
}
