# Solver024

This is a simple 24 points solver written in rust.

## Example

```log
7 7 2 9
(2 + 7) + (7 | 9)
2 + (7 + (7 | 9))
(2 - -7) + (7 | 9)
2 + (7 - (-7 & -9))
((2 + 7) | 7) + 9
((2 + 7) | 7) - -9
(2 + 7) - (-7 & -9)
2 - (-7 + (-7 & -9))
2 - (-7 - (7 | 9))
(2 - -7) - (-7 & -9)
((2 - -7) | 7) + 9
((2 - -7) | 7) - -9
((2 * 7) | 7) + 9
((-2 * -7) | 7) + 9
((2 * 7) | 7) - -9
((-2 * -7) | 7) - -9
Found 16 solutions in 0.916 ms
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
