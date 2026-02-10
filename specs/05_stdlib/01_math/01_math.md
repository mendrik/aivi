# Math Module

The `aivi.math` module provides standard numeric functions and constants for `Int` and `Float`.
It is intentionally small, predictable, and aligned with common math libraries across languages.

## Overview

```aivi
use aivi.math

area = pi * r * r
clamped = clamp 0.0 1.0 x
```

## Constants

```aivi
pi : Float
tau : Float
e : Float
inf : Float
nan : Float
phi : Float
sqrt2 : Float
ln2 : Float
ln10 : Float
```

## Angles

Angles are represented by a dedicated domain so trigonometric functions are not called with raw `Float` values.

```aivi
Angle = { radians: Float }
```

```aivi
radians : Float -> Angle
```
Create an `Angle` from a raw radians value.

```aivi
degrees : Float -> Angle
```
Create an `Angle` from a raw degrees value.

```aivi
toRadians : Angle -> Float
```
Extract the radians value from an `Angle`.

```aivi
toDegrees : Angle -> Float
```
Extract the degrees value from an `Angle`.

## Basic helpers

```aivi
abs : Int -> Int
abs : Float -> Float
```
Absolute value of the input number.

```aivi
sign : Float -> Float
copysign : Float -> Float -> Float
```
`sign x` returns `-1.0`, `0.0`, or `1.0` based on the sign of `x`.
`copysign mag sign` returns `mag` with the sign of `sign`.

```aivi
min : Float -> Float -> Float
max : Float -> Float -> Float
minAll : List Float -> Option Float
maxAll : List Float -> Option Float
```
`min a b` and `max a b` return the smaller or larger of two values.
`minAll xs` and `maxAll xs` return the min or max of a list, or `none` when empty.

```aivi
clamp : Float -> Float -> Float -> Float
sum : List Float -> Float
sumInt : List Int -> Int
```
`clamp low high x` limits `x` to the closed interval `[low, high]`.
`sum` and `sumInt` add a list of floats or ints (empty lists yield `0.0` or `0`).

## Rounding and decomposition

```aivi
floor : Float -> Float
ceil : Float -> Float
trunc : Float -> Float
round : Float -> Float
fract : Float -> Float
```
`floor`, `ceil`, and `trunc` round toward `-inf`, `+inf`, and `0` respectively.
`round` uses banker's rounding (ties to even).
`fract x` returns the fractional part of `x` with the same sign as `x`.

```aivi
modf : Float -> (Float, Float)
frexp : Float -> (Float, Int)
ldexp : Float -> Int -> Float
```
`modf x` returns `(intPart, fracPart)` where `x = intPart + fracPart`.
`frexp x` returns `(mantissa, exponent)` such that `x = mantissa * 2^exponent`.
`ldexp mantissa exponent` computes `mantissa * 2^exponent`.

## Powers, roots, and logs

```aivi
pow : Float -> Float -> Float
sqrt : Float -> Float
cbrt : Float -> Float
hypot : Float -> Float -> Float
```
`pow base exp` raises `base` to `exp`.
`sqrt` and `cbrt` compute square and cube roots.
`hypot x y` computes `sqrt(x*x + y*y)` with reduced overflow/underflow.

```aivi
exp : Float -> Float
exp2 : Float -> Float
expm1 : Float -> Float
```
`exp x` computes `e^x`.
`exp2 x` computes `2^x`.
`expm1 x` computes `e^x - 1` with improved precision near zero.

```aivi
log : Float -> Float
log10 : Float -> Float
log2 : Float -> Float
log1p : Float -> Float
```
`log` computes the natural log.
`log10` and `log2` are base-10 and base-2 logs.
`log1p x` computes `log(1 + x)` with improved precision near zero.

## Trigonometry

```aivi
sin : Angle -> Float
cos : Angle -> Float
tan : Angle -> Float
```
`sin angle`, `cos angle`, and `tan angle` compute the trigonometric ratios for an `Angle`.

```aivi
asin : Float -> Angle
acos : Float -> Angle
atan : Float -> Angle
atan2 : Float -> Float -> Angle
```
`asin x`, `acos x`, and `atan x` return the angle whose sine, cosine, or tangent is `x`.
`atan2 y x` returns the angle of the vector `(x, y)` from the positive x-axis.

## Hyperbolic functions

```aivi
sinh : Float -> Float
cosh : Float -> Float
tanh : Float -> Float
asinh : Float -> Float
acosh : Float -> Float
atanh : Float -> Float
```
Standard hyperbolic sine, cosine, and tangent, and their inverses.

## Integer math

```aivi
gcd : Int -> Int -> Int
lcm : Int -> Int -> Int
gcdAll : List Int -> Option Int
lcmAll : List Int -> Option Int
```
`gcd a b` and `lcm a b` compute the greatest common divisor and least common multiple.
`gcdAll xs` and `lcmAll xs` fold across a list, returning `none` when empty.

```aivi
factorial : Int -> BigInt
comb : Int -> Int -> BigInt
perm : Int -> Int -> BigInt
```
`factorial n` computes `n!`.
`comb n k` computes combinations ("n choose k").
`perm n k` computes permutations ("n P k").

```aivi
divmod : Int -> Int -> (Int, Int)
modPow : Int -> Int -> Int -> Int
```
`divmod a b` returns `(q, r)` where `a = q * b + r` and `0 <= r < |b|`.
`modPow base exp modulus` computes `(base^exp) mod modulus`.

Notes:
- `BigInt` is from `aivi.number.bigint` and is re-exported by `aivi.math`.

## Floating-point checks

```aivi
isFinite : Float -> Bool
isInf : Float -> Bool
isNaN : Float -> Bool
nextAfter : Float -> Float -> Float
ulp : Float -> Float
```
`isFinite`, `isInf`, and `isNaN` test floating-point classification.
`nextAfter from to` returns the next representable float after `from` toward `to`.
`ulp x` returns the size of one unit-in-the-last-place at `x`.

## Remainders

```aivi
fmod : Float -> Float -> Float
remainder : Float -> Float -> Float
```
`fmod a b` returns the remainder using truncation toward zero.
`remainder a b` returns the IEEE-754 remainder (round-to-nearest quotient).

## Usage Examples

```aivi
use aivi.math

angle = degrees 90.0
unit = sin angle

digits = [1.0, 2.0, 3.0] |> sum
```
