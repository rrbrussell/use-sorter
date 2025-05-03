#include <stdlib.h>
#include <stdio.h>

#include "atom.h"

extern char **environment;

int main(int argument_count, char* arguments[]) {
  // Setup
  atom_with_use_flags_t* at1 = newAtomWithUseFlags();
  setName(at1, "New Name");
  char* name = getName(at1);
  fputs(name, stdout);
  fputc('\n', stdout);
  fprintAtom(at1, stdout);
  setName(at1, "Bart Simpson");
  fprintAtom(at1, stdout);
  freeAtomWithUseFlags(at1);
  return 0;
}
