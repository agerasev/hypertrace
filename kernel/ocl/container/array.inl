#include <macros.hh>

#if !defined($Self) || !defined($Elem) || !defined($N)
#error "All required macro parameters must be defined."
#endif

typedef struct $Self {
    $Elem elements[$N];
} $Self;
