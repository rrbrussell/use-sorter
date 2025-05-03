#include <stdio.h>
#include <string.h>

#include "atom.h"
#include "memory_helper.h"

const size_t STRING_LENGTH = 64;
const size_t NUMBER_OF_USE_FLAGS = 64;

atom_with_use_flags_t*  newAtomWithUseFlags() {
  atom_with_use_flags_t* temp = allocateOrFail(1,
                                               sizeof(atom_with_use_flags_t));
  temp->name = (char *) allocateOrFail(STRING_LENGTH + 1, sizeof(char));
  temp->use_flags = (char **) allocateOrFail(NUMBER_OF_USE_FLAGS + 1,
                                             sizeof(char*));
  return temp;
}

void freeAtomWithUseFlags(atom_with_use_flags_t* to_destroy) {
  free(to_destroy->name);
  char** temp = to_destroy->use_flags;
  int i = 0;
  while(temp[i] != NULL) {
    free(temp[i]);
    i++;
  }
  free(to_destroy);
  to_destroy = NULL;
}

/*
  The returned pointer must be freed by the caller.

  A NULL return value means that this did not have any set name.
 */
char* getName(atom_with_use_flags_t* this) {
  char* new_str = allocateOrFail(STRING_LENGTH + 1, sizeof(char));
  if (this->name != NULL) {
    strncpy(new_str, this->name, STRING_LENGTH);
    return new_str;
  } else {
    return NULL;
  }
}

/*
  Ownership of new_name's contents is not moved to this. The caller must clean
  up new_name appropriately
*/
void setName(atom_with_use_flags_t* this, char* new_name) {
  if (this->name == NULL) {
    this->name = allocateOrFail(STRING_LENGTH + 1, sizeof(char));
  }
  strncpy(this->name, new_name, STRING_LENGTH);
}

void fprintAtom(atom_with_use_flags_t* this, FILE* output) {
  if (this->name != NULL) {
    fputs(this->name, output);
    fputc('\n', output);
  }
}
