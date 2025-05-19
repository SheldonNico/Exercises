#import "@preview/codly:1.3.0": *
#show: codly-init.with()

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
