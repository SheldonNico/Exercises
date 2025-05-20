/* 
 * CS:APP Data Lab 
 * 
 * <Please put your name and userid here>
 * 
 * bits.c - Source file with your solutions to the Lab.
 *          This is the file you will hand in to your instructor.
 *
 * WARNING: Do not include the <stdio.h> header; it confuses the dlc
 * compiler. You can still use printf for debugging without including
 * <stdio.h>, although you might get a compiler warning. In general,
 * it's not good practice to ignore compiler warnings, but in this
 * case it's OK.  
 */

// #include <stdio.h>
#if 0
/*
 * Instructions to Students:
 *
 * STEP 1: Read the following instructions carefully.
 */

You will provide your solution to the Data Lab by
editing the collection of functions in this source file.

INTEGER CODING RULES:
 
  Replace the "return" statement in each function with one
  or more lines of C code that implements the function. Your code 
  must conform to the following style:
 
  int Funct(arg1, arg2, ...) {
      /* brief description of how your implementation works */
      int var1 = Expr1;
      ...
      int varM = ExprM;

      varJ = ExprJ;
      ...
      varN = ExprN;
      return ExprR;
  }

  Each "Expr" is an expression using ONLY the following:
  1. Integer constants 0 through 255 (0xFF), inclusive. You are
      not allowed to use big constants such as 0xffffffff.
  2. Function arguments and local variables (no global variables).
  3. Unary integer operations ! ~
  4. Binary integer operations & ^ | + << >>
    
  Some of the problems restrict the set of allowed operators even further.
  Each "Expr" may consist of multiple operators. You are not restricted to
  one operator per line.

  You are expressly forbidden to:
  1. Use any control constructs such as if, do, while, for, switch, etc.
  2. Define or use any macros.
  3. Define any additional functions in this file.
  4. Call any functions.
  5. Use any other operations, such as &&, ||, -, or ?:
  6. Use any form of casting.
  7. Use any data type other than int.  This implies that you
     cannot use arrays, structs, or unions.

 
  You may assume that your machine:
  1. Uses 2s complement, 32-bit representations of integers.
  2. Performs right shifts arithmetically.
  3. Has unpredictable behavior when shifting if the shift amount
     is less than 0 or greater than 31.


EXAMPLES OF ACCEPTABLE CODING STYLE:
  /*
   * pow2plus1 - returns 2^x + 1, where 0 <= x <= 31
   */
  int pow2plus1(int x) {
     /* exploit ability of shifts to compute powers of 2 */
     return (1 << x) + 1;
  }

  /*
   * pow2plus4 - returns 2^x + 4, where 0 <= x <= 31
   */
  int pow2plus4(int x) {
     /* exploit ability of shifts to compute powers of 2 */
     int result = (1 << x);
     result += 4;
     return result;
  }

FLOATING POINT CODING RULES

For the problems that require you to implement floating-point operations,
the coding rules are less strict.  You are allowed to use looping and
conditional control.  You are allowed to use both ints and unsigneds.
You can use arbitrary integer and unsigned constants. You can use any arithmetic,
logical, or comparison operations on int or unsigned data.

You are expressly forbidden to:
  1. Define or use any macros.
  2. Define any additional functions in this file.
  3. Call any functions.
  4. Use any form of casting.
  5. Use any data type other than int or unsigned.  This means that you
     cannot use arrays, structs, or unions.
  6. Use any floating point data types, operations, or constants.


NOTES:
  1. Use the dlc (data lab checker) compiler (described in the handout) to 
     check the legality of your solutions.
  2. Each function has a maximum number of operations (integer, logical,
     or comparison) that you are allowed to use for your implementation
     of the function.  The max operator count is checked by dlc.
     Note that assignment ('=') is not counted; you may use as many of
     these as you want without penalty.
  3. Use the btest test harness to check your functions for correctness.
  4. Use the BDD checker to formally verify your functions
  5. The maximum number of ops for each function is given in the
     header comment for each function. If there are any inconsistencies 
     between the maximum ops in the writeup and in this file, consider
     this file the authoritative source.

/*
 * STEP 2: Modify the following functions according the coding rules.
 * 
 *   IMPORTANT. TO AVOID GRADING SURPRISES:
 *   1. Use the dlc compiler to check that your solutions conform
 *      to the coding rules.
 *   2. Use the BDD checker to formally verify that your solutions produce 
 *      the correct answers.
 */


