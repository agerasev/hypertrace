#include <macros.hh>
#include <types.hh>

#if !defined($Self) || !defined($self) || !defined($Element) || !defined($element_size)
#error "All required macro parameters must be defined."
#endif

typedef struct $Self {
    usize size;
#if $element_size != 0
    $Element elements[1];
#endif
} $Self;

_ALLOW_MULTIPLE_DEFINITIONS_
$Element *$2($self,__element)($Self *self, usize index) {
#if $element_size != 0
    return &self->elements[index];
#else
    return NULL;
#endif
}

_ALLOW_MULTIPLE_DEFINITIONS_
const $Element *$2($self,__element__c)(const $Self *self, usize index) {
#if $element_size != 0
    return &self->elements[index];
#else
    return NULL;
#endif
}

_ALLOW_MULTIPLE_DEFINITIONS_
__global $Element *$2($self,__element__g)(__global $Self *self, usize index) {
#if $element_size != 0
    return &self->elements[index];
#else
    return NULL;
#endif
}

_ALLOW_MULTIPLE_DEFINITIONS_
__global const $Element *$2($self,__element__gc)(__global const $Self *self, usize index) {
#if $element_size != 0
    return &self->elements[index];
#else
    return NULL;
#endif
}
