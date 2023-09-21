#include <stdlib.h>
#include <string.h>

char *dna_to_rna(const char *dna) {
    int length = strlen(dna);
    char *rna = (char *)malloc((length + 1) * sizeof(char));

    for(int i=0; i<length; i++){
        if(dna[i] == 'T'){
            rna[i] = 'U';
        }
        else{
            rna[i] = dna[i];
        }
    }

    rna[length] = '\0';
	return rna;
}
