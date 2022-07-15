#include <err.h>
#include <stdio.h>

#include "weggli.h"

#define DEMO_SOURCE \
  "void foo() {int foo = 3; uint16_t foo = 3; unsigned long foo = 3;}"

bool iter_captures(size_t start, size_t end, void *userdata) {
  printf("span: (%zu, %zu]\n", start, end);

  return true;
}

bool iter_variables(const char *name, size_t start, size_t end, void *userdata) {
  printf("variable %s: span: (%zu, %zu])\n", name, start, end);

  return true;
}

bool iter_matches(const QueryResult *result, void *userdata) {
  printf("%s\n", "match!");

  printf("======== raw captures ========\n");
  weggli_iter_match_captures(result, iter_captures, NULL);
  printf("====== end raw captures ======\n");

  printf("======== var captures ========\n");
  weggli_iter_match_variables(result, iter_variables, NULL);
  printf("====== end var captures ======\n");

  return true;
}

int main(int argc, char const *argv[]) {
  QueryTree *qt = weggli_new_query("{ $t $a = 3; }", false);

  if (qt == NULL) {
    errx(0, "weggli_new_query failed");
  }

  QueryResults *matches = weggli_matches(qt, DEMO_SOURCE, sizeof(DEMO_SOURCE) - 1, false);

  if (matches == NULL) {
    errx(0, "weggli_matches failed");
  }

  weggli_iter_matches(matches, iter_matches, NULL);

  weggli_destroy_matches(matches);
  weggli_destroy_query(qt);
  return 0;
}