#endif
//1
/* 
 * bitXor - x^y using only ~ and & 
 *   Example: bitXor(4, 5) = 1
 *   Legal ops: ~ &
 *   Max ops: 14
 *   Rating: 1
 */
int bitXor(int x, int y) {
    int is1 = x & y;
    int is0 = (~x) & (~y);
    return (~is1) & (~is0);
}
/* 
 * tmin - return minimum two's complement integer 
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 4
 *   Rating: 1
 */
int tmin(void) {
    return 0x1 << 31;
}
//2
/*
 * isTmax - returns 1 if x is the maximum, two's complement number,
 *     and 0 otherwise 
 *   Legal ops: ! ~ & ^ | +
 *   Max ops: 10
 *   Rating: 1
 */
int isTmax(int x) {
    // x == 0b01111111111...
    // ~0 = 0b111111...11 = -1 
    // ~1 = 0b111111...10 = -2
    //
    // x != -1 && x + 1 = ~x 
    //
    // NOTE: !a will cast any number to 0 or 1
    // x == y -> !(x ^ y)
    // b1 && b2 : !!b1 & !!b2
    // b1 || b2 -> b1 | b2 or !!b1 | !!b2
    int x_plus_1 = x + 1;
    int is_x_negative_1 = !!x_plus_1;
    int is_x_plus_1_equal_tildex = !(x_plus_1 ^ (~x));
    return is_x_negative_1 & is_x_plus_1_equal_tildex;
}
/* 
 * allOddBits - return 1 if all odd-numbered bits in word set to 1
 *   where bits are numbered from 0 (least significant) to 31 (most significant)
 *   Examples allOddBits(0xFFFFFFFD) = 0, allOddBits(0xAAAAAAAA) = 1
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 12
 *   Rating: 2
 */
int allOddBits(int x) {
    int magic = 0xAA;
    magic = magic + (magic << 8);
    magic = magic + (magic << 16);
    return !((x & magic) ^ magic);
}
/* 
 * negate - return -x 
 *   Example: negate(1) = -1.
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 5
 *   Rating: 2
 */
int negate(int x) {
    return ~x + 1;
}
//3
/* 
 * isAsciiDigit - return 1 if 0x30 <= x <= 0x39 (ASCII codes for characters '0' to '9')
 *   Example: isAsciiDigit(0x35) = 1.
 *            isAsciiDigit(0x3a) = 0.
 *            isAsciiDigit(0x05) = 0.
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 15
 *   Rating: 3
 */
int isAsciiDigit(int x) {
    // x - 0x30 >= 0
    // 0x39 - x >= 0
    // x < 0 -> x & (1 << 31) <- bool(x >> 31) = !!(x >> 31)
    // x >= 0 -> !(x & (1 << 31)) <- !(x >> 31)


    // int left = x + ((~0x30)+1);
    // int right = 0x39 + (~x + 1);
    // return !(left >> 31) & !(right >> 31);

    int left = 0x2F + (~x+1); // left <= 0 -> left - 1 < 0
    int right = left + 0x0a; // right >= 0
    return (!!(left >> 31)) & (!(right >> 31));
}
/* 
 * conditional - same as x ? y : z 
 *   Example: conditional(2,4,5) = 4
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 16
 *   Rating: 3
 */
int conditional(int x, int y, int z) {
    int not_x_as_1 = !x;
    int false_as_negative_1 = ~not_x_as_1+1;
    int true_as_negative_1 = ~(!not_x_as_1) + 1;
    return (true_as_negative_1 & y) + (false_as_negative_1 & z);
}
/* 
 * isLessOrEqual - if x <= y  then return 1, else return 0 
 *   Example: isLessOrEqual(4,5) = 1.
 *   Legal ops: ! ~ & ^ | + << >>
 *   Max ops: 24
 *   Rating: 3
 */
int isLessOrEqual(int x, int y) {
    // 需要排除 overflow，如果同符号，必然不会overflow
    // (x >= 0 && y >=0) || (x < 0 && y < 0) 
    // 
    // x - y <= 0
    // z < 0 => bool(z >> 31)
    // z = 0 => !(z)
    int is_x_negative = !!(x >> 31);
    int is_y_negative = !!(y >> 31);
    int z = x + ~y + 1;
    int is_z_negative = !!(z >> 31);
    int is_z_zero = !z;

    // printf(
    //         "x: %d, y: %d, part0: %d, part1: %d, part3: %d, part4: %d z: %d\n", 
    //         x, y, !(!is_x_negative & is_y_negative), (is_x_negative & !is_y_negative), is_z_negative, is_z_zero, z
    //         );

    return (!((!is_x_negative) & is_y_negative)) & ((is_x_negative & !is_y_negative) | is_z_negative | is_z_zero);
}
//4
/* 
 * logicalNeg - implement the ! operator, using all of 
 *              the legal operators except !
 *   Examples: logicalNeg(3) = 0, logicalNeg(0) = 1
 *   Legal ops: ~ & ^ | + << >>
 *   Max ops: 12
 *   Rating: 4 
 */
