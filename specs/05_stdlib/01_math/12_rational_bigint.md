# Rational & BigInt Domains

This domain provides **Arbitrary-Precision Integers** (`BigInt`) and **Exact Rational Numbers** (`Rational`).

Standard computers are bad at math. They run out of fingers at 9 quintillion (`Int64`), and they think `0.1 + 0.2` equals `0.30000000000000004` (Floating Point).

*   **BigInt** grows automatically to fit *any* integer number, limited only by your computer's RAM.
*   **Rational** stores exact fractions (like `1/3`), ensuring that `1/3 + 1/3 + 1/3` equals exactly `1`, not `0.999999`.

If you are doing cryptography, combinatorics, or financial calculations where a missing penny is a lawsuit, use these types.

## Overview

```aivi
import aivi.std.math.number use { BigInt, Ratio }

// Calculate with atoms in the universe
let huge = 10_000_000_000_000_000_000_000n

// Exact fraction arithmetic
let part = 1/3 + 1/6 
// -> Result is exactly 1/2, not 0.4999...
```

## Features

```aivi
BigInt = { sign: Int, limbs: List Int }
Rational = { num: BigInt, den: BigInt }
```

## Domain Definition

```aivi
domain BigInt over BigInt = {
  (+) : BigInt -> BigInt -> BigInt
  (-) : BigInt -> BigInt -> BigInt
  (*) : BigInt -> BigInt -> BigInt
}

domain Rational over Rational = {
  (+) : Rational -> Rational -> Rational
  (-) : Rational -> Rational -> Rational
  (*) : Rational -> Rational -> Rational
  (/) : Rational -> Rational -> Rational
}
```

## Helper Functions

```aivi
fromInt : Int -> BigInt
fromInt n = { sign: if n < 0 then -1 else 1, limbs: [abs n] }

gcd : BigInt -> BigInt -> BigInt
gcd a b = if b == fromInt 0 then a else gcd b (a % b)

normalize : Rational -> Rational
normalize r = {
  num: r.num / gcd r.num r.den,
  den: r.den / gcd r.num r.den
}

toFloat : Rational -> Float
toFloat r = (toFloat r.num) / (toFloat r.den)
```

## Usage Examples

```aivi
use aivi.std.rational

half = normalize { num: fromInt 1, den: fromInt 2 }
sum = half + half
```
