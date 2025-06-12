#import "@preview/codly:1.3.0": *

= Chapter 2
== 2.2
#table(
  columns: (auto, auto, auto),
  align: right,
  table.header([n], $ 2^n "(decimal)" $, $ 2^n ("hexadecimal") $),

  $5$ , $32$, [0x20],
  $23$, $8388608$, [0x80000],
  $15$, $32758$, [ 0x8000 ],
  $13$, $#calc.pow(2, 13)$, [ 0x2000 ],
  $12$, $#calc.pow(2, 12)$, [ 0x1000 ],
  $#calc.log(64, base:2)$, $64$, [ 0x40 ],
  $8$, $#calc.pow(2, 8)$, [ 0x100 ],
)

== 2.3
#table(
  columns: 3,
  [Decimal], [Binary], [Hexadecimal],
  $0$, $0000 \_ 0000$, [0x00],
  $158$, $1001 \_ 1110$, [0x9E],
  $76$ ,   $1001100$  , [0x4C],
  $145$  , $10010001$ , [0x91],
  $174$  , $10101110$ , [0xAE],
  $60$   , $111100$   , [0x3C],
  $241$  , $11110001$ , [0xF1],
)

#table(
  columns: 3,
  [Decimal], [Binary], [Hexadecimal],
  $117$, $1110101$, [0x75],
  $189$, $10111101$, [0xBD],
  $245$, $11110101$, [0xF5],
)

== 2.4
#[
#set enum(numbering: "A.")
+ 0x605c + 0x5 = 0x605$[ 5+c ]$ = 0x60$[ 5+1 ]$$[1]$ = 0x6061 \
+ 0x605c - 0x20 = 0x60$[ 5-2 ]$$[c - 0]$ = 0x60$[ 3 ]$$[c]$ = 0x603c \
]

== 2.5
#[
#set enum(numbering: "A.")
+ Little endian: #underline[be] Big dndian: #underline[07]
+ Little endian: #underline[be c8] Big dndian: #underline[07 5b]
+ Little endian: #underline[be c8 5b] Big dndian: #underline[07 5b c8]
]

== 2.6
#[
#set enum(numbering: "A.")
+ Little endian(should be reversed) $ 2607352"i32" &= 11111000 \_ 11001000 \_ 00100111 \_ 00000000 \ 2607352.0"f32" &= 11100000 \_ 00100011 \_ 00011111 \_ 01001010 $
+ 20 bits match $
00000000001 & 00111110010001111000 \
            &"********************" \
  010010100 & 0011111001000111100000
$
+ Head of integer and float does not match. Tail match the smallest length.
]

== 2.7
#align(left,
$
  m ,&& n   ,&& o   ,&& p   ,&& q   ,&& r   ,&& 0 \
109 ,&& 110 ,&& 111 ,&& 112 ,&& 113 ,&& 114 ,&& 0 \
6d  ,&& 6e  ,&& 6f  ,&& 70  ,&& 71  ,&& 72  ,&& 0 \
$
)

== 2.8
#table(
  columns: 2,
  [Operation], [Result], 
  $a$, $01001110$,
  $b$, $11100001$,
  $~a$, $10110001$,
  $~b$, $00011110$,
  $a \& b$, $01000000$,
  $a bar b$, $11101111$,
  $a \^ b$, $10101111$,
)

== 2.9
#[
#set enum(numbering: "A.")
+ table of ~ operations #table(
  columns: 3,
  [operation]  , [result] , [color]   ,
  [\~black]    , $111$    , [white]   ,
  [\~blue]     , $110$    , [yellow]  ,
  [\~green]    , $101$    , [magenta] ,
  [\~cyan]     , $100$    , [red]     ,
)
+ $ 
"Blue" bar "Green" &= 011 = "Cyan" \ 
"Yello" amp "Cyan" &= 010 = "Green" \ 
"Red" hat "Magenta" &= 001 = "Blue" 
$
]

== 2.10
#table(
  columns: 3,
  [Step], $ast x$, $ast y$,
  [Initially], $a$, $b$,
  [Step 1], $a$, $a hat b$,
  [Step 2], $a hat (a hat b) = b$, $a hat b$,
  [Step 3], $b$, $b hat (a hat b) = a$,
)

== 2.11
#[
#set enum(numbering: "A.")
+ the element at k, mark it as $a_k$
+ $a_k hat a_k = 0$
+ modify #text(fill: green)[$"first" <= "last" arrow.double "first" < "last"$]  in line 4, to skip the last item 
]

== 2.12
#[
#set enum(numbering: "A.")
+ $x space amp "0xff"$
+ $ (tilde.basic x) space amp (tilde.basic "0xff") bar (x space amp "0xff")$
+ $ x space amp (tilde.basic "0xff") bar ("0xff")$
]

== 2.13
given thorem ref to https://en.wikipedia.org/wiki/Exclusive_or

$ a hat b = (a amp (tilde.basic b)) bar (b amp (tilde.basic a)) $

```c
int bis(int x, int m) {
  (x | m)
}

int bic(int x, int m) {
  a & (~b)
}

int bool_or(int x, int y) {
  return bis(x, y);
}

int bool_xor(int x, int y) {
  return bit_or(bic(x, y), bic(y, x));
}
```

== 2.14
$ a &= 01010101 \ b &= 01000110 $

#table(
  columns: 4,
  [Expression], [Value], [Expression], [Value],
  $a amp b$, $01000100 = "0x" 44$, $a amp amp b$, $1 amp amp 1 = 1$,
  $a bar b$, $01010111 = "0x" 57$, $a bar.v.double b$, $1 bar.v.double 1 = 1$,
  $tilde.basic a bar tilde.basic b$, $10101010 bar 10111001 = 10111011 = "0xbb"$, $excl a bar.v.double excl b$, $0 bar.v.double 0 = 0$,
  $a amp excl b$, $01010101 amp 1 = 1 = "0x01"$, $a amp amp tilde.basic b$, $1 amp amp 1 = 1$,
)

== 2.15
$
x eq y arrow.l.r.double (x hat y) eq 0
$

== 2.16
#[

#let hcell(body) = table.cell(colspan: 2, align: center + bottom, body)

#table(
  columns: 8,
  hcell[$a$], hcell[$a lt.double 2$], hcell[$ "Logical" \ a gt.double 3 $], hcell[$ "Arithmetic" \ a gt.double 3 $],
  [Hex], [Binary], [Hex], [Binary], [Hex], [Binary], [Hex], [Binary],
  [0xD4], $11010100$, $01010000$, [0x50], $00011010$, [0x1A], $11111010$, [0xFA],
  [0x64], $01100100$, $10010000$, [0x90], $00001100$, [0x0C], $00001100$, [0x0C],
  [0x72], $01110010$, $11001000$, [0xc8], $00001110$, [0x0E], $00001110$, [0x0E],
  [0x44], $01000100$, $00010000$, [0x10], $00001000$, [0x08], $00001000$, [0x08],
)
]

== 2.17
#[
#let xbar = $accent(x, arrow)$

#table(
  columns: 4,
  table.cell(colspan: 2)[$ xbar $], [], [],
  [Hexadecimal], [Binary], $"B2U"_4 (xbar)$, $"B2T"_4 (xbar)$,
  [0x1], $0001$, $2^0=1$, $0+2^0=1$,
  [0xB], $1011$, $2^3+2^1+2^0=11$, $-2^3+2^1+2^0=-5$,
  [0x2], $0010$, $2^1=2$, $0+2^1=2$,
  [0x7], $0111$, $2^2+2^1+2^0=7$, $2^2+2^1+2^0=7$,
  [0xC], $1100$, $2^3+2^2=12$, $-2^3+2^2=-4$,
)
]

== 2.18
#[
#set enum(numbering: "A.")

+ $ "0x000002e0" = 00000000000000000000001011100000 = 736 $
+ $ "0xffffffa8" = 11111111111111111111111110101000 = -88 $
+ $ "0x00000028" = 00000000000000000000000000101000 = 40 $
+ $ "0xffffffd0" = 11111111111111111111111111010000 = -48 $
+ $ "0x00000078" = 00000000000000000000000001111000 = 120 $
+ $ "0x00000088" = 00000000000000000000000010001000 = 136 $
+ $ "0x000001f8" = 00000000000000000000000111111000 = 504 $
+ $ "0x00000008" = 00000000000000000000000000001000 = 8 $
+ $ "0x000000c0" = 00000000000000000000000011000000 = 192 $
+ $ "0xffffffb8" = 11111111111111111111111110111000 = -72 $
]

== 2.19
#table(
  columns: 2,
  $x$, $"T2U"_4(x)$,
  $-1$, $16+ (-1) = 15$,
  $-5$, $16+ (-5) = 11$,
  $-6$, $16+ (-6) = 10$,
  $-4$, $16+ (-4) = 12$,
  $1$, $1$,
  $8$, $8$,
)

=== Data::tmin
+ Practice Problem 1: 存在隐式转化，当赋值结束后，类型都是 `int`，由于转换后 binary 的形式不变，他们都表达了 -2147483648，所以结果都是0
+ Practice Problem 2: 应该是可以的，由于 hex 同样不需要进行到类型转化这一步
+ Practice Problem 3
  + Explained: 1. `<<` 的优先级低于 +/-/\*/\/，所以表达式变成 $1"L" << 4 << (3 - 1)$ 2. `<<` 从左计算，所以表达式变成 $(1"L" << 4) << 2 = 1 << 6 = 2 ^ 6 = 64$
  + for 64-bit similar: $1"L" << 8 << 2 = 2 ^ 10 = 1024$
  + just add parents: `(1L << (sizeof(long) << 3)) - 1`
+ Practice Problem 4: #table(
  columns: 4,
  [Word Size], [C Version], [-9223372036854775808], [0x8000000000000000],
  [32], [C90], [`undefined`], [`undefined`],
  [32], [C99], [`undefined`], [`unsigned long long`],
  [64], [C90], [`unsigned long`], [`unsigned long`],
  [64], [C99], [`undefined`], [`unsigned long`],
)

== 2.21
#table(
  columns: 3,
  [Expression], [Type], [Evaluation],
  `-2147483647-1 == 2147483648U`, [Unsigned], $ "T2U"_32 ("tmin") &= 2 ^ 31 \ "tmin" + 2^32 &= 2 ^ 31 \ &= "true" $,
  `-2147483647-1 < 2147483647`, [Signed], $ "tmin" < "tmax" arrow "true" $,
  `-2147483647-1U < 2147483647`, [Unsigned], $ "T2U" (-(2^(w-1) - 1)) - 1 &< "T2U"( "tmax" ) \ 2^(w-1) + 1 - 1 &< "tmax" \ 2^(w-1) &< 2^(w-1) - 1 \ &= "false" $,
  `-2147483647-1 < -2147483647`, [Signed], $ "tmin" < -"tmax" arrow "true" $,
  `-2147483647-1U < -2147483647`, [Unsigned], $ "T2U" (-(2^(w-1) - 1)) - 1 &< "T2U"( -"tmax") \ 2^(w-1) &< -(2^(w-1) - 1) + 2^w \ 2^(w-1) &< 2^(w-1) + 1 \ &= "true" $,
)

== 2.22
#[
#set enum(numbering: "A.")
+ $1100_2 = -2^3 + 2^2 = -8 + 4 = -4$
+ $11100_2 = -2^4 + 2^3 + 2^2 = -16 + 8 + 4 = -4$
+ $111100_2 = -2^5 + 2^4 + 2^3 + 2^2 = -32 + 16 + 8 + 4 = -4$
]

== 2.23
+ results: #table(
  columns: 3,
  $w$, $"func1" (w)$, $"func2" (w)$,
  `0x00000076`, [`0x00000076` (118)], [`0x77777776` (2004318070)],
  `0x87654321`, [`0x00000021` (33)], [`0x22222221` (572662305)],
  `0x000000C9`, `0x000000C9`, `0xCCCCCCC9`,
  `0xEDCBA987`, `0x00000087`, `0x88888887`,
)
+ func1: $"out" *= 2^24 -> "out" "/=" 2^24 -> "U2T"("out") $; func2: $"U2T"("out") -> "out" *= 2^24 -> "out" "/=" 2^24$, the difference lies in the order of conversion to signed int

== 2.24
#[
#let hcell(body) = table.cell(colspan: 2, align: center + bottom, body)

#table(
  columns: 6,
  hcell[Hex], hcell[Unsigned], hcell[Two's complement],
  [Original], [Truncated], [Original], [Truncated], [Original], [Truncated],
  [1 = $0001$], [1], [1], [1], [1], [1],
  [3 = $0011$], [3], [3], [3], [3], [3],
  [5 = $0101$], [5], [5], [5], [5], $5 - 8 = -3$,
  [C = $1100$], [4], [12], $12 mod 8 = 4$, [-4], $4 - 8 = -4$,
  [E = $1110$], [6], [14], $14 mod 8 = 6$, [-2], $6 - 8 = -2$,
)
]

== 2.25
when $"length" = 0$, then for iteration condition check over 
$ i &< 0_"unsigned" - 1 \ &< 4294967295 $
that's what we called overflow, then in for body we try to access `a[1]`, which will then causing a segmentfault.

== 2.26
#[
#set enum(numbering: "A.")
+ any one `strlen(s) < strlen(t)`, since this function always return 1 except `strlen(s) = strlen(t)`.
+ since unsigned math always return unsigned, so the expression shoule return 1 except when they are equal.
+ just change to #text(fill: green)[`return strlen(s) > strlen(t)`].
]

== 2.27
```c
int uadd_ok(unsigned x, unsigned y) {
  int result = x + y;
  return !(result < x);
  // return !(result < y);
}
```

== 2.28 <h228>
#[
#let hcell(body) = table.cell(colspan: 2, align: center + bottom, body)

#table(
  columns: 4,
  hcell[$ x $], hcell[$ -_4^u x $], 
  [Hex], [Decimal], [Decimal], [Hex],
  $1 = 0001$, [1], [15], $F=1111$,
  $4 = 0100$, [4], [12], $B=1100$,
  $7 = 0111$, [7], [9],  $9=1001$,
  $A = 1010$, [10], [6], $6=0110$,
  $E = 1110$, [14], [2], $2=0010$,
)
]

