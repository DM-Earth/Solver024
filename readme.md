# Solver024

This is a simple 24 points solver written in rust.

## Example

```log
1 9 7 8
(1 * 7) + (8 + 9)
1 * (7 + (8 + 9))
((1 * 7) + 8) + 9
((1 - 7) + 9) * 8
((1 * 7) + 9) + 8
(1 * 8) + (7 + 9)
1 * (8 + (7 + 9))
((1 * 8) + 7) + 9
((1 * 8) + 9) + 7
((1 + 9) - 7) * 8
(1 * 9) + (7 + 8)
1 * (9 + (7 + 8))
((1 * 9) + 7) + 8
((1 * 9) + 8) + 7
7 + (1 * (8 + 9))
(7 / 1) + (8 + 9)
Found 16 solutions in 0.308 ms
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
