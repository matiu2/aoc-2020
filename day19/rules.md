# Rule breakdown

This is my manual breakdown of the sample rules, and expansion into their
possiblilities. Used for debugging.

    0: 8 11
    1: "a"
    2: 1 24 | 14 4  : ABA | BAA
    3: 5 14 | 16 1
    4: 1 1          : AA
    5: 1 14 | 15 1  : AB | AA | BA
    6: 14 14 | 1 14 : BB | AB
    7: 14 5 | 1 21  : BAB | BAA | BBA | ABA | AAB
    8: 42 | 42 8
    9: 14 27 | 1 26  : BABB | BAAB | BBAA | BBAB | BBBA | BBBB | ABBB | AABB | AAAA | AAAB
    10: 23 14 | 28 1 : AAAB | ABAB | BBBB | AAAA | BAAA | BBAA
    11: 42 31 | 42 11 31
    12: 24 14 | 19 1 : BAB | BAA | BBA
    13: 14 3 | 1 12  : BABB | BAAB | BBAB | ABAB | ABAA | ABBA
    14: "b"
    15: 1 | 14       : A | B
    16: 15 1 | 14 14 : AA | BA | BB
    17: 14 2 | 1 7   : BABA | BBAA | ABAB | ABAA | ABBA | AABA | AAAB
    18: 15 15        : AA | AB | BA | BB
    19: 14 1 | 14 14 : BA | BB
    20: 14 14 | 1 15 : BB | AA | AB
    21: 14 1 | 1 14  : BA | AB
    22: 14 14       : BB
    23: 25 1 | 22 14: AAA | ABA | BB
    24: 14 1        : BA
    25: 1 1 | 1 14  : AA | AB
    26: 14 22 | 1 20: BBB | ABB | AAA | AAB
    27: 1 6 | 14 18 : ABB | AAB | BAA | BAB | BBA | BBB
    28: 16 1        : AAA | BAA | BBA
    31: 14 17 | 1 13: BBABA | BBBAA | BABAB | BABAA | BABBA | BAABA | BAAAB | ABABB | ABAAB | ABBAB | AABAB | AABAA | AABBA
    42: 9 14 | 10 1 : BABBB | BAABB | BBAAB | BBABB | BBBAB | BBBBB | ABBBB | AABBB | AAAAB | AAABB | AAABA | ABABA | BBBBA | AAAAA | BAAAA | BBAAA


