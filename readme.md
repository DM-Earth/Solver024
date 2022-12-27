# Solver024

This is a simple 24 points solver written in rust.

## Example

```log
6 7 12 7
(6 + 7) + (7 ^ 12)
6 + (7 + (7 ^ 12))
6 * (7 & (7 * 12))
(6 & 7) * (7 & 12)
6 * (7 & (7 & 12))
6 << (7 + (7 - 12))
(6 << 7) >> (12 - 7)
6 << (7 - (12 - 7))
6 << (7 ^ (12 - 7))
Found 9 solutions in 0.014 ms
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
