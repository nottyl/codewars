#include <stddef.h>
#include <stdio.h>

// The brainless approach
void josephus_permutation(size_t n, int permuted[n], const int array[n], size_t k) {
	// For keeping track of which element to take out
	int index = 0;
	// Take out the element and store it in the permuted array
	for(int i = 1; i <= n; i++) {
		if(index <= n) {
			index += k;
		}
		else {
			index -= n;
			index += k;
		}

    }
}
