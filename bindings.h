#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Rc_RefCell_Function Rc_RefCell_Function;

typedef struct Rc_RefCell_Vec_Value Rc_RefCell_Vec_Value;

typedef struct InternedString {
  uintptr_t _0;
} InternedString;

typedef struct Rc_RefCell_Function Ptr_Function;

typedef struct Rc_RefCell_Vec_Value Ptr_Vec_Value;

enum Value_Tag {
  Number,
  Boolean,
  String,
  Function,
  Array,
  Void,
  None,
};
typedef uint8_t Value_Tag;

typedef union Value {
  Value_Tag tag;
  struct {
    Value_Tag number_tag;
    double number;
  };
  struct {
    Value_Tag boolean_tag;
    bool boolean;
  };
  struct {
    Value_Tag string_tag;
    struct InternedString string;
  };
  struct {
    Value_Tag function_tag;
    Ptr_Function function;
  };
  struct {
    Value_Tag array_tag;
    Ptr_Vec_Value array;
  };
} Value;



extern int32_t sum(int32_t a, int32_t b);
