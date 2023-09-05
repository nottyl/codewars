// Link: https://www.codewars.com/kata/5270d0d18625160ada0000e4/train/c

#include <stdio.h>

int score(const int dice[5]) {
	int total_score = 0;
	int count[6] = {0};
	for(int i = 0; i < 5; i++) {
		count[dice[i] - 1]++;
	}
	if(count[0] >= 3) {
		total_score += 1000;
		count[0] -= 3;
	}
	if(count[5] >= 3) {
		total_score += 600;
	}
	if(count[4] >= 3) {
		total_score += 500;
		count[4] -= 3;
	}
	if(count[3] >= 3) {
		total_score += 400;
	}
	if(count[2] >= 3) {
		total_score += 300;
	}
	if(count[1] >= 3) {
		total_score += 200;
	}
	if(count[0] > 0) {
		total_score += 100 * count[0];
	}
	if(count[4] > 0) {
		total_score += 50 * count[4];
	}
    return total_score;
}

// // Best Practice (Honestly kind of the same? Very imperative)
// int score(const int dice[5]) {
//     int count[7] = {0, 0, 0, 0, 0, 0, 0};
//     for( int i = 0; i < 5; i++ ) {
//         count[dice[i]]++;
//     }
//     
//     int score = 0;
//     // Three 1's => 1000 points
//     score += 1000 * (count[1] / 3);
//     // Three 6's =>  600 points
//     score += 600 * (count[6] / 3);
//     // Three 5's =>  500 points
//     score += 500 * (count[5] / 3);
//     // Three 4's =>  400 points
//     score += 400 * (count[4] / 3);
//     // Three 3's =>  300 points
//     score += 300 * (count[3] / 3);
//     // Three 2's =>  200 points
//     score += 200 * (count[2] / 3);
//     // One   1   =>  100 points
//     score += 100 * (count[1] % 3);
//     // One   5   =>   50 point
//     score += 50 * (count[5] % 3);
//     
//     return score;
// }