== 2.29
#[
#let cell(body) = table.cell(align: center, body)

#table(
  columns: 5,
  $ x $, $ y $, $ x+y $, $ x +_5^t y $, [Case],
  cell[ $-2^4 + 2^2 /*= -16 + 4 */ = -12$ \ 10100 ], cell[ $-2^4 + 1=-15$ \ 10001 ], cell[ $-2^5+2^2+1=-27$ \ 100101 ], cell[ $2^2+1=5$ \ 00101], cell[ 1 ],
  cell[ $-2^4 + 2^3 /*= -16 + 8 */ = -8$ \ 11000 ], cell[ $-8$ \ 11000 ], cell[ $-16$ \ 110000 ], cell[ $-16$ \ 10000], cell[ 2 ],
  cell[ $-2^4 + 2^2 + 2 + 1 /*= -16 + 7 */ = -9$ \ 10111 ], cell[ $2^3 = 8$ \ 01000 ], cell[ $-1$ \ 11111 ], cell[ $-1$ \ 11111], cell[ 2 ],
  cell[ $2$ \ 00010 ], cell[ $2^2 + 1 = 5$ \ 00101 ], cell[ $7$ \ 00111 ], cell[ $7$ \ 00111], cell[ 3 ],
  cell[ $2^3+2^2=12$ \ 01100 ], cell[ $2^2 = 4$ \ 00100 ], cell[ $16$ \ 10000 ], cell[ $-16$ \ 10000], cell[ 4 ],
)
]

== 2.30
```c
int tadd_ok(int x, int y) {
  int r = x + y;
  bool not_ok = (x > 0 && y > 0 && r < 0) || (x < 0 && y < 0 && r > 0);
  return !not_ok;
}
```

== 2.31
从条件约束上看是没问题的，将case1的符号带入是能够发现不满足的，但 `sum - x` 涉及到减法，自然又会有 overflow 的问题，这里
没有 `-` 的实现细节，如果我们认为满足加减法的结合律的话，那么比如 `sum < 0` 同时 `x > 0`，为了让 `sum` overflow，我们假
设 `sum = TMIN`，那么得到的结果为正数，与检测定理同符号。实际上由于 `-` 是由 `+` 推导出来的，这个检测函数永远都是返回 `true`

== 2.32
问题不在表达式上，问题再 Two's complement 的数据范围不是对称的，如果 $y = -"TMax"_w$, $x = 0$，那么减法必定 overflow，但由于 
$-2^w lt.eq -y lt 2^w$，后者不会检测出 overflow

== 2.33 <h233>
#[
#let hcell(body) = table.cell(colspan: 2, align: center + bottom, body)

#table(
  columns: 4,
  hcell[$ x $], hcell[$ -_4^t x $], 
  [Hex], [Decimal], [Decimal], [Hex],
  $2="0x0010"$, [2], [-2], $"0x1110"=E$,
  $3="0x0011"$, [3], [-3], $"0x1101"=D$,
  $9="0x1001"$, [-7], [7], $"0x0111"=7$,
  $B=11="0x1011"$, [-5], [5], $"0x0101"=5$,
  $C=12="0x1100"$, [-4], [4], $"0x0100"=4$,

  $8="0x1000"$, [-8], [-8], $"0x1000"=0$,
)
]

除开 0x0000，其余都满足 $x + -_4^t x = 100000$，二进制下向前进一位。

=== Data::tneg
+ Practice Problem 1: #table(
  columns: 3,
  $ accent(x, arrow) $, $ tilde accent(x, arrow) $, $ "incr"(tilde accent(x, arrow)) $,
  $01101=13$, $10010=-14$, $10011=-13$,
  $01110=14$, $10001=-15$, $10010=-14$,
  $11000=-8$, $00111=7$, $01000=8$,
  $11111=-1$, $00000=0$, $00001=1$,
  $10000=-16$, $01111=15$, $10000=-16$,
)
+ Practice Problem 2: rely on 1. bit level $"decr(accent(x, arrow))"$ is the same as $x -_w^t 1$, should be prove similar to previous principle 2. $-x = tilde x + 1$. $
"B2T"_w (tilde (accent(x, arrow) -1)) &= "B2T"_w (tilde (accent(x - 1, arrow))) \
                                      &= -1 - (x-1) \
                                      &= -x
$
+ Practice Problem 3: #table(
  columns: 2,
  $ x $, $ -x $, 
  [ #emph("1001")1=13 ], [#emph("1001")1 $=-16+2+1=13$ ],
  [ #emph("011")10=14], [#emph("100")10 $-16+2=-14$],
  [ #emph("1")1000=-8], [#emph("0")1000 $=8$], 
  [ #emph("1111")1=-1], [#emph("0000")1 $=1$],
  [ 10000=-16], [10000=16],
)
+ Practice Problem 4: 补位都是0，只有最后一位同时是1，参考 2.28 和 2.23 #block[
```c
int rightmost_one(unsigned x) {
  if (x == 0) {
    return 0;
  }
  return (-x) & x;
}
```
]
+ Practice Problem 5: 由于都是小类型转化为大类型，然后truncate回小类型，可以预见都是正确的结果 #table(
  columns: 4,
  [label], [type], [numeric value], [cast to $"TMin"_32$?],
  [A], [int], $-2147483648$, [yes],
  [B], [int], $-2147483648$, [yes],
  [C], [long long], $2147483648$, [yes],
  [D], [unsigned], $2147483648$, [yes],
  [E], [long long], $-2147483648$, [yes],
  [F], [unsigned], $2147483648$, [yes],
)
+ Practice Problem 6: #block[
  if k > 0, then Given $-x = tilde x + 1$, we assume $x = [x_(w-1), x_(w-2), ..., x_(k+1), 1, 0, ..., 0]$
  $
    -x &= tilde [x_(w-1), x_(w-2), ..., x_(k+1), 1, 0, ..., 0] + 1 \
       &= [tilde x_(w-1), tilde x_(w-2), ..., tilde x_(k+1), 0, 1, ..., 1] + 1 \
       &= [tilde x_(w-1), tilde x_(w-2), ..., tilde x_(k+1), 1, 0, ..., 0]
  $
  That's what the are doing

  for k = 0, then

  $
    -x &= tilde [x_(w-1), x_(w-2), ..., x_1, 1] + 1 \
       &= [tilde x_(w-1), tilde x_(w-2), ..., tilde x_1, 0] + 1 \
       &= [tilde x_(w-1), tilde x_(w-2), ..., tilde x_1, 1]
  $
  Same as what Complement Upper Bits doing.
]

== 2.34
#table(
  columns: 5,
  [Mode], $x$, $y$, $x dot y$, [Truncated $x dot y$],
  [Unsigned], $4=100$, $5=101$, $20=010100$, $4=100$,
  [Tow's complement], $-4=100$, $-3=101$, $12=001100$, $4=100$,
  table.cell(colspan: 5)[],

  [Unsigned],         $2=010$, $7=111$, $14=001110$, $6=110$,
  [Tow's complement], $2=010$, $-1=111$, $-2=111110$, $4=110$,
  table.cell(colspan: 5)[],

  [Unsigned],         $6=110$, $6=110$, $36=100100$, $4=100$,
  [Tow's complement], $-2=110$, $-2=110$, $4=000100$, $4=100$,
  table.cell(colspan: 5)[],
)

== 2.35
+ #[ 
Given $x(x eq.not 0), y, p = x *_w^t y, q = p / x$, 

$ 
p = x *_w^t y &= "U2T"_w ((x dot y) mod 2^w)  \
"T2U"(p)  &= (x dot y) mod 2 ^w \
(p + p_w * 2^w) + k * 2^w &= (x dot y) \ 
p + (k+p_w) * 2^w &= x dot y \
p + t * 2^w &= x dot y
$
]
+ #[

$
q &= p / x \
p &= q dot x + q mod x \
r &= q mod x
$

then $abs(r) < abs(x)$
]
+ #[
Given above two, we got

$
p = q dot x + r &= x dot y - t 2^w "where" abs(r) < abs(x) \
    r + t 2^w &= x dot (y - q) "where" abs(r) < abs(x) 
$

- If $y = q$, then $r +t 2^w = 0$, where $abs(r) < abs(x) < 2^(w-1)$, we can see $
r &&= -t 2^w & \
abs(r) &&= abs(t) 2^w &< 2 ^ (w-1) \
&& abs(t) &= 0
$ So $r = t = 0$
- If $r = t = 0$, then $
0 &= x dot (y - q) \
0 &= y -q "Since" x != 0\
y &= q
$
]

== 2.36
`int64_t` can fill all result values of multiplication of `int32_t`
```c
int tmult_ok(int x, int y) {
  int64_t r = int64_t(x) * int64_t(y);
  return r >= (1<<31) && r < (1<<31 - 1);
}
```

== 2.37
#[
#set enum(numbering: "A.")
+ No, `uint64_t` still truncated into `int32_t`, the loop will still access memory out of boundary.
+ change line 9 to #[

#codly(offset: 8)
```c
int ele_len = ele_cnt * el_size
if (!ele_cnt || ele_len / ele_cnt == y) {
  return NULL;
}
```
]
]

== 2.38
we can get 7 unique combination result.
#table(
  columns: 3,
  [k], [a], [result],
  $0$, $0$, $a$,
  $0$, $a$, $2a$,
  $1$, $0$, $2a$,
  $1$, $a$, $3a$,
  $2$, $0$, $4a$,
  $2$, $a$, $5a$,
  $3$, $0$, $8a$,
  $3$, $a$, $9a$,
)

== 2.39
$
B &= x << (n+1) - x << m \
  &= x << (w-1+1) - x << m \
  &= x << w - x << m \
  &= - x << m
$

== 2.40
#table(
  columns: 4,
  $K$, [Shifts], [Add/Subs], [Expression],
  $7$, $1$, $1$, $7=2^2+2^1+2^0=2^3-2^0=x << 3 - x$,
  $30$, $4$, $3$, $30=2^5-2^1=x << 5 - x << 1$,
  $28$, $2$, $1$, $28=2^5-2^2=x<<5 - x <<2$,
  $55$, $2$, $2$, $55=2^5+2^4+2^2+2^1+2^0=2^6-2^0-2^3=x << 6 - x << 3 - x$,
)

== 2.41
Suppose cost of Shifts is $c_"shift"$, cost of Add/Subs is $c_"add_sub"$, just compare:
- $A=(n-m+1) c_"shift"  + (n-m)c_"add_sub"  $
- $B=2 c_"shift" + c_"add_sub"$

If $n=m$, then choose A, else choose B.

== 2.42
```c
int div16(int x) {
  int delta = 1 << 4 - 1;
  return (x + delta & (x >> 31)) >> 4
  // Is || a if operation?
  // return (x + (x > 0 || delta + 1) - 1) >> 4
}
```
== 2.43
$
"result" &= x + y \
         &= x << 5- x + (y < 0 ? y + 7 : y) >> 3 \
         &= x dot 2^5 - x dot 2^ 0 + (y < 0 ? y + 2^3 - 1 : y) / 2^3 \
         &= x * (2^5-2^0) + y / 2^3
$

$M = 31$ and $N = 8$.

== 2.44
#[
#set enum(numbering: "A.")
+ `(x > 0) || (x - 1 < 0)` is false if `x=-1<<31`
+ `(x & 7) != 7 || (x << 29 < 0)` if true
+ `(x * x) >= 0` is false if `x = 46340`，只要找到一个数n，满足 $2^31 <= (n^2 mod 2^32) < 2^32 $，直接计算 $sqrt(2^31) + 1$
+ `x < 0 || -x <= 0` is true
+ `x > 0 || -x >= 0` is false for $x = "TMin" arrow.double -x = "TMin"$
+ `x + y = ux + uy` is true, since bit-wise for addition is same for both signed and unsigned
+ `x * ~y + uy * ux = -x` is true since $
x *_w^t ~y + x_u *_w^u y_u &= x dot (-y - 1) + x_u dot y_u \
                       &= - x y - x + x_u y_u \
                       &= -x
$
]

== 2.45
#table(
  columns: 3,
  [Fractional value], [Binary representation], [Decimal representation],
  $1/8$, $0.001$, $0.125$, 
  $3/4$, $0.110$, $0.75$, 
  $5/16$, $0.0101$, $#(5/16)$, 
  $2^1+2^(-1)+2^(-3)+2^(-4)=2+1/2+1/8+1/16=2 11/16$, $10.1011$, $#(11/16+2)$, 
  $2^0+2^(-3)=1+1/8=1 1/8$, $1.001$, $1.125$, 
  $2^2+2^0+2^(-1)+2^(-2)+2^(-3)=5 7/8$, $101.111$, $#(5+7/8)$, 
  $2^1+2^0+2^(-3)+2^(-4)=3 3/16$, $11.0011$, $#(3+3/16)$, 
)

== 2.46
$
0.1_10 &= 0.0 dot(0) dot(0) dot(1) dot(1)_2 \
    &= 1/2 dot 0. dot(0) dot(0) dot(1) dot(1)_2 \
    &= 1/2 dot 3 dot sum_(k=1)^infinity (1/16)^k = 3/2 dot (1/(1-1/16)-1) = 3/2 dot 1/15 = 1/10\
$

#[
#set enum(numbering: "A.")
+ $0.1-x=1 - 0.00011001100110011001100_2= 0.000000000000000000000 dot(0) dot(0) dot(1) dot(1)_2$
+ $0.000000000000000000000 dot(0) dot(0) dot(1) dot(1)_2 approx =2^(-20) * 0.1 approx #(calc.pow(2, -20) * 0.1)$
+ $100*60*60*10 dot 0.1 dot 2^(-20) approx #(100*60*60 * calc.pow(2, -20))$
+ $epsilon * 2000 approx #(100*60*60 * calc.pow(2, -20) * 2000)$ meters
]

