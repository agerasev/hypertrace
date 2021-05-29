#include <macros.hh>

#if !defined($Self) || !defined($Element) || !defined($size)
#error "All required macro parameters must be defined."
#endif

typedef struct $Self {
    $Element elements[$size];
} $Self;