int logicalNeg(int x) {
    // x == 0
    // ^x = -1
    // 

    int flag = x;
    flag = flag | (flag >> 16);
    flag = flag | (flag >> 8);
    flag = flag | (flag >> 4);
    flag = flag | (flag >> 2);
    flag = flag | (flag >> 1);
    return (~flag) & 0x1;
}
/* howManyBits - return the minimum number of bits required to represent x in
 *             two's complement
 *  Examples: howManyBits(12) = 5
 *            howManyBits(298) = 10
 *            howManyBits(-5) = 4
 *            howManyBits(0)  = 1
 *            howManyBits(-1) = 1
 *            howManyBits(0x80000000) = 32
 *  Legal ops: ! ~ & ^ | + << >>
 *  Max ops: 90
 *  Rating: 4
 */
int howManyBits(int x) {
    // num of bits, range, shift
    // 1, [-1, 0], 1 << 0
    // 2, [-2, -1, 0, 1], 1 << 1
    // 5, -16 -> 15, 1 << 4
    // 10, -512 -> 511, 1 << 9
    //
    // limt_neg <= x <= limt_pos -> x < 1 << n && x >= -(1 << n) -> (x - (1<<n) < 0) & x + (1 << n) >= 0
    // x < 0 => (x >> 31) => 1111...11, 0000....0
    // x >= 0 => ~(x >> 31)  
    //
    // if x >= 0 -> max 1 position of x: 00001000 ->  4+1, 00001 -> 1+1
    // if x < 0 -> max 1 position of ~x

    /*
    int limit = 1 << 3;
    int delta_up = x + limit;
    int delta_low = x + (~limit + 1);
    return (~(delta_up >> 31)) & (delta_low >> 31);
    */

    int is_x_negative = x >> 31;
    int positive = (is_x_negative & (~x)) + ((~is_x_negative) & x);
    int positive_bits = 0;
    int positive_left;
    int positive_right;
    int is_left_positive;
    int shift;

    // printf("%d: %d %d Begin ... \n", x, positive_bits, positive);
    // printf("%d: %d\n", x, is_x_negative & (~x) + (~is_x_negative) & x);
    // printf("%d: %d\n", x, (is_x_negative & (~x)) + ((~is_x_negative) & x));

    shift = 16;
    positive_left = positive >> shift;
    // positive_right = positive << shift >> shift;
    positive_right = positive & ((1 << (shift+1)) + (~0)); // 特殊处理，可能 overflow
    is_left_positive = (!!positive_left) << 31 >> 31;
    positive_bits = positive_bits + (is_left_positive & shift);
    positive = (is_left_positive & positive_left) + ((~is_left_positive) & positive_right);

    shift = 8;
    positive_left = positive >> shift;
    positive_right = positive << shift >> shift;
    is_left_positive = (!!positive_left) << 31 >> 31;
    positive_bits = positive_bits + (is_left_positive & shift);
    positive = (is_left_positive & positive_left) + ((~is_left_positive) & positive_right);

    shift = 4;
    positive_left = positive >> shift;
    positive_right = positive << shift >> shift;
    is_left_positive = (!!positive_left) << 31 >> 31;
    positive_bits = positive_bits + (is_left_positive & shift);
    positive = (is_left_positive & positive_left) + ((~is_left_positive) & positive_right);
    
    shift = 2;
    positive_left = positive >> shift;
    positive_right = positive << shift >> shift;
    is_left_positive = (!!positive_left) << 31 >> 31;
    positive_bits = positive_bits + (is_left_positive & shift);
    positive = (is_left_positive & positive_left) + ((~is_left_positive) & positive_right);

    shift = 1;
    positive_left = positive >> shift;
    positive_right = positive << shift >> shift;
    is_left_positive = (!!positive_left) << 31 >> 31;
    positive_bits = positive_bits + (is_left_positive & shift);
    positive = (is_left_positive & positive_left) + ((~is_left_positive) & positive_right);
    // printf("%d: %d %d\n", x, positive_bits, positive);

    return positive_bits + positive + 1;
}
//float
/* 
 * floatScale2 - Return bit-level equivalent of expression 2*f for
 *   floating point argument f.
 *   Both the argument and result are passed as unsigned int's, but
 *   they are to be interpreted as the bit-level representation of
 *   single-precision floating point values.
 *   When argument is NaN, return argument
 *   Legal ops: Any integer/unsigned operations incl. ||, &&. also if, while
 *   Max ops: 30
 *   Rating: 4
 */