== 2.47
#table(
  columns: 9,
  [Bits], $e$, $E$, $2^E$, $f$, $M$, $2^E times M$, $V$, [Decimal],
  [0 00 00], $0$, $0+1-1=0$, $1$, $0/4$, $0/4$, $0/4$, $0/4$, $0.0$,
  [0 00 01], $0$, $0+1-1=0$, $1$, $1/4$, $1/4$, $1/4$, $1/4$, $0.25$,
  [0 00 10], $0$, $0+1-1=0$, $1$, $2/4$, $2/4$, $2/4$, $1/2$, $0.5$,
  [0 00 11], $0$, $0+1-1=0$, $1$, $3/4$, $3/4$, $3/4$, $3/4$, $0.75$,

  [0 01 00], $1$, $1-1=0$, $1$, $0/4$, $4/4$, $4/4$, $4/4$, $1$,
  [0 01 01], $1$, $1-1=0$, $1$, $1/4$, $5/4$, $5/4$, $5/4$, $1.25$,
  [0 01 10], $1$, $1-1=0$, $1$, $2/4$, $6/4$, $6/4$, $3/2$, $1.5$,
  [0 01 11], $1$, $1-1=0$, $1$, $3/4$, $7/4$, $7/4$, $7/4$, $1.75$,

  [0 10 00], $2$, $2-1=1$, $2$, $0/4$, $4/4$, $8/4$, $2$, $2$,
  [0 10 01], $2$, $2-1=1$, $2$, $1/4$, $5/4$, $5/2$, $5/2$, $2.5$,
  [0 10 10], $2$, $2-1=1$, $2$, $2/4$, $6/4$, $6/2$, $3$, $3$,
  [0 10 11], $2$, $2-1=1$, $2$, $3/4$, $7/4$, $7/2$, $7/2$, $3.5$,

  [0 11 00], [-], [-], [-], [-], [-], [-], $infinity$, [-],
  [0 11 01], [-], [-], [-], [-], [-], [-], $"NAN"$, [-],
  [0 11 10], [-], [-], [-], [-], [-], [-], $"NAN"$, [-],
  [0 11 11], [-], [-], [-], [-], [-], [-], $"NAN"$, [-],
)

== 2.48
$
3510593_10 &= 00359141_16 = 00000000001101011001000101000001_2 \
           &= 1.101011001000101000001_2 times 2^21 \
           &= 2^(21+128-1) times 1.101011001000101000001_2 \
           &= 2^148 times 1.101011001000101000001_2 \
           &= 2^(10010100_2) times 1.101011001000101000001_2
$

So we fill rest part as 0, give us:

$
00000000001 & 101011001000101000001_2 = "0x00359141" \
            &"*********************" \
  010010100 & 10101100100010100000100_2 = "0x4a564504"
$

== 2.49
#[
#set enum(numbering: "A.")
+ from above example, this number requeire n+1 bit $ 1  underbrace(0 0 ... 0, n)  1 = 2^(n+1)+1 $
+ $2^24+1 = #(1+calc.pow(2, 24))$
]

== 2.50
#[
#set enum(numbering: "A.")
+ $10.111_2 (2 7/8)= 11.0_2 (3)$
+ $11.010_2 (3 1/4)= 10.0_2 (3)$
+ $11.000_2 (3)= 10.0_2 (3)$
+ $10.110_2 (2 3/4)= 11.0_2 (3)$
]

== 2.51
Stupid question, answer to first question just in the body, don't bother to ask. don't know that the question is about.
#[
#set enum(numbering: "A.")
+ $x^' = 0.00011001100110011001100_2 = 2^(-23) times ((2^2+2^3) + (2^6+2^7)+(2^10+2^11)+(2^14+2^15)+(2^18+2^19)) = 838860 / 2^23 = 0.09999990463256836 $
]

== 2.52
#[
#let hcell(body) = table.cell(colspan: 2, align: center + bottom, body)

#table(
  columns: 4,
  hcell[Format A], hcell[Format B],
  [Bits], [Value], [Bits], [Value],
  [011 0000], [1], [0111 000], [1],
  [101 1110], $ 2^(5-3) *(1 7/8) = 15 / 2 $, [1001 111], $ 2^(9-7) times (1 7/8) = 15 / 2 $,
  [110 1111], $ 2^(6-3) *(1 15/16) = 31 / 2 $, [0010 111], $ 2^(10-7) times 15/8 = 15 $,
  [000 0001], $ 2^(1-3) *(1/16) = 1 / 2^6 $, [0001 000], $ 2^(1-7) times 1 = 1 / 2^6 $,
) 
]

== 2.53
```c
// let it overflow
#def POS_INFINITY=1.8E309 
#def NEG_INFINITY=(-POS_INFINITY)
#def NEG_ZERO= 1.0 / NEG_INFINITY
```

== 2.54
#[
#set enum(numbering: "A.")

+ `x == (int)(double) x` is true
+ `x == (int)(float)x` is false, for any value with length of valid bits in bit-wise represention is greater than 23.
+ `d == (double)(float) d` is false, any _d_ satisfy $d > "TMAX"_"float"$
+ `f == (float)(double) f` is true
+ `f == -(-f)` is true
+ `1.0 / 2 == 1 / 2.0` is #strike(stroke: red)[false, since `/2` is bit shift, and `1/2.0` is double math].
+ `d * d >= 0.l0` is true
+ `f+d-f==d` is false, since addition will use round in result

]

= Chapter 3
== 3.1
#table(
  columns: 2,
  [Operand], [Value],
  `%rax`, [0x100],
  `0x104`, [0xAB],
  `$0x108`, [0x108],
  `(%rax)`, [0xFF],
  `4(%rax)`, [0xAB],
  `9(%rax, %rdx)`, [0x11],
  `260(%rcx, %rdx)`, $260+"0x1"+"0x3"="0x108" => "0x13"$,
  `0xFC(, %rcx, 4)`, $"0xFC" + "0x1" * 4 = "0x100" => "0xFF" $,
  `(%rax, %rdx, 4)`, $"0x100" + "0x3" * 4 = "0x10C" => "0x11" $,
)

== 3.2
#[
#set enum(numbering: "A.")

mem 中的量可以是任意宽度，需要由另一个量确定
+ `move`#text(fill: red)[l] `%eax, (%rsp)`: `%eax` 是32bit
+ `move`#text(fill: red)[w] `(%rax), %dx`
+ `move`#text(fill: red)[b] `$0xFF, %bl`
+ `move`#text(fill: red)[b] `(%rsp,%rdx,4), %dl`
+ `move`#text(fill: red)[q] `(%rdx), %rax`
+ `move`#text(fill: red)[w] `%dx, (%rax)`: `%dx` 是 16 bit
]

== 3.3
#[
#set enum(numbering: "A.")
+ `movb $0xF, (%ebx)`: `(%ebx)` 非法，地址只能是 64 bit 的register
+ `movl %rax, (%rsp)`: `%rax` 64 bit 需要 moveq
+ `movw (%rax),4(%rsp)`: 操作符两侧不能都是 mem (不知道宽度)
+ `movb %al,%sl`: 没有 `%sl`
+ `movq %rax,$0x123`: 无法写入常数量，后者不是寄存器也不是mem
+ `movl %eax,%rdx`: 32 bit -> 64 bit 要声明填充方式
+ `movb %si,8(%rbp)`: `%si` 宽度 16 bit，要用 `movw`
]

== 3.4
- 大变小: 依靠 register 取 truncate
- 小变大: 依靠 `movez##` or `moves##`

#table(
  columns: 3,
  [src_t], [dest_t], [Instruction],
  `long`, `long`, [ `movq (%rdi), %rax` \ `movq %rax, (%rsi)` ],
  `char`, `int`, [ `movsbl (%rdi), %eax` \ `movl %eax, (%rsi)` ],
  `char`, `unsigned`, [ `movsbl (%rdi), %eax` \ `movl %eax, (%rsi)` ],
  `unsigned char`, `long`, [ #strike[`movzbq (%rdi), %rax`] #text(fill: red)[`movezbl (%rdi), %eax`] \ `movq %rax, (%rsi)` ],
  `int`, `char`, [ `movl (%rdi), %eax` \ `movb %al, (%rsi)` ],
  `unsigned`, `unsigned char`, [ `movl (%rdi), %eax` \ `movb %al, (%rsi)` ],
  `char`, `short`, [ `movsbw (%rdi), %ax` \ `movw %ax, (%rsi)` ],
)

== 3.5
```asm
; %rdi=xp, %rsi = yp, %rdx = z
decode1:
  movq (%rdi), %r8 ; long tmp = *xp
  movq (%rsi), %rcx ; long tmp2 = *yp
  movq (%rdx), %rax ; long rtn = *z
  movq %r8, (%rsi) ; *yp = tmp 
  movq %rcx, (%rdx) ; *z = tmp2 
  movq %rax, (%rdi) ; *xp = rtn
  ret
```

so the c code should be:

```c
void decode1(long *xp, long *yp, long *zp) {
  // x -> y, y -> z, z -> x
  long tmp1 = *xp; long tmp2 = *yp; long tmp3 = *zp;

  *yp = tmp1; *zp = tmp2; *xp = tmp3;
}
```

== 3.6
#table(
  columns: 2,
  [Instruction], [Result],
  `leaq 9(%rdx),%rax`, $q+9$,
  `leaq (%rdx,%rbx),%rax`, $q+p$,
  `leaq (%rdx,%rbx,3),%rax`, $q+3p$,
  `leaq 2(%rbx,%rbx,7),%rax`, $p+2+7p=8p+2$,
  `leaq 0xE(,%rdx,3),%rax`, $14+3q$,
  `leaq 6(%bx,%rdx,7),%rax`, $6+p+7q$,
)

== 3.7
```asm
scale3:
  leaq (%rsi,%rsi,9),%rbx ; %rbx=y+9y=10y
  leaq (%rbx,%rdx),%rbx ; %rbx=10y+z
  leaq (%rbx,%rdi,%rsi),%rbx ; %rbx=10y+z+x*y
```

So the expression is `short t = 10*y+z+x*y`

== 3.8
#table(
  columns: 3,
  [Instruction], [Destination], [Value],
  `addq %rcx,(%rax)`, [0x100], $"0xff"+"0x1"="0x100"$,
  `subq %rdx,8(%rax)`, [0x108], $"0xab"-"0x3"="0xa8"$,
  `imulq $16,(%rax,%rdx,8)`, $"0x100" + "0x3" times 8 = "0x118"$, $"0x11" * 16=17 times 16 = 272 = "0x110"$,
  `incq 16(%rax)`, $"0x110"$, $"0x13" + 1 = "0x14"$,
  `decq %rcx`, `%rcx`, $"0x1" - 1 = "0x0"$,
  `subq %rdx,%rax`, `%rax`, $"0x100" - "0x3" = "0xfd"$,
)

== 3.9
```asm
shift_left4_rightn:
  movq %rdi,%rax ; %rax = x
  salq $4,%rax   ; x <<= 4
  movl %esi,%ecx ; Get n
  sarq %cl,%rax  ; x >>= n
  rtn
```

== 3.10
```asm
arith3:
  orq %rsi, %rdx  ; z = z | y
  sarq $9,%rdx    ; z = z >> 9
  notq %rdx       ; z = ~z
  movq %rdx, %rax ; rtn = z
  subq %rsi,%rbx  ; %rbx - y 
```

So we can construct out c code:

```c
short arith3(short x, short y, short z) {
  short p1 = z | y;
  short p2 = p1 >> 9; // 由于q是8byte，short是2byte，所以 sarq 和 shrq 效果一样
  short p3 = ~p2;
  short p4 = p3; // ??? wtf?
  return p4;
}
```

== 3.11
#[
#set enum(numbering: "A.")
+ it set `%rcx = 0`, just clear all its bits.
+ `movq $0, %rcx`
+ #text(fill: red)[xor use 3 bytes, while movq use 7 bytes], check https://stackoverflow.com/questions/4567903/how-to-tell-the-length-of-an-x86-instruction
]

== 3.12
```asm
; x in %rdi, y in %rsi, qp in %rdx, rp in %rcx
uremdiv:
  movq (%rdx), (%r8)
  movq (%rdi), (%rax)
  movq $0, (%rdx)
  divq %rsi
  movq %rax, (%r8)
  movq %rdx, (%rcx)
  ret
```

#text(fill: red)[the `movq $0, (%rdi)` should be `movl $0, (%edx)` since move 4 bit will fill upper 4 bit to zero]

== 3.13
#[
#set enum(numbering: "A.")
+ `a < b`: 32-bit int
+ `a >= b`: 16-bit short
+ `a <= b`: 8-bit unsigned char
+ `a != b`: 64-bit long / #text(fill: red)[unsigned long / pointer]
]

== 3.14
#[
#set enum(numbering: "A.")
+ `a > 0`: 64-bit long
+ `a == 0`: 16-bit short / unsigned short
+ `a > 0`: 8-bit unsigned char
+ `a <= 0`: 32-bit int
]

== 3.15
#[
#set enum(numbering: "A.")
+ `je <0x4003fc+0x02>` #sym.arrow `je 0x4003fe`
+ `je <0x400431+0xf4>` #sym.arrow `je 0x400425`
+ The code is below #[
```
400543: 77 02
400545: 5d
```
]
+ $400560$
]

== 3.16
#[
#set enum(numbering: "A.")

+ The C with goto version #[
```c
void godo_cond(short a, short *p) {
  if (!a) 
    goto skip;
  if (a <= *p) 
    goto skip;
  *p = a;

skip:
  return;
}
```
]
+ Because the C code condition has two logical with `&&`
]

== 3.17
#[
#set enum(numbering: "A.")

+ The C code #[
```c
long gotodiff_se(long x, long y) {
  long result; 
  if (x < y) {
    goto x_l_y;
  }
  ge_cnt++;
  result = x - y;
  return result;

x_l_y:
  lt_cnt++;
  return = y - x;
  return result;
}
```
]
+ #strike(stroke: red)[分支预测，编译器分析那边可能性越小，越偏好goto，以减少跳转] 为了便于写没有else条件的情况
]

