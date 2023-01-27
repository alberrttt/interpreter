#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct BytecodeFunction BytecodeFunction;

typedef struct InternedString {
  uintptr_t _0;
} InternedString;

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
    const struct BytecodeFunction *function;
  };
  struct {
    Value_Tag array_tag;
    const union Value *array;
  };
} Value;



extern union Value sum(union Value a, union Value b);
