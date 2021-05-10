#include <macros.hh>
#include <types.hh>

#if !defined($Self) || !defined($self) || !defined($Elem)
#error "All required macro parameters must be defined."
#endif

typedef struct $Self {
    usize size;
    $Elem elements[0];
} $Self;

_ALLOW_MULTIPLE_DEFINITIONS_
$Elem *$2($self,__elements)($Self *self, usize index) {
    return self->elements[index];
}

_ALLOW_MULTIPLE_DEFINITIONS_
const $Elem *$2($self,__elements__c)(const $Self *self, usize index) {
    return self->elements[index];
}

_ALLOW_MULTIPLE_DEFINITIONS_
__global $Elem *$2($self,__elements__g)(__global $Self *self, usize index) {
    return self->elements[index];
}

_ALLOW_MULTIPLE_DEFINITIONS_
__global const $Elem *$2($self,__elements__gc)(__global const $Self *self, usize index) {
    return self->elements[index];
}
