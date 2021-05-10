#include <macros.hh>
#include <types.hh>

#if !defined($Self) || !defined($self) || !defined($Elem)
#error "All required macro parameters must be defined."
#endif

typedef struct $Self {
    usize size;
    $Elem elements[1];
} $Self;

_ALLOW_MULTIPLE_DEFINITIONS_
$Elem *$2($self,__element)($Self *self, usize index) {
    return &self->elements[index];
}

_ALLOW_MULTIPLE_DEFINITIONS_
const $Elem *$2($self,__element__c)(const $Self *self, usize index) {
    return &self->elements[index];
}

_ALLOW_MULTIPLE_DEFINITIONS_
__global $Elem *$2($self,__element__g)(__global $Self *self, usize index) {
    return &self->elements[index];
}

_ALLOW_MULTIPLE_DEFINITIONS_
__global const $Elem *$2($self,__element__gc)(__global const $Self *self, usize index) {
    return &self->elements[index];
}
