# Solver024

This is a simple 24 points solver written in rust.

## Example

```log
7 7 4 2
2 * (7 ^ (4 + 7))
(2 - 7) & (4 * 7)
4 ^ (2 * (7 + 7))
4 * (7 & (2 * 7))
4 ^ (7 << (2 & 7))
4 * (7 - (7 >> 2))
4 * (7 ^ (7 >> 2))
(4 & 7) ^ (7 << 2)
(4 ^ 7) << (7 / 2)
(7 << 2) - (4 & 7)
(7 / 2) << (4 ^ 7)
(7 / 2) << (7 - 4)
(7 - 4) << (7 / 2)
(7 * 7) >> (4 >> 2)
Found 14 solutions in 2.101 ms
```

## Operators

`+, -, *, /`

With super mode:

`>>, <<, ^, &`

## Usage Commands

### Toggle super mode

`super`

### Configure count limit

`max <limit>`

### Exit

`exit` `quit`