== 3.18
```asm
; short test(short x, short y, short z)
; x in %rdi, y in %rsi, z in %rdx
test:
  leaq (%rdx, %rsi),%rax ; long %rax = z+y
  subq %rdi,%rax         ; %rax -= x
  cmpq %5,%rdx           ; z - 5
  jle  .L2               ; jump if z <= 5
  cmpq $2,%rsi           ; y - 2
  jle  .L3               ; jump if y <= 2
  movq %rdi,%rax         ; %rax = x
  idivq %rdx, %rax       ; %rax /= z
  ret

.L3:
  movq %rdi,%rax         ; %rax = x
  idivq %rsi,%rax        ; %rax /= y
  ret
.L2
  cmpq $3,%rdx           ; z - 3
  jge .L4                ; jump if z >= 3
  movq %rdx,%rax         ; %rax = z
  idivq %rsi,%rax        ; %rax /= y
.L4:
  rep; ret
```

So the C code should be
```c
short test(short x, short y, short z) {
  short val = z + y - x;
  if (z > 5) {
    if (y > 2) {
      val = x / z
    } else {
      val = x / y;
    }
  } else if (z < 3) {
    val = z / y;
  }

  return val;
}
```

== 3.19
#[
#set enum(numbering: "A.")
+ $2(45-25)=40$
+ $25+40=65$
]

== 3.20
```asm
; short arith(short x)
; x in %rdi
arith:
  leaq 15(%rdi),%rbx ; %rbx = 15 + x 
  testq %rdi,%rdi    ; x
  cmovns %rdi,%rbx   ; %rbx = x if x >= 0
  sarq $4,%rbx       ; %rbx >>_arithmetic 4
  ret
```

// (x >= 0 ? x : (15 + x)) >> 4
// (x >= 0 ? x : (15 + x)) / 16
// x >= 0 ? x / 16 : (15 + x) / 16
#[
#set enum(numbering: "A.")
+ #strike(stroke: red)[` >= 0 ? x / 16 : (15 + x)`] this is singned integer dividend.
+ See comment
]

== 3.21
```asm
; short test(short x, short y)
; x in %rdi, y in %rsi
test:
  leaq 12(%rsi),%rbx ; %rbx=12+y
  testq %rdi,%rdi    ; x
  jge .L2            ; jump if x >= 0
  movq %rdi,%rbx     ; %rbx=x
  imulq %rsi,%rbx    ; %rbx = x * y
  movq %rdi,%rdx     ; %rdx = x
  orq %rsi,%rdx      ; %rdx = x | y
  cmpq %rsi,%rdi     ; x - y
  cmovge %rdx,%rbx   ; %rbx = (x | y) if x >= y
  ret
.L2:
  idivq %rsi,%rdi ; x = x / y
  cmpq $10,%rsi   ; y - 10
  cmovge $rdi,%rbx ; %rbx = x / y if y >= 10
  ret
```

So the code should be:

```c
short test(short x, short y) {
  short val = 12 + y;
  if (x < 0) {
    if (x < y) {
      val = x * y;
    } else {
      val = x | y;
    }
  } else if (y >= 10) {
    val = x / y;
  }

  return val;
}
```

== 3.22
#[
#set enum(numbering: "A.")
+ $14! = 87178291200 > 2^31 - 1 = 2147483647$ yes, it will overflow
+ 64-bit won't overflow.
]

== 3.23
The C code in book is wrong.

```asm
; short dw_loop(short x)
; x initially in %rdi
dw_loop:
  movq %rdi, %rbx ; %rbx = x
  movq %rdi, %rcx ; %rcx = x
  icivq $9, %rcx  ; %rcx = x / 9 <- %rcx is y
  leaq (,%rdi,4),%rdx ; %rdx = 4 * x <- %rdx is n
.L2:
  leaq 5(%rbx,%rcx),%rcx ; y = 5 + x + y
  subq $1,%rdx ; n -= 1
  testq %rdx,%rdx ; n
  jq .L2 ; jump if n > 0
  rep; ret
```

#[
#set enum(numbering: "A.")
+ `y = %rcx, n = %rdx, x=%rcx`
+ yes, it use one single `leaq` do both operation
+ see comment
]

== 3.24
```asm
; short loop_while(short a, short b)
; a in %rdi, b in %rsi
loop_while:
  movl %0,%eax ; %eax = 0
  jmp .L2
.L3:
  leaq (,%rsi,%rdi),%rdx ; %rdx = b * a
  addq %rdx,%rax         ; %rax += %rdx
  subq $1,%rdi           ; a -= 1
.L2:
  cmpq %rsi,%rdi ; a - b
  jg .L3         ; jump if a > b
  rep; ret
```

So the c code should be:

```c
short loop_while(short a, short b) {
  short result = 0;
  while (a > b) {
    result += a * b;
    a -= 1;
  }
}
```

== 3.25
```asm
; a in %rdi, b in %rsi
loop_while2:
  testq %rsi,%rsi ; test b
  jle .L8         ; jump if b <= 0
  movq %rsi,%rax  ; %rax = b
.L7:
  imulq %rdi,%rax ; %rax *= a
  subq %rdi,%rsi  ; b -= a
  testq %rsi,%rsi ; test b
  jg .L7          ; jump if b > 0
  rep; ret
.L8:
  movq %rsi,%rax ; %rax = b
  ret
```

So the code should be:

```c
long loop_while2(long a, long b) {
  long result = b;
  while (b > 0) {
    result *= a;
    b -= a;
  }
  return result;
}
```

== 3.26
```asm
; short test_one(unsigned short x)
; x in %rdi
test_one:
  movl $1, %eax   ; %eax = 1
  jmp .L5         ; jump to L5
.L6:
  xorq %rdi,%rax  ; %rax ^= x
  shrq %rdi       ; x >>_L 1
.L5:
  testq %rdi,%rdi ; test x
  jne .L6         ; jump if x != 0
  addl $0,%eax    ; %eax &= 0
  ret
```

so the c code should be:

```c
short test_one(unsigned short x) {
  short val = 1;
  while (x != 0) {
    val ^= x;
    x >>= 1;
  }
  val &= 0;
  return val;
}
```

#[
#set enum(numbering: "A.")
+ use the first method to translate while loop
+ see above
+ #text(fill:red)[WTF? return 1 if there is odd number of 1 else return 0]
]

== 3.27
```c
long fibonacci(int n) {
  long a = 0;
  long b = 1;
  int i = 0;
  long tmp = b;

  while (i < n) {
    tmp = a + b;
    a = b;
    b = tmp;
    i += 1;
  }
  return b;
}

long fibonacci_guarded_do(int n) {
  long a = 0;
  long b = 1;
  int i = 0;
  long tmp = b;

  if (i >= n) 
    goto done;

loop:
  tmp = a + b;
  a = b;
  b = tmp;
  i += 1;
  if (i < n)
    goto loop;

done:
  return b;
}
```

== 3.28
```asm
; test fun_b(unsigned test x)
; x in %rdi
test_two:
  movl $1, %edx  ; %edx = 1
  movl $65,%eax  ; %eax = 65
.L10:
  movq %rdi,%rcx ; %rcx = x
  andl $1,%ecx   ; (x)_32 &= 1
  addq %rax,%rax ; 65 *= 2 (%rax)
  orq %rcx,%rax  ; 130 |= x (%rax)
  shrq %rdi      ; x >> 1
  addq $1,%rdx   ; 1 += 1 (%rdx)
  jne .L10        ; jump if %rdx != 0
  rep; ret
```

So the c code should be

```c
short test_two(unsigned short x) {
  short rax = 0;
  short rcx;
  for (int i = 1, rax = 65; i != 0; i += 1) {
    rcx = x;
    rcx &= 1;
    rax *= 2;
    rax |= rcx;

    x >>= 1;
  }

  return rax;
}
```

+ see code above
+ it's guarded-do style, but since `int i = 1` definintly won't cause fail, so that branch just be ignored.
+ ??? #text(fill: red)[The book has many error]

== 3.29
```c
long sum = 0;
long i;

i = 0;

while (i < 10) {
  if (i & 1)
    continue;

  sum += i;

  i++;
}
```

```c
long sum = 0;
long i;

i = 0;

loop:
while (i < 10) {
  if (i & 1)
    goto update;

  sum += i;

update:
  i++;
}
```

== 3.30
```asm
; void switch2(short x, short *dest)
; x in %rdi
switch2:
  addq $2, %rdi ; x += 2
  cmpq %8, %rdi ; x - 8
  ja .L2        ; jump if (unsigned) x > 8
  jmp *.L4(,%rid,8) ; jump to L4[x]

.L4:
  .quad .L9 ; 0
  .quad .L5
  .quad .L6
  .quad .L7
  .quad .L2
  .quad .L7
  .quad .L8
  .quad .L2
  .quad .L5
```

So the C code should be:

```c
void switch2(short x, short *dest) {
  short val = 0;
  switch (x) {
    case -2: .L9,
    case -1,6: .L5,
    case 0: .L6,
    case 1,3: .L7,
    case 2,5: .L2,
    case 4: .L8,
  }
}
```

== 3.31
```asm
; void switcher(long a, long b, long c, long* dest)
; a in %rdi, b in %rsi, c in %rdx, d in %rcx
switcher:
  cmpq $7,%rdi ; a - 7
  ja .L2       ; jump if (unsigned) a > 7
  jmp *.L4(%rdi,8) ; jump to L4[b]
  .section .rodata

.L7:
  xorq $15,%rsi  ; b ^= 15
  movq %rsi,%rdx ; c = b
.L3:
  leaq 112(%rdx),%rdi ; a = 112 + c
  jmp .L6
.L5:
  leaq (%rdx,%rsi),%rdi ; a = c + b
  salq $2,%rdi ; a <<= 2
  jmp .L6
.L2:
  movq %rsi,%rdi ; a = b
.L6:
  movq %rdi,(%rcx) ; *d = a
  ret

.L4:
  .quad .L3
  .quad .L2
  .quad .L5
  .quad .L2
  .quad .L6
  .quad .L7
  .quad .L2
  .quad .L5
```

So the code should be filled as:

```c
void switcher(long a, long b, long c, long *dest) {
  long val;
  switch (a) {
    case 5: // .L7
      c = b ^ 15;
    case 0: // .L3
      val = 112 + c
      break;

    // .L5
    case 2: 
    case 7: 
      val = (c + b) << 2;
      break;

    case 4: // .L6
      val = a;
      break;

    // .L2
    // case 1: case 3, case 6: // .L2
    default:
      val = b;
  }

  *dest = val;
}
```

== 3.32
#table(
  columns: 9,
  table.cell(colspan: 3, align: center + bottom, [Instruction]), table.cell(colspan: 5, align: center + bottom, [Instruction]), [],
  [Label], [PC], [Instruction], `%rdi`, `%rsi`, `%rax`, `%rsp`, `*%rsp`, [Description],
  [M1], `0x400560`, [callq], [10], $hyph$, $hyph$, `0x7ffffffe820`, $hyph$, [Call first(10)],
  [F1], `0x400548`, [lea], [10], $hyph$,$hyph$, `0x7ffffffe818`, `0x400565`, [Entry of first],
  [F2], `0x40054c`, [sub], [10], [11],$hyph$, `0x7ffffffe818`, `0x400565`, [],
  [F3], `0x400550`, [callq], [9], [11],$hyph$, `0x7ffffffe818`, `0x400565`, [call last(9, 11)],
  [L1], `0x400540`, [mov], [9], [11],$hyph$, `0x7ffffffe810`, `0x400555`, [Entry of last],
  [L2], `0x400543`, [imul], [9], [11],[9], `0x7ffffffe810`, `0x400555`, [],
  [L3], `0x400547`, [retq], [9], [11],[99], `0x7ffffffe810`, `0x400555`, [Return 99],
  [F4], `0x400555`, [retq], [9], [11],[99], `0x7ffffffe818`, `0x400565`, [Return 99],
  [M2], `0x400565`, [mov], [9], [11],[99], `0x7ffffffe820`, $hyph$, [Resume main],
)

== 3.33
```asm
procprob:
  movslq %edi,%rdi ; (signed extend) %edi -> %rdi
  addq %rdi,(%rdx) ; (%rdx) += ..(quad)
  addb %sil,(%rcx) ; (%rcx) += ..(byte)
  movl $6,%eax     ; (%eax) = 6
  ret              ; return 6
```

So the c code should be 

```c
int procprob(int a, short b, long* u, char* v) {
  // long* u, int a, char* v, char b
  *u += a;
  *v += b;
  return sizeof(a) + sizeof(b); // 4 + 2 ??
}

int procprob(int b, short a, long* v, char* u) {
  // long* v, int b, char* u, char a
  *u += a;
  *v += b;
  return sizeof(a) + sizeof(b); // 4 + 2 ??
}
```

== 3.34
```asm
; long P(long x)
; x in %rdi
P:
  pushq %r15
  pushq %r14
  pushq %r13
  pushq %r12
  pushq %rbp
  pushq %rbx
  subq $24, %rsp ; prepare stack with 8 * 3
  movq %rdi,%rbx ; %rbx = x
  movq 1(%rdi),%r15 ; %r15 = x+1
  movq 2(%rdi),%r14 ; %r14 = x+2
  movq 3(%rdi),%r13 ; %r13 = x+3
  movq 4(%rdi),%r12 ; %r12 = x+4
  movq 5(%rdi),%rbp ; %rbp = x+5
  movq 6(%rdi),%rax 
  movq %rax,(%rsp)  ; &stack = x + 6 <- save to stack
  leaq 7(%rdi),%rdx
  leaq %rdx,8(%rsp) ; &(stack+8) = x+7 <- save to stack
  movl $0,%eax ; clear %rax?
  call Q
```

#[
#set enum(numbering: "A.")
+ a0-a5 saved in callee-saved registers
+ a6=x+6 and a7=x+7 is saved in stack
+ #text(fill: red)[all six callee-saved register not enough for 8 value]
]

