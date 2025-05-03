#include <stdlib.h>
#include <stdio.h>
#include "memory_helper.h"

void* allocateOrFail(size_t count, size_t size) {
  void* temp_pointer = calloc(count, size);
  if (temp_pointer == NULL) {
      perror("use-sorter");
      abort();
    }
  return temp_pointer;
}
