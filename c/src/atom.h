#ifndef __ATOM_H
#define __ATOM_H

typedef struct atom_with_use_flags {
  char* name;
  char** use_flags;
} atom_with_use_flags_t;

atom_with_use_flags_t* newAtomWithUseFlags();
void freeAtomWithUseFlags(atom_with_use_flags_t* to_destroy);
char* getName(atom_with_use_flags_t* this);
void setName(atom_with_use_flags_t* this, char* new_name);
void fprintAtom(atom_with_use_flags_t* this, FILE* output);

#endif