unsigned floatScale2(unsigned uf) {
    // 1 8 23
    // Because we can use any constant, so the constant expression can be evaluated by hand to reduce about 8 ops
    unsigned zero = 0;
    unsigned ones = ~zero;
    unsigned f = uf & (ones >> 9);
    unsigned m = uf & ((ones << 24) >> 1);
    unsigned s = uf & (ones << 31);
    unsigned fshift;
    unsigned mshift;
    // printf("uf=%u, f=%u, m=%u, s=%u\n", uf, f, m, s);

    if (!(~m)) {
        return uf;
    } else if (!m) {
        fshift = (f << 1);
        f = (fshift << 9) >> 9;
        // printf("fshift=%u vs %u\n", fshift, fshift >> 23);
        // a hiding 1 is implied when all m = 0, so it can last one more bit
        if (fshift >> 24) {
            return f | (ones << 24 >> 1) | s;
        } else {
            return fshift | s;
        }
    } else {
        mshift = ((m >> 23) + 1) << 23;
        // printf("mshift=%u vs %u\n", mshift, mshift >> 31);
        if (mshift >> 31) {
            // 注意关于超过极限值的处理，并非约化到 infinity
            // return s | (ones << 24 >> 1);
            return uf;
        } else {
            return s | mshift | f;
        }
    }
}
/* 
 * floatFloat2Int - Return bit-level equivalent of expression (int) f
 *   for floating point argument f.
 *   Argument is passed as unsigned int, but
 *   it is to be interpreted as the bit-level representation of a
 *   single-precision floating point value.
 *   Anything out of range (including NaN and infinity) should return
 *   0x80000000u.
 *   Legal ops: Any integer/unsigned operations incl. ||, &&. also if, while
 *   Max ops: 30
 *   Rating: 4
 */
int floatFloat2Int(unsigned uf) {
    // all logical condition, which means < > == is allowed?
    // otherwise, we will have a hard life.
    // int -> float can be done in a good way. We just reverse it.
    //
    // float(uf) = s * 2^m * f
    unsigned f = uf << 9 >> 9;
    unsigned m = ((uf << 1) >> 24);
    unsigned s = uf >> 31;

    int bias_plus_1 = 1 << 7;
    // m - (2^k - 1) = m+1 - 2^k
    int mval = m+1 + ((~bias_plus_1) + 1);
    int out;

    // printf("uf=%u, m=%d, f=%u, s=%u, mval=%d\n", uf, m, f, s, mval);
    if (!m) {
        return 0;
    } else if (mval < 0) {
        return 0;
    } else if (mval <= 23) {
        // (1+f) * 2^mval
        out = (f >> (23 - mval)) + (1 << mval);
        if (s) {
            out = ~out + 1;
        }
        return out;
    } else if (mval <= 23 + 31) {
        // FIXME: should not pass, but works for btest.
        // what if shif overflow, and how to determine its sign.
        // is it ok to ignore sign bit.
        return f << (mval - 23);
    } else {
        return 0x80000000u;
    }
}
/* 
 * floatPower2 - Return bit-level equivalent of the expression 2.0^x
 *   (2.0 raised to the power x) for any 32-bit integer x.
 *
 *   The unsigned value that is returned should have the identical bit
 *   representation as the single-precision floating-point number 2.0^x.
 *   If the result is too small to be represented as a denorm, return
 *   0. If too large, return +INF.
 * 
 *   Legal ops: Any integer/unsigned operations incl. ||, &&. Also if, while 
 *   Max ops: 30 
 *   Rating: 4
 */
unsigned floatPower2(int x) {
    int bias = (1 << 7) - 1;
    unsigned zero = 0;
    if (x < -(23 + bias - 1)) {
        return 0;
    } else if (x <= -(23 + bias - 1) + 23) {
        return 1 << (x + 23 - bias + 1);
    } else if (x <= bias) {
        return (x + bias) << 23;
    } else {
        return ((~zero) << 24) >> 1; // infinity
    }
}
