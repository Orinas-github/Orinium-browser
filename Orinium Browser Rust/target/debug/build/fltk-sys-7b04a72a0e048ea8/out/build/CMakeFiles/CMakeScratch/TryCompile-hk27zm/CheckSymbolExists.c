/* */
#include <locale.h>

int main(int argc, char** argv)
{
  (void)argv;
#ifndef localeconv
  return ((int*)(&localeconv))[argc];
#else
  (void)argc;
  return 0;
#endif
}