== 3.35
```asm
; long rfun(unsigned long x)
; x in %rdi
rfun:
  pushq %rbx
  movq %rdi,%rbx  ; move x to %rbx
  movl $0,%eax    ; %rax = 0
  testq %rdi,%rdi ; test x
  je .L2          ; jump if x == 0
  shrq $2,%rdi    ; x >>= 2
  call rfun       ; rfun(x>>2)
  addq %rbx,%rax  ; %rax += x
.L2
  popq %rbx
  ret
```

So c code should be

```c
long rfun(unsigned long x) {
  if (x == 0) {
    return 0;
  }
  unsigned long nx = x >> 2;
  long rv = rfun(nx);
  return rv + x;
}
```

== 3.36
#table(
  columns: 5,
  [Array], [Element size], [Total size], [Start address], [Element $i$],
  $ P $, $ 4 $, $ 20 $, $ x_P $, $ x_P + 4i $,
  $ Q $, $ 2 $, $ 4 $, $ x_Q $, $ x_Q + 2i $,
  $ R $, $ 8 $, $ 72 $, $ x_R $, $ x_R + 8i $,
  $ S $, $ 8 $, $ 80 $, $ x_S $, $ x_S + 8i $,
  $ T $, $ 8 $, $ 16 $, $ x_T $, $ x_T + 8i $,
)

== 3.37
#table(
  columns: 4,
  [Expression], [Type], [Value], [Assembly code],
  `P[1]`, [short], $M[x_p + 2]$, `movw 2(%rdx), %ax`,
  `P+3+i`, [short \*], $x_p + 6 + 2i$, `leaq 6(%rdx,%rcx,2), %rax`,
  `P[i*6 - 5]`, [short], $M[x_p + 12i - 10]$, text(fill:red)[`movw -10(%rdx,%rcx,12), %ax`], // wtf?
  `P[2]`, [short], $M[x_p + 4]$, `movw 4(%rdx), %ax`,
  `&P[i+2]`, [short \*], $x_p + 2i + 4$, `leaq 4(%rdx,%rcx,2), %rax`,
)

== 3.38
```asm
; long sum_element(long i, long j)
; i in %rdi, j in %rsi
sum_element:
  leaq 0(,%rdi,8),%rdx ; i*8 -> %rdx
  subq %rdi,%rdx       ; %rdx = 8 i - i
  addq %rsi,%rdx       ; %rdx = 7i + j
  leaq (%rsi,%rsi,4),%rax ; %rax = 5j
  addq %rax,%rdi       ; %rdi = 5j + i
  movq Q(,%rdi,8),%rax ; %rax = Q[8*(5j+i)]
  addq P(,%rdx,8),%rax ; %rax = P[8*(7i+j)]
  ret
```

So we can conclude

$
  8 (N times i + j) &= 8 times (7 i + j) & wide 8 (M times j + i) &=  8 times (5j + i) \
  N&=7 & wide M &= 5
$

== 3.39
- $"Aptr" = &A[i][0] = x_A + 4 * (16*i + 0) = x_A + 64i$
- $"Bptr" = &B[0][k] = x_B + 4 * (16*0 + k) = x_B + 4k$
- $"Bend" = &B[N][k] = x_B + 4 * (16*16 + k) = x_B + 4k + 1024$

== 3.40
```asm
; void fix_set_diag(fix_matrix A, int val)
; A in %rdi, val in %rsi
fix_set_diag:
  movl $0,%eax 
.L13:
  movl %esi,(%rdi,%rax) ; A[%rax] = val
  addq $68,%rax         ; %rax += 68 = 4 * 17
  compq $1088,%rax      ; %rax - 1088 = 68 * 16
  jne .L13              ; jump if %rax != 1088
  rep; ret
```

So the C code is below

```c
void fix_set_diag(fix_matrix A, int val) {
  int offset = 0;
  int row = (N + 1);
  int end = N * row;
  do {
    *(A[0][0]+offset) = val;
    offset += row;
  } while (offset != end)
}

```

== 3.41
#[
#set enum(numbering: "A.")
+ fields:
  - $p: 0$ 
  - $s.x: 8$ 
  - $s.y: 10$ 
  - $"next": 12$ 
]
+ total bytes = 20
+ The code is below: #[
```asm
; void st_init(struct test *st)
; st in %rdi
st_init:
  movl 8(%rdi),%eax  ; %eax=st->x 
  movl %eax,10(%rdi) ; st->y = st->x
  leaq 10(%rdi),%rax ; %rax = st->y
  movq %rax,(%rdi)   ; st->p = st->y
  movq %rdi,12(%rdi) ; st->next = st
  ret
```

```c
void st_init(struct test *st) {
  st->y = st->x;
  st->p = &(st->y);
  st->next= st;
}
```
]

== 3.42
```asm
; short test(struct ACE *ptr)
; ptr in %rdi
; ptr->v = 0
; ptr->p = 2
test:
  movl $1,%eax
  jmp .L2
.L3:
  imulq (%rdi),%rax ; %rax *= ptr->v
  movq 2(%rdi),%rdi ; ptr = ptr->p
.L2:
  testq %rdi,%rdi ; test ptr
  jne .L3         ; jump if ptr != 0
  rep; ret
```

So the c code is below, cumproduct all v of ACE:

```c
short test(struct ACE *ptr) {
  short val = 1;
  while (ptr != 0) {
    val *= ptr->v;
    ptr = ptr->p;
  }
  return val;
}
```

== 3.43
#table(
  columns: 4,
  [expr], [type], [Code], [Right Answer],
  `up->t1.u`, `long`, [ `movq (%rdi),%rax` \ `movq %rax,(%rsi)` ], [],
  `up->t1.v`, `short`, [ `leaq 8(%rdi),%rdx` \ `movw (%rdx),%ax` \ `movw %ax,(%rsi)` ], [ `movw 8(%rdi),%ax` \ `movw %ax,(%rsi)` ],
  `&up->t1.w`, `char *`, [ `leaq 10(%rdi),(%rsi)` ],[ `leaq 10(%rdi),%rax` \ `movq %rax,(%rsi)` ],
  `up->t2.a`, `int *`, [ `movq (%rdi),%rax` \ `movq %rax,(%rsi)` ], [`movq %rdi,(%rsi)`],
  `up->t2.a[up->t1.u]`, `int`, [ `movq (%rdi),%rax` \ `leaq (%rdi,%rax,4),%rax` \ `movl (%rax),%ecx` \ `movl %ecx,(%rsi)` ], [ `movq (%rdi),%rax` \ `movl (%rdi,%rax,4),%ecx` \ `movl %ecx,(%rsi)` ],
  `*up->t2.p`, `char`, [ `leaq 8(%rdi),%rdx` \ `movq (%rdx),%rcx` \ `movb (%rcx),%al` \ `movb %al,(%rsi)` ],[ `movq 8(%rdi),%rcx` \ `movb (%rcx),%al` \ `movb %al,(%rsi)` ],
)

== 3.44
#table(
  columns: 5,
  [Expr], [Sizes], [Size with alignment], [Alignment], [Total Size],
  `struct P1 { short i; int c; int* j; short* d; }`, $2, 4, 8, 8$, $2(0), [2], 4(4), 8(8), 8(16)$, [8], [24],
  `struct P2 { int i[2]; char c[8]; short s[4]; long *j; }`, $8, 8, 8, 8$, $8(0), 8(8), 8(16), 8(24)$, [8], [32],
  `struct P3 { long w[2]; int *c[2] }`, $16, 16$, $16, 16$, [8], [32],
  `struct P4 { char w[16]; char* c[2]; }`, $16, 16$, $16, 16$, [8], [32],
  `struct P5 { struct P4 a[2]; struct P1 t; }`, $32 times 2 = 64, 24$, $64(0), 24(64), [40]$, [8], [88],
)

== 3.45
#[
#set enum(numbering: "A.") 
+ size of field: $8, 4, 1, 2, 8, 8, 4, 8$, so with gap should be $8, 4, 1, [3], 2, [6], 8, 8, 4, [4], 8$
+ total size should be 56
+ rearange to $8, 4, 4, 8, 8, 8, 1, 2, [5]$
]

== 3.46
The problem seems remain $10$ on buffer, while the answer should be $16$ byte

#[
#set enum(numbering: "A.")
+ the buf left is length 10
+ So the stack is #table(
  columns: 8,
  [07], [06], [05], [04], [03], [02], [01], [00],  // %rsp+10+8
  [09], [08], [07], [06], [05], [04], [03], [00],  // saved %rbx
  [09], [08], [07], [06], [05], [04], [03], [02], 
  [09], [08], [07], [06], [05], [04], [03], [02], 
)
+ 0x0706050403020100
+ %rbx
+ No '0' character is append after string; use strlen over buf, while buf can have no 0 character.
]

== 3.47
#[
#set enum(numbering: "A.")
+ $"0xb754" - "0xd754" = 2^(12+2-1) = 8192$
+ $2^13 / 2^7 = 2^6 = 64$
]

== 3.48
#[
#set enum(numbering: "A.")
+ all address
  - for without protector: Return address: `%rsp + 40`, buf: `%rsp`, v: `%rsp+24` 
  - for protector version: Return address: `%rsp + 56`, buf: `%rsp+16`, v: `%rsp+8`, cannary value: `40+%rsp`
+ The protected version change positon of `buf` and `v`: So if buf overrun accross v, but not cannary value, the old version won't detect, while the protect can. The protected version is more sensitive.
]

== 3.49
#[
#set enum(numbering: "A.")
+ $-16="0xfffffffffffffff0"$, so the $s_2 = "%rsp" = (22 + 8n) amp (-16) = (22 + 8n) >> 4 << 4$. So the value $s_2$ must be a multiplor of 16 and be greater than $8n$(Because $22 > 16$). So $

s_2 := cases(
16 + 8n "if" n mod 2 = 0,
16 + 8(n-1) = 8n + 8 "if" n mod 2 != 0,
) 

$
+ The code calculate `%rcx` equal to $(s_2-7) >> 3 << 3$, then $s_2 = 8(floor.l s_2 / 8 floor.r - 1)$. Because of above equation, it will be enough to hold n inside allocated area.
+ #table(
  columns: 6,
  $n$, $s_1$, $s_2$, $p$, $e_1$, $e_2$,
  $5$, $2065$, $2065+16+32=2113$, $2104$, $#(2104-32-2065)$, $#(2113-2104)-8=1$,
  $6$, $2064$, $2064+16+48=#(2064+16+48)$, $2120$, $#(2120-40-2064)$, $#(2128-2120)-8=0$,
)
+ $s_2 - s_1$ must align to 16, while $p - s_1$ align to 8.
]

== 3.50
```asm
; double fcvt2(int *ip, float *fp, double *dp, long l)
; ip in %rdi, fp in %rsi, dp in %rdx, l in %rcx
; Result returned in %xmm0
fcvt2:
  movl (%rdi),%eax       ; %rax =(int) *ip
  vmovss (%rsi),%xmm0    ; %xmm0 =(float) *fp
  vcvttsd2si (%rdx),%r8d ; %r8d =(int) *dp
  movl %r8d,(%rdi)       ; *ip =(int) *dp;
  vcvtsi2ss %eax,%xmm1,%xmm1  ; %xmm1 =(float) *ip
  vmovss %xmm1,(%rsi)    ; *fp = (float) *ip
  vcvtsi2sdq %rcx,%xmm1,%xmm1 ; %xmm1 =(double) l
  vmovsd %xmm1,(%rdx)    ; *dp = (double) l
  vunpcklps %xmm0,%xmm0,%xmm0
  vcvtps2pd %xmm0,%xmm0
  ret
```

So the C could should be

```c
double fcvt2(int *ip, float *fp, double *dp, long l) {
  int i = *ip; float f = *fp; double d = *dp;
  *ip = (int) d;
  *fp = (float) i;
  *dp = (double) l;
  return (double) f;
}
```

== 3.51
#table(
  columns: 3,
  $T_x$, $T_y$, [Instruction(s)],
  `long`, `double`, `vcvtsi2sdq %rdi,%xmm0`,
  `double`, `int`, [`vcvttsd2si %xmm0,%eax`],
  `double`, `float`, [`vmovddup %xmm0,%xmm0` \ `vcvtpd2psx %xmm0,%xmm0`],
  `long`, `float`, [ `vcvtsi2ssq %rdi,%xmm0,%xmm0` ],
  `float`, `long`, [ `vcvttss2siq %xmm0,%rax` ],
)

== 3.52
#[
#set enum(numbering: "A.")
+ `double a @ %xmm0, long b @ %rdi, %float c @ %xmm1, int d @ %esi`
+ `int a @ edi, double *b @ %rsi, float *c @ rdx, long d @ %rcx`
+ `double *a @ %rdi, double b @ %xmm0, int c @ %esi, float d @ %xmm1`
+ `float a @ %xmm0, int *b @ %rdi, float c @ %xmm1, double d @ %xmm2`
]

== 3.53
```asm
; double funct1(arg1_t p, arg2_t q, arg3_t r, arg4_t s)
; float/double: float _ 
; other:        _ long
; double funct1(int p, float q, long r, double s)
; double funct1(int p, long q, float r, double s)
funct1:
  vcvtsi2ssq %rsi,%xmm2,%xmm2 ; convert second arg long to float
  vaddss %xmm0,%xmm2,%xmm0 ; float add over first float and second long
  vcvtsi2ss %edi,%xmm2,%xmm2 ; convert int to float
  vdivss %xmm0,%xmm2,%xmm0   ; float div over p / (q+r)
  vunpcklps %xmm0,%xmm9,%xmm0
  vcvtps2pd %xmm0,%xmm0      ; float to double
  vsubsd %xmm1,%xmm0,%xmm0   ; p/(q+r) - s
  ret
```

