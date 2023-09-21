#include <stdbool.h>
#include <string.h>

// input is a null-terminated string

bool isValidWalk(const char *walk) {
	int len = strlen(walk);
	if(len != 10) {
		return false;
	}
	int endpoint[2] = {0, 0};
	for(int i = 0; i < len; i++) {
		if(walk[i] == 'n') {
			endpoint[0]++;
		}
		else if(walk[i] == 's') {
			endpoint[0]--;
		}
		else if(walk[i] == 'w') {
			endpoint[1]++;
		}
		else if(walk[i] == 'e') {
			endpoint[1]--;
		}
	}
	if(endpoint[0] == 0 && endpoint[1] == 0) {
		return true;
	}
	return false;
}

// Best Practice
// bool isValidWalk(const char *walk) {
// 	if(strlen(walk) != 10) return 0;
//
// 	int h = 0, v = 0;
// 	while(*walk) {
// 		switch(*walk) {
// 			case 'n':
// 				++v;
// 				break;
// 			case 's':
// 				--v;
// 				break;
// 			case 'e':
// 				++h;
// 				break;
// 			case 'w':
// 				--h;
// 				break;
// 		}
// 		++walk;
// 	}
// 	return h == 0 && v == 0;
// }
