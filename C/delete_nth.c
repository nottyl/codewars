#include <stddef.h>
#include <stdlib.h>

//  return a dynamically allocated int array
//  the return array will be freed by tester
//  set *szout to the length of output array

int *delete_nth(size_t szin, const int order[szin], int max_e, size_t *szout) {
	int *output = (int *) malloc(szin * sizeof(int));
    int *temp = (int *) malloc(szin * sizeof(int));
	int count = 0;

	int counters[szin];
	for(int i = 0; i < szin; i++) {
		counters[i] = 0;
	}

    for (int i = 0; i < szin; i++) {
        int current = order[i];
        int current_count = counters[current];
        if (current_count < max_e) {
            temp[count] = current;
            counters[current]++;
            count++;
        }
    }

    *szout = count;
    for (int i = 0; i < count; i++) {
        output[i] = temp[i];
    }

    free(temp);

    return output;	return output;
}