== 3.54
```asm
; double funct2(double w, int x, float y, long z)
; w in %xmm0, x in %edi, y in %xmm1, z in %rsi
funct2:
  vcvtsi2ss %edi,%xmm2,%xmm2 ; convert (int)x to float
  vmulss %xmm1,%xmm2,%xmm1   ; y *= x float
  vunpcklps %xmm1,%xmm1,%xmm1
  vcvtps2pd %xmm1,%xmm2      ; %xmm2 = %xmm1 = double (y)
  vcvtsi2sdq %rsi,%xmm1,%xmm1; convert y = (double)z
  vdivsd %xmm1,%xmm0,%xmm0   ; %xmm0 = (double) w / y
  vsubsd %xmm0,%xmm2,%xmm0   ; %xmm0 = double(y) - (double) w / y
  ret
```

So the C code should be:

```c
double funct2(double w, int x, float y, long z) {
  return double(y * (float) x) - w / (double) z;
}
```

== 3.55
$
&1077936128_10 space 0_10 \
&0 space underbrace(10000000100, 11) space 0000000000000000000000000000000000000000000000000000_2 \
&1 times 2^(1028-1023) times 2^0 \
&1 times 2^5 \
&32
$

== 3.56
#[
#set enum(numbering: "A.")
+ the double at `%xmm1`: $2147483647 space 4294967295 = "7fffffff" "ffffffff"_16$  So just: `abs(x)`
+ clear all bits of `%xmm0`, so `expr(x)` should be `0.0`
+ the double at `%xmm1` is $"80000000" "0000000"_16$, so just: `-x`
]

== 3.57
```asm
; double funct3(int *ap, double b, long c, float *dp)
; ap in %rdi, b in %xmm0, c in %rsi, d in %rdx
funct3; 
  vmovss (%rdx),%xmm1 ; float %xmm1 = *dp
  vcvtsi2sd (%rdi),%xmm2,%xmm2 ; double %xmm2 = *ap
  vucomisd %xmm2,%xmm0 ; double (b - *ap)
  jbe .L8 ; jump if b <= *ap
  vcvtsi2ssq %rsi,%xmm0,%xmm0 ; float %xmm0 = c
  vmulss %xmm1,%xmm0,%xmm1    ; float %xmm1 = (*dp) * c
  vunpcklps %xmm1,%xmm1,%xx1
  vcvtps2pd %xmm1,%xmm0    ; convert float to double
  ret
.L8:
  vaddss %xmm1,%xmm1,%xmm1 ; %xmm1 = (*dp) * 2
  vcvtsi2ssq %ris,%xmm0,%xmm0 ; float %xmm0 = c
  vaddss %xmm1,%xmm0,%xmm0 ; float %xmm0 += (*d) * 2
  vunpcklps %xmm0,%xmm0,%xx0
  vcvtps2pd %xmm0,%xmm0    ; convert float to double
  ret
```

So the C code should be

```c
double funct3(int *ap, double b, long c, float *dp) {
  float d = *dp; int a = *ap;
  if (b - double(a) <= 0.0) {
    return double(float(c) + (d * 2));
  } else {
    return double(d * float(c))
  }
}
```

== 3.58
```asm
; long decode2(long x, long y, long z)
; x in %rdi, y in %rsi, z in %rdx
decode2:
  subq %rdx,%rsi ; y -= z
  imulq %rsi,%rdi ; x *= y
  movq %rsi,%rax ; %rax = y
  salq $63,%rax ; %rax << 63
  sarq $63,%rax ; %rax >> 63
  xorq %rdi,%rax ; %rax ^= x
  ret
```

So the c code should be:

```c
long decode2(long x, long y, long z) {
  y -= z;
  x *= y;
  return ((y << 63) >> 63) ^ x;
}
```

== 3.59
```asm
; int128_t* dest in %rdi, int64_t x in %rsi, int64_t y in %rdx
store_prod:
  movq %rdx,%rax  ; %rax = y
  cqto            ; sign extend %rax -> %rdx:%rax
  movq %rsi,%rcx  ; %rcx = x_h
  sarq $63,%rcx   ; x_h >>= 63 -> SignExtend(x_63)
  imulq %rax,%rcx ; x_h *= y_t -> SignExtend(x_63) *= y
  imulq %rsi,%rdx ; y_h *= x_t -> SignExtend(y_63) *= x
  addq %rdx,%rcx  ; x_h += y_h -> %rcx = SignExtend(x_63) * y + SignExtend(y_63) * x
  mulq %rsi       ; x_t * y_t -> %rdx:%rax
  addq %rcx,%rdx  ; y_h += x_h
  movq %rax,(%rdi)  ; move lower 8 byte
  movq %rdx,8(%rdi) ; move upper 8 byte
  ret
```

when sign extend a 64-bit sign integer to 128-bit sign integer:

$ x_(128"bit") = "SignExtend"(x_(64"bit"))$

Given

$
x_(128"bit") &= -2^127 x_63 + sum_(i=0)^62 2^i x_i + sum_(i=63)^126 2^i x_63 \
             &= -2^127 x_63 + sum_(i=0)^63 2^i x_i + sum_(i=64)^126 2^i x_63 \
             &= -2^127 x_63 + x_64^u + sum_(i=64)^126 2^i x_63 "where" x_64^u "meas unsigned value of x" \
             &= 2^64 times x_63 times ( -2^63 + sum_(i=0)^62 2^i) + x_64^u  \
             &= - 2^64 times x_63 + x_64^u
$

Then

$
x_(128"bit") times y_(128"bit") &= (- 2^64 times x_63 + x_64^u) times (- 2^64 times y_63 + y_64^u) \
&= 2^128 times x_63 y_63 - 2^64 times x_63 * y^u_64 - 2^64 times y_63 * x^u_64 + x_64^u y_64^u \
&= 2^64 times (2^64 x_63 y_63 - x_63 * y^u_64 - y_63 * x^u_64) + x_64^u y_64^u \
$

The assembly `mulq %rsi` will give unsigned multiplication result as $x_64^u y_64^u$. So the question left is %rcx equal to the equation in the parens.

#table(
  columns: 4,
  $x_63$, $y_63$, $ 2^64 x_63 y_63 - x_63 * y^u_64 - y_63 * x^u_64 $, [`%rcx` = $ "SE"(x_63) y + "SE"(y_63) x$],
  $0$, $0$, $0$, $0$,
  $0$, $1$, $-x^u_64 = -x^t_64 "Since" x_63=0$, $-x$,
  $1$, $0$, $-y^u_64 = -y^t_64 "Since" y_63=0$, $-y$,
  $1$, $1$, $2^64-y^u_64-x^u_64 = 2^64 - (2^63 + y^u_63) - (2^63 + x^u_63)$, $ -y - x = -y^u_63 - x^u_63$,
)

Some Lemma:
$
x_64^t = -2^63 x_63 + sum_(i=0)^(62) 2^i x_i = -2^63 x_63 + x_63^u
$

== 3.60
```asm
; long loop(long x, int n)
; x in %rdi, n in %esi
loop:
  movl %esi,%ecx ; %ecx = n
  movl $1, %edx  ; %edx = 1
  movl $0, %eax  ; %rax = 0
  jmp .L2
.L3:
  movq %rdi,%r8 ; %r8 = x
  andq %rdx,%r8 ; %r8 = x & %rdx
  orq %r8,%rax  ; %rax |= x & %rdx
  sqlq %cl,%rdx ; %rdx << %cl
.L2:
  testq %rdx,%rdx
  jne .L3 ; jump if %rdx != 0
  rep;ret
```

So the C code should be

```c
long loop(long x, int n) {
  long result = 0;
  for (long rdx = 1; rdx != 0 ; rdx <<= n) {
    result |= x & rdx;
  }

  return result;
}
```

#[
#set enum(numbering: "A.")
+ `x -> %rdi`, `n -> %esi or %ecx`, `result -> %rax`, `mask -> %rdx`
+ see the code ...
]

== 3.61
```c
long cread_alt(long *xp) {
    if (xp != 0) {
        return *xp;
    } else {
        return 0;
    }
}
```

== 3.62
```asm
; p1 in %rdi, p2 in %rsi, action in %edx
.L8:
  movl $27,%eax ; %rax = 27
  ret
.L3:
  movq (%rsi),%rax ; result = *p2
  movq (%rdi),%rdx
  movq %rdx,(%rsi) ; *p2 = *p1
  ret
.L5:
  movq (%rdi),%rax ; result = *p1
  addq (%rsi),%rax ; result += *p2
  movq %rax,(%rdi) ; *p1 = result
  ret
.L6:
  movq $59,(%rdi) ; *p1 = 59
  movq (%rsi),%rax ; result = *p2
  ret
.L7:
  movq (%rsi),%rax ; result = *p2
  movq %rax,(%rdi) ; *p1 = result
  movl $27,%eax    ; result = 27
  ret
.L9:
  movl $12,%eax 
  ret
```

```c
long switch3(long *p1, long *p2, mode_t action) {
  long result = 0;
  switch(action) {
    case MODE_A:
      result = *p2;
      *p2 = *p1;
      berak;
    case MODE_B:
      result = *p1 + *p2;
      *p1 = result;
      break;
    case MODE_C:
      *p1 = 59
      result = *p2
      break;
    case MODE_D:
      result = *p2;
      *p1 = result;
      // can fall through
      // result = 27;
      // break;
    case MODE_E:
      result = 27;
      break;
    default:
      result = 12;
  }
  return result;
}

```

== 3.63
```asm
; long switch_prob(long x, long n)
; x in %rdi, n in %rsi
switch_prob:
  sub $0x3c,%rsi ; n -= 60
  cmp $0x5,%rsi  ; n - 5
  ja ..          ; jump if n > 5
  jmpq *0x4006f8(,%rsi,8) ; jump to switch

  lea 0x0(,%rdi,8),%rax ; result = x*8
  nop
  retq

  mov %rdi,%rax ; result = x
  sar $0x3,%rax ; result >>= 3
  retq

  mov %rdi,%rax ; result = x
  shl $0x4,%rax ; result <<= 4
  sub %rdi,%rax ; result -= x
  mov %rax,%rdi ; 
  imul %rdi,%rdi; %rdi = result * result
  lea 0x4b(%rdi),%rax ; result = 0x4b + result * result
  retq

```

So the C code should be 

```c
long switch_prob(long x, long n) {
  long result = x;
  switch (n) {
    case 60:
    case 62:
      // 4005a1
      result *= 8;
      break;
    case 63:
      // 4005aa
      result >>= 3;
      break;

    case 64:
      // 4005b2
      result <<= 4;
      result -= x;
    case 65:
      // 4005bf
      result *= result;

    // case 61:
    default:
      // 4005c3
      result += 0x4b;
  }
  return result;
}
```

== 3.64
#[
#set enum(numbering: "A.")
+ $amp A[i][j][k] = x_A + L(T dot (S dot i + j) +k)$
+ So the assembly code should be #[
```asm
; long store_ele(long i, long j, long k, long *dest)
; i in %rdi, j in %rsi, k in %rdx, dest in %rcx
store_ele:
  leaq (%rsi,%rsi,2),%rax ; %rax = 3*j
  leaq (%rsi,%rax,4),%rax ; %rax = j + 12j = 13j
  movq %rdi,%rsi          ; %rsi = i
  salq %6,%rsi            ; %ris = i * 2^6 = 64i
  addq %rsi,%rdi          ; %rdi = 65i
  addq %rax,%rdi          ; %rdi = 65i + 13j
  addq %rdi,%rdx          ; %rdx = k + 65i + 13j
  movq A(,%rdx,8),%rax    ; %rax = A[8 * %rdx]
  movq %rax,(%rcx)
  movl $3640,%eax
```
]So we got $amp A[i][j][k] = x_A + 8(13 dot (5 dot i + j) +k)$, then $T = 13, S= 5$. Because $T S R times 8 = 3640$, then $R = 7$
]

== 3.65
```asm
.L6:
  movq (%rdx),%rcx 
  movq (%rax),%rsi 
  movq %rsi,(%rdx)
  movq %rci,(%rax) ; above exchanged (%rdx) <-> (%rax)
  addq $8,%rdx     ; %rdx += 8
  addq $120,%rax   ; %rax j+= 120   120 = 8*15
  cmpq %rdi,%rax
  jne .L6          ; jump if %rax != %rdi
```

#[
#set enum(numbering: "A.")
+ `%rdx` Because it increase by 1
+ `%rax`
+ $M = 15$
]

== 3.66
```asm
; long sum_col(long n, long A[NR(n)][NC(n)], long j)
; n in %rdi, A in %rsi, j in %rdx
sum_col:
  leaq 1(,%rdi,4),%r8 ; %r8 = 1+4n
  leaq (%rdi,%rdi,2),%rax ; %rax = 3 %rdi = 3 n
  movq %rax,%rdi      ; %rdi = 3n
  testq %rax,%rax
  jle .L4             ; jump if n <= 0
  salq %3,%r8         ; %r8 = 8(1+4n)
  leaq (%rsi,%rdx,8),%rcx ; %rcx = A + 8 j
  movl $0,%eax
  movl $0,%edx
.L3:
  addq (%rcx),%rax   ; cumsum A[i][j]
  addq $1,%rdx       ; index += 1
  addq %r8,%rcx      ; %rcx = A + 8j += 8(1+4n)
  cmpq %rdi,%rdx     ; index - 3n
  jne .L3            ; jump if index != n
  rep;ret
.L4:
  movl $0,%eax ; return 0
  ret
```

So $"NR(n)" = 3n$, and $8(1+4n) = "NC(n)" times 8 arrow.double "NC(n)" = 1+4n $

== 3.67
```asm
; x in %rdi, y in %rsi, z in %rdx
eval:
  subq $104,%rsp
  movq %rdx,24(%rsp) ; z -> 24+S
  leaq 24(%rsp),%rax ; %rax <- &z
  movq %rdi,(%rsp)   ; x -> S
  movq %rsi,8(%rsp)  ; y -> 8 + S
  movq %rax,16(%rsp) ; &z -> 16 + S
  leaq 64(%rsp),%rdi ; %rdi <- 64+S
  call process

; s in %rdi
process:
  movq %rdi,%rax     ; return %rax = 64+S
  movq 24(%rsp),%rdx ; %rdx <- (24+S) <- &z
  movq (%rdx),%rdx   ; %rdx = z
  movq 16(%rsp),%rcx ; 
  movq %rcx,(%rdi)   ; (S+64) <- (16+S) <- y
  movq 8(%rsp),%rcx
  movq %rcx,8(%rdi)  ; (S+64+8) <- (8+S) <- x
  movq %rdx,16(%rdi) ; (s+64+16) <- z
  ret
```

