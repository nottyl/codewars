#include <stdint.h>

char *create_phone_number(char phnum[15], const uint8_t digits[10]) {
	// write to phnum and return it
	// it must be nul-terminated
    for(int i=0; i<10; i++){
        *phnum = digits[i];
        phnum++;
    }
	*phnum = '\0';
	return phnum;
}

int main(){
    char phone_number[15];
    const uint8_t digits[10] = {1,2,3,4,5,6,7,8,9};
    create_phone_number(phone_number, digits);
}
