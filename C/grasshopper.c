#include <stdbool.h>

bool check_for_factor(int base, int factor) {
  if(base % factor != 0) {
      return false;
  }
  else {
      return true;
  }
}