#text(fill: red)[Note: the write in stack is in reverse order, so we write 8 bit value x to `S+24`, then the `(S+32)` should be value of `z`, not `(S+24)` ]

#[
#set enum(numbering: "A.")
+ the diagram is #table(
  columns: 1,
  `%rsp+104`,
  [`%rsp+24` #sym.arrow.double `z`],
  [`%rsp+16` #sym.arrow.double `&z`],
  [`%rsp+8` #sym.arrow.double `y`],
  [`%rsp` #sym.arrow.double `x`],
)
+ It pass over `%rdi` which is `%rsp + 64`
+ just access through Stack with offset
+ Set in Stack offset from [64-80]
+ the diagram should be #table(
  columns: 1,
  `%rsp+104`,
  text(fill: red)[`%rsp+80` #sym.arrow.double `z`],
  [`%rsp+72` #sym.arrow.double `x`],
  [`%rsp+64` #sym.arrow.double `y`],
  [`%rsp+24` #sym.arrow.double `z`],
  [`%rsp+16` #sym.arrow.double `&z`],
  [`%rsp+8` #sym.arrow.double `y`],
  [`%rsp` #sym.arrow.double `x`],
)
+ function don't know how structure store field offset, the compiler hard coded all offset.
]

== 3.68
```asm
; void setVal(str1 *p, str2 *q)
; p in%rdi, q in %rsi
setVal:
  movslq 8(%rsi),%rax ; long(t)
  addq %rax,184(%rdi)
  ret
```

So t at `%rsi+8`, and u at `%rsi 32`, the struct size of str2 should be: $[B], [4], [2 A], [8]$. And size of str1 should be $[4 A B] [8]$, 
consider alignment, we have
$
4 A B + delta = 184 wide B + delta = 8 wide 8 + 2A + delta = 32 \
$

Try out we found $B = 5 wide A = 9$ so with alignment 

$
5, [3], 4, 18, [6], 8 \
180, [4], 8
$

== 3.69
```asm
; void test(long i, b_struct *bp)
; i in %rdi, bp in %rsi
<test>:
  mov 0x120(%rsi),%ecx ; (bp+288) -> %ecx
  add (%rsi),%ecx      ; %ecx += bp->first, %rcx is n
  lea (%rdi,%rdi,4),%rax ; %rax <- 5i
  lea (%rsi,%rax,8),%rax ; %rax <- bp + 40i
  mov 0x8(%rax),%rdx     ; %rdx <- (bp+40i+8) <- &ap->idx
  movslq %ecx,%rcx       ; %rcx = (long)(n)
  mov %rcx,0x10(%rax,%rdx,8) ; (bp+40i+16+8*(idx)) <- n
  ret
```

the offset for idx and x should be:

$
"idx" &= "bp"+8+40i \
"x" &= "bp"+8+40i + 8 + 8*"idx"
$

So the struct for a_struct should be

```c
typedef a_struct {
  long idx;
  long x[4];
}
```

and CNT should be 7, so $8 + 40 *7 = 288$

== 3.70
#[
#set enum(numbering: "A.")
+ fields offset: 
  - `e1.p` = 0
  - `e1.y` = 8
  - `e2.x` = 0
  - `e2.next` = 8
+ total bytes should be 16
+ the assembly code #[
```asm
; void proc (union ele *up)
; up in %rdi
proc:
  movq 8(%rdi),%rax ; (up+8) -> %rax
  movq (%rax),%rdx  ; *(up+8) -> (up->e2.ele) -> %rdx
  movq (%rdx),%rdx  ; *((up->e2.ele)->e1.p)
  subq 8(%rax),%rdx ; *((up->e2.ele)->e1.p) - up->e1.y
  movq %rdx,(%rdi)  ; up->e2.x = %rdx
  ret
```
] so expression should be `up->e2.x = *((up->e2.next)->e1.p) - up->e1.y`
]

== 3.71
```c
#include <stdio.h>

#define BSIZE 32

void good_echo() {
    char buf[BSIZE];
    while (fgets(buf, BSIZE, stdin)) {
        if (fputs(buf, stdout) < 0) {
            break;
        }
    }
}
```

== 3.72
#[
#set enum(numbering: "A.")
+ $s_2 =s_1 - (30 + 8n) / 16 * 16$
+ $p = (s_2 + 15) / 16 * 16$
+ $p_e = p + n * 8$ so $
e_1 &= s_1 - p_e \
    &= s_1 - (s_2 + 15) / 16 * 16 - n * 8 \ 
    &= s_1 - (s_1 - (30 + 8n) / 16 * 16 + 15) / 16 * 16 - n * 8 \ 
$simulation tells the result range in [1, 24]
+ distance to $s_1$ must align to 16, and enough for $8n$ bits
]

== 3.73
```c
range_t find_range(float x) {
    int result;
    unsigned char cf, zf, pf;
    float y = 0.0;

    asm(
            "vucomiss %[x],%[y]\n\t"
            "setc %[cf]\n\t"
            "setz %[zf]\n\t"
            "setp %[pf]\n\t"
            : [cf] "=r" (cf), [zf] "=r" (zf), [pf] "=r" (pf)
            : [x] "x" (x), [y] "x" (y)
    );
    printf("%f: %d %d %d\n", x, cf, zf, pf);

    if (pf) {
        return OTHER;
    } else if (cf) {
        return POS;
    } else if (zf) {
        return ZERO;
    } else {
        return NEG;
    }
}
```

== 3.74
```c
range_t find_range_move(float x) {
    int result;
    char cf = 0, zf = 0, pf = 0;
    float y = 0.0;
    short one = 1;

    asm(
            "vucomiss %[x],%[y]\n\t"
            // "cmovpw %[one],%[pf]\n\t"
            // "cmovcw %[one],%[cf]\n\t"
            // "cmovzw %[one],%[zf]\n\t"
            "setc %[cf]\n\t"
            "setz %[zf]\n\t"
            "setp %[pf]\n\t"
            // "cmovp %[cf],%[zf]\n\t"
            : [cf] "=r" (cf), [zf] "=r" (zf), [pf] "=r" (pf)
            : [x] "x" (x), [y] "x" (y), [one] "r" (one)
    );
    printf("%f: %d %d %d\n", x, cf, zf, pf);

    zf = cf | zf;
    return zf + cf + pf;

}
```

== 3.75
#[
#set enum(numbering: "A.")
+ `%xmm0` store the real part, `%xmm1` store the imag part. Any more will continue this way.
+ return in the same way.
]

== ASM:EASM
=== Practice Problem 1
```c
int tmult_ok4(long x, long y, long *dest) {
    unsigned char bresult;
    asm(
            "imulq %[x],%[y]\n\t"
            "movq %[y],%[p]\n\t"
            "setae %[b]"
            : [p] "=m" (*dest), [b] "=r" (bresult)
            : [x] "r" (x), [y] "r" (y)
    );

    return (int) bresult;
}
```

=== Practice Problem 2
```c
void umult_full(unsigned long x, unsigned long y, unsigned long *dest) {
    unsigned long* r0 = dest;
    unsigned long* r1 = dest+1;
    asm(
            "movq %[x],%%rax\n\t"
            "mulq %[y]\n\t"
            "movq %%rax,%[r0]\n\t"
            "movq %%rdx,%[r1]"
            : [r0] "=m" (*r0), [r1] "=m" (*r1)
            : [x] "r" (x), [y] "r" (y)
            : "%rax","%rdx"
    );
}
```

=== Practice Problem 3
```c
void lock_incr(int *ptr) {
    asm(
            "lock\n\t"
            "addl $1,%[p]"
            : [p] "+m" (*ptr)
    );
}
```

=== Practice Problem 4
```c
int odd_parity(unsigned long x) {
    unsigned char bflag = 0;
    unsigned char bflag8;
    while (x) {
        bflag8 = 1;
        asm(
                "testq %[x],%[x]\n\t"
                "jp .L0\n\t"
                "movb $0,%[b8]\n\t"
                ".L0:\n\t"
                : [b8] "+r" (bflag8)
                : [x] "r" (x)
        );
        bflag ^= bflag8;
        x >>= 8;
    }

    return (int)bflag;
}
```

=== Practice Problem 5
```c
double dmin(double x, double y) {
    double result;
    asm(
            "vminsd %[x],%[y],%[r]\n\t"
            : [r] "=x" (result)
            : [x] "x" (x), [y] "x" (y) 
    );
    return result;
}
```

=== Practice Problem 6
```c
double dsqrt(double x) {
    double result;
    asm(
            "sqrtsd %[x],%[r]"
            : [r] "=x" (result)
            : [x] "x" (x)
    );
    return result;
}
```

== Lab::Bomb
Use pwndbg to save your life: https://github.com/pwndbg/pwndbg
=== phase4::func4
输入 `%rdi` 必须小于等于14，同时让该函数返回 0，发现会按照条件 (arg / 2) - arg < 0 进行循环，
参数: `%rdi` `%rdx` 均为输入参数
```
   0x0000000000400fce <+0>:     sub    rsp,0x8
   0x0000000000400fd2 <+4>:     mov    eax,edx ; eax = arg0
   0x0000000000400fd4 <+6>:     sub    eax,esi ; eax -= 0 => arg0
   0x0000000000400fd6 <+8>:     mov    ecx,eax ; ecx = arg0
   0x0000000000400fd8 <+10>:    shr    ecx,0x1f ; 0 >>= 31
=> 0x0000000000400fdb <+13>:    add    eax,ecx  ; eax += 0
   0x0000000000400fdd <+15>:    sar    eax,1    ; eax /= 2
   0x0000000000400fdf <+17>:    lea    ecx,[rax+rsi*1] ; ecx = arg0 / 2
   0x0000000000400fe2 <+20>:    cmp    ecx,edi  ; ecx - arg0
   0x0000000000400fe4 <+22>:    jle    0x400ff2 <func4+36>

   0x0000000000400fe6 <+24>:    lea    edx,[rcx-0x1]
   0x0000000000400fe9 <+27>:    call   0x400fce <func4>
   0x0000000000400fee <+32>:    add    eax,eax
   0x0000000000400ff0 <+34>:    jmp    0x401007 <func4+57>

   0x0000000000400ff2 <+36>:    mov    eax,0x0 ; eax = 0
   0x0000000000400ff7 <+41>:    cmp    ecx,edi ; ecx - edi => (arg0 / 2) - arg0
   0x0000000000400ff9 <+43>:    jge    0x401007 <func4+57>

   0x0000000000400ffb <+45>:    lea    esi,[rcx+0x1] ; esi => (arg0 / 2) + 1
   0x0000000000400ffe <+48>:    call   0x400fce <func4>
   0x0000000000401003 <+53>:    lea    eax,[rax+rax*1+0x1]

   0x0000000000401007 <+57>:    add    rsp,0x8
   0x000000000040100b <+61>:    ret


  rdi rsi rdx rcx
1 0xe 0   0xe 0 
2 0xe 0x8 0xe 0x7
1 0xe 0xc 0xe 0xb
1 0xe 0xe 0xe 0xd
```

=== phase5
```python
salt = "maduiersnfotvbyl"

def encrypt(c: chr) -> chr:
    return salt[ord(c) % len(salt)]

for c in "123456":
    print(encrypt(c), end="")
print()

target = "flyers"
smap = {}
for c in [chr(u) for u in range(100)] + list(salt):
    smap[c] = encrypt(c)
smap_rev = {v: k for k, v in smap.items()}
print(smap_rev)
print(target, "==>", "".join([smap_rev[c] for c in target]))
```

=== phase6
+ 一共有6个数字
+ $n_0 <= 6$

```
Dump of assembler code for function phase_6:
   0x00000000004010f4 <+0>:     push   r14
   0x00000000004010f6 <+2>:     push   r13
   0x00000000004010f8 <+4>:     push   r12
   0x00000000004010fa <+6>:     push   rbp
   0x00000000004010fb <+7>:     push   rbx
   0x00000000004010fc <+8>:     sub    rsp,0x50
   0x0000000000401100 <+12>:    mov    r13,rsp
   0x0000000000401103 <+15>:    mov    rsi,rsp
   0x0000000000401106 <+18>:    call   0x40145c <read_six_numbers>
   0x000000000040110b <+23>:    mov    r14,rsp
   0x000000000040110e <+26>:    mov    r12d,0x0

   0x0000000000401114 <+32>:    mov    rbp,r13
   0x0000000000401117 <+35>:    mov    eax,DWORD PTR [r13+0x0] ; 读取第一个数字到rax

   0x000000000040111b <+39>:    sub    eax,0x1
   0x000000000040111e <+42>:    cmp    eax,0x5 ; compare num0 - 1 - 5
   0x0000000000401121 <+45>:    jbe    0x401128 <phase_6+52> ; jump if num0 <= 6
   0x0000000000401123 <+47>:    call   0x40143a <explode_bomb>

   0x0000000000401128 <+52>:    add    r12d,0x1 ; r12d += 1 
   0x000000000040112c <+56>:    cmp    r12d,0x6 ; cmp r12d - 6
   0x0000000000401130 <+60>:    je     0x401153 <phase_6+95> ; loop till r12d -> 6
   0x0000000000401132 <+62>:    mov    ebx,r12d 
   0x0000000000401135 <+65>:    movsxd rax,ebx  ; rax = ebx = r12d
   0x0000000000401138 <+68>:    mov    eax,DWORD PTR [rsp+rax*4] ; 读取第 r12d 参数到 rax
   0x000000000040113b <+71>:    cmp    DWORD PTR [rbp+0x0],eax   ; rax - num0
   0x000000000040113e <+74>:    jne    0x401145 <phase_6+81>     ; 第 r12d 个参数不能等于第一个，否则explode
   0x0000000000401140 <+76>:    call   0x40143a <explode_bomb>

   0x0000000000401145 <+81>:    add    ebx,0x1                   ; ebx += 1, ebx <- r12d
   0x0000000000401148 <+84>:    cmp    ebx,0x5                   ; ebx - 5
   0x000000000040114b <+87>:    jle    0x401135 <phase_6+65>     ; jump if ebx <= 5
   // 65 - 87 的循环，验证了 num1-num5 的值不能与num0相等

   0x000000000040114d <+89>:    add    r13,0x4
   0x0000000000401151 <+93>:    jmp    0x401114 <phase_6+32>
   // r13 向下迭代，然后重复到 32 的指令
   // 综合上面的解释，这里要求 num0 - num5 均满足 <= 6 同时要求不能重复 
   // 同时不断迭代 r12d 知道 60 行跳出循环

   //////////////////////////////////////////////////////
   0x0000000000401153 <+95>:    lea    rsi,[rsp+0x18]
   0x0000000000401158 <+100>:   mov    rax,r14
   0x000000000040115b <+103>:   mov    ecx,0x7
   0x0000000000401160 <+108>:   mov    edx,ecx
   0x0000000000401162 <+110>:   sub    edx,DWORD PTR [rax]
   0x0000000000401164 <+112>:   mov    DWORD PTR [rax],edx
   0x0000000000401166 <+114>:   add    rax,0x4
   0x000000000040116a <+118>:   cmp    rax,rsi
   0x000000000040116d <+121>:   jne    0x401160 <phase_6+108>
   // 以上将 [num0, .. num5] 变成 [7-num0, .. 7-num5]

   //////////////////////////////////////////////////////
   0x000000000040116f <+123>:   mov    esi,0x0
   0x0000000000401174 <+128>:   jmp    0x401197 <phase_6+163>
   // 以上色图 esi = 0, 然后跳转到 163

   0x0000000000401176 <+130>:   mov    rdx,QWORD PTR [rdx+0x8]
   0x000000000040117a <+134>:   add    eax,0x1
   0x000000000040117d <+137>:   cmp    eax,ecx
   0x000000000040117f <+139>:   jne    0x401176 <phase_6+130>
   0x0000000000401181 <+141>:   jmp    0x401188 <phase_6+148>
   0x0000000000401183 <+143>:   mov    edx,0x6032d0
   0x0000000000401188 <+148>:   mov    QWORD PTR [rsp+rsi*2+0x20],rdx
   0x000000000040118d <+153>:   add    rsi,0x4
   0x0000000000401191 <+157>:   cmp    rsi,0x18
   0x0000000000401195 <+161>:   je     0x4011ab <phase_6+183>

   0x0000000000401197 <+163>:   mov    ecx,DWORD PTR [rsp+rsi*1]
   0x000000000040119a <+166>:   cmp    ecx,0x1
   0x000000000040119d <+169>:   jle    0x401183 <phase_6+143> ; 如果 num[rsi] <= 1 跳转 143

   0x000000000040119f <+171>:   mov    eax,0x1
   0x00000000004011a4 <+176>:   mov    edx,0x6032d0
   0x00000000004011a9 <+181>:   jmp    0x401176 <phase_6+130>
   // 以上直到130 将 [node0, .. node5] 写入stack，node0 = 0x6032d0 后面依次增加 0x10

   0x00000000004011ab <+183>:   mov    rbx,QWORD PTR [rsp+0x20] ; 将 node1 的值 写入 rbx
   0x00000000004011b0 <+188>:   lea    rax,[rsp+0x28]           ; rax = node2
   0x00000000004011b5 <+193>:   lea    rsi,[rsp+0x50]           ; rsi = node1 + 0x30 = node7
   0x00000000004011ba <+198>:   mov    rcx,rbx                  ; rbx 移动到 rcx

   0x00000000004011bd <+201>:   mov    rdx,QWORD PTR [rax]      ; node2 写入 rdx
   0x00000000004011c0 <+204>:   mov    QWORD PTR [rcx+0x8],rdx  ; 将 node2 的值，写入 node1+0x8 地址中
   0x00000000004011c4 <+208>:   add    rax,0x8
   0x00000000004011c8 <+212>:   cmp    rax,rsi
   0x00000000004011cb <+215>:   je     0x4011d2 <phase_6+222>
   0x00000000004011cd <+217>:   mov    rcx,rdx
   0x00000000004011d0 <+220>:   jmp    0x4011bd <phase_6+201>
   // 以上是各循环，将6位数字节复制 rax -> rcx 中
   // 5 6 4 3 2 1: node2 - node1
   // 1 6 4 3 2 5: node6 - node1 => node1 - node3
   // 1 2 4 3 6 5: node6 - node5
   // 2 1 4 3 6 5: node5 - node6 => node6 - node3
   // 2 1 3 4 6 5: node5 - node6 => node6 - node4
   // 综上所诉，可以看出，是重新对node做了排序，7-n 为node，需要产生node序列为降序
   // 查看mem，可以看到 [node0, .., node5] = [332, 168, 924, 691, 477, 443]
   // 降序序号为：[3 4 5 6 1 2]
   // 对应输入为：[4 3 2 1 6 5]
  // pwndbg> x/32wd 0x6032d0                                                                                                                                                                                                                                                                                                                                    
  // 0x6032d0 <node1>:       332     1       6304480 0                                                                                                                                                                                                                                                                                                          
  // 0x6032e0 <node2>:       168     2       0       0                                                                                                                                                                                                                                                                                                          
  // 0x6032f0 <node3>:       924     3       6304464 0                                                                                                                                                                                                                                                                                                          
  // 0x603300 <node4>:       691     4       6304496 0                                                                                                                                                                                                                                                                                                          
  // 0x603310 <node5>:       477     5       6304544 0                                                                                                                                                                                                                                                                                                          
  // 0x603320 <node6>:       443     6       6304512 0                                                                                                                                                                                                                                                                                                          

   0x00000000004011d2 <+222>:   mov    QWORD PTR [rdx+0x8],0x0
   0x00000000004011da <+230>:   mov    ebp,0x5
   0x00000000004011df <+235>:   mov    rax,QWORD PTR [rbx+0x8]
   0x00000000004011e3 <+239>:   mov    eax,DWORD PTR [rax]
   0x00000000004011e5 <+241>:   cmp    DWORD PTR [rbx],eax
   0x00000000004011e7 <+243>:   jge    0x4011ee <phase_6+250>
   0x00000000004011e9 <+245>:   call   0x40143a <explode_bomb>
   0x00000000004011ee <+250>:   mov    rbx,QWORD PTR [rbx+0x8]
   0x00000000004011f2 <+254>:   sub    ebp,0x1
   0x00000000004011f5 <+257>:   jne    0x4011df <phase_6+235>
   0x00000000004011f7 <+259>:   add    rsp,0x50
   0x00000000004011fb <+263>:   pop    rbx
   0x00000000004011fc <+264>:   pop    rbp
   0x00000000004011fd <+265>:   pop    r12
   0x00000000004011ff <+267>:   pop    r13
   0x0000000000401201 <+269>:   pop    r14
   0x0000000000401203 <+271>:   ret
```

=== phase_defused
```asm
Dump of assembler code for function phase_defused:
=> 0x00000000004015c4 <+0>:     sub    rsp,0x78
   0x00000000004015c8 <+4>:     mov    rax,QWORD PTR fs:0x28
   0x00000000004015d1 <+13>:    mov    QWORD PTR [rsp+0x68],rax
   0x00000000004015d6 <+18>:    xor    eax,eax
   0x00000000004015d8 <+20>:    cmp    DWORD PTR [rip+0x202181],0x6        # 0x603760 <num_input_strings>
   0x00000000004015df <+27>:    jne    0x40163f <phase_defused+123>
   0x00000000004015e1 <+29>:    lea    r8,[rsp+0x10]
   0x00000000004015e6 <+34>:    lea    rcx,[rsp+0xc]
   0x00000000004015eb <+39>:    lea    rdx,[rsp+0x8]
   0x00000000004015f0 <+44>:    mov    esi,0x402619
   0x00000000004015f5 <+49>:    mov    edi,0x603870
   0x00000000004015fa <+54>:    call   0x400bf0 <__isoc99_sscanf@plt>
   0x00000000004015ff <+59>:    cmp    eax,0x3
   0x0000000000401602 <+62>:    jne    0x401635 <phase_defused+113>
   0x0000000000401604 <+64>:    mov    esi,0x402622
   0x0000000000401609 <+69>:    lea    rdi,[rsp+0x10]
   0x000000000040160e <+74>:    call   0x401338 <strings_not_equal>
   0x0000000000401613 <+79>:    test   eax,eax
   0x0000000000401615 <+81>:    jne    0x401635 <phase_defused+113>
   0x0000000000401617 <+83>:    mov    edi,0x4024f8
   0x000000000040161c <+88>:    call   0x400b10 <puts@plt>
   0x0000000000401621 <+93>:    mov    edi,0x402520
   0x0000000000401626 <+98>:    call   0x400b10 <puts@plt>
   0x000000000040162b <+103>:   mov    eax,0x0
   0x0000000000401630 <+108>:   call   0x401242 <secret_phase>
   0x0000000000401635 <+113>:   mov    edi,0x402558
   0x000000000040163a <+118>:   call   0x400b10 <puts@plt>
   0x000000000040163f <+123>:   mov    rax,QWORD PTR [rsp+0x68]
   0x0000000000401644 <+128>:   xor    rax,QWORD PTR fs:0x28
   0x000000000040164d <+137>:   je     0x401654 <phase_defused+144>
   0x000000000040164f <+139>:   call   0x400b30 <__stack_chk_fail@plt>
   0x0000000000401654 <+144>:   add    rsp,0x78
   0x0000000000401658 <+148>:   ret

pwndbg> disas secret_phase
Dump of assembler code for function secret_phase:
   0x0000000000401242 <+0>:     push   rbx
   0x0000000000401243 <+1>:     call   0x40149e <read_line>
   0x0000000000401248 <+6>:     mov    edx,0xa
   0x000000000040124d <+11>:    mov    esi,0x0
   0x0000000000401252 <+16>:    mov    rdi,rax
   0x0000000000401255 <+19>:    call   0x400bd0 <strtol@plt>
   0x000000000040125a <+24>:    mov    rbx,rax
   0x000000000040125d <+27>:    lea    eax,[rax-0x1]
   0x0000000000401260 <+30>:    cmp    eax,0x3e8
   0x0000000000401265 <+35>:    jbe    0x40126c <secret_phase+42>
   0x0000000000401267 <+37>:    call   0x40143a <explode_bomb>
   0x000000000040126c <+42>:    mov    esi,ebx
   0x000000000040126e <+44>:    mov    edi,0x6030f0
=> 0x0000000000401273 <+49>:    call   0x401204 <fun7>
   0x0000000000401278 <+54>:    cmp    eax,0x2
   0x000000000040127b <+57>:    je     0x401282 <secret_phase+64>
   0x000000000040127d <+59>:    call   0x40143a <explode_bomb>
   0x0000000000401282 <+64>:    mov    edi,0x402438
   0x0000000000401287 <+69>:    call   0x400b10 <puts@plt>
   0x000000000040128c <+74>:    call   0x4015c4 <phase_defused>
   0x0000000000401291 <+79>:    pop    rbx
   0x0000000000401292 <+80>:    ret

// crack 需要 fun7 的返回值为2
Dump of assembler code for function fun7:
   0x0000000000401204 <+0>:     sub    rsp,0x8  ; 分配 stack
   0x0000000000401208 <+4>:     test   rdi,rdi  
   0x000000000040120b <+7>:     je     0x401238 <fun7+52> ; rdi 地址为0就跳转到 +52
   0x000000000040120d <+9>:     mov    edx,DWORD PTR [rdi]; [rdi] -> edx
   0x000000000040120f <+11>:    cmp    edx,esi            ;
   0x0000000000401211 <+13>:    jle    0x401220 <fun7+28> ; jump if edx <= rsi(arg0)
   0x0000000000401213 <+15>:    mov    rdi,QWORD PTR [rdi+0x8] ; rdi 后移动 0x8
   0x0000000000401217 <+19>:    call   0x401204 <fun7>         ; 递归
   0x000000000040121c <+24>:    add    eax,eax                 ; return rax *= 2
   0x000000000040121e <+26>:    jmp    0x40123d <fun7+57>

   0x0000000000401220 <+28>:    mov    eax,0x0 ; rax = 0
   0x0000000000401225 <+33>:    cmp    edx,esi ; edx - rsi(arg0) ; return rax = 0
   0x0000000000401227 <+35>:    je     0x40123d <fun7+57>

   0x0000000000401229 <+37>:    mov    rdi,QWORD PTR [rdi+0x10] ; rdi 后移动 0x10
   0x000000000040122d <+41>:    call   0x401204 <fun7>          ; 递归
   0x0000000000401232 <+46>:    lea    eax,[rax+rax*1+0x1]      ; return rax = rax*2 + 1
   0x0000000000401236 <+50>:    jmp    0x40123d <fun7+57>

   0x0000000000401238 <+52>:    mov    eax,0xffffffff ; return 0xffffffff
   0x000000000040123d <+57>:    add    rsp,0x8
   0x0000000000401241 <+61>:    ret
// 要想返回 2，很容易推到 返回分支依次应该是 +33 -> +46 -> +24，让返回的值变为 0 -> 1 -> 2
// 需要满足条件
// edx == arg0 
// edx < arg0 
// edx > arg0
//
// edx 每次进入fun7 都会变化，根据gdb反推每次call的参数，很幸运，arg0 永远不变，这让反推变得非常简单
// 1. arg0 < 36
// 2. 8 < arg0
// 3. 22 == arg0
```

== Lab::Attack
+ run `attack.hex2raw.py attack.ctarget.01.in` to get output text for 01. Same for other levels.
+ ref to https://pwn.elmo.sg/miscellaneous/movaps-issue.html to fix movaps bugs. This is not mentioned by instructor.
+ use pwndbg to save your life.
