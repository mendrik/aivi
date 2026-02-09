# Complex Domain

The `Complex` domain supports arithmetic with **Complex Numbers**.

A complex number expands the idea of the number line into a 2D plane. It has a **Real** part (normal numbers) and an **Imaginary** part (multiples of `i`, where `i` is the square root of -1). 

While "imaginary" sounds made up, these numbers effectively power modern civilization. They are the native language of electricity, radio waves, and signal processing. Trying to simulate circuits using separate `x` and `y` float variables is like doing arithmetic with Roman numerals—tedious and error-prone. This domain lets you treat them as first-class values.

## Overview

```aivi
import aivi.std.math.complex use { Complex, i }

// 3.0 Real, 4.0 Imaginary
z1 = 3.0 + 4.0 * i
z2 = 1.0 - 2.0 * i

// Add them just like normal numbers
sum = z1 + z2
```

## Features

```aivi
Complex = { re: Float, im: Float }

Scalar = Float
```

## Domain Definition

```aivi
domain Complex over Complex = {
  (+) : Complex -> Complex -> Complex
  (+) a b = { re: a.re + b.re, im: a.im + b.im }
  
  (-) : Complex -> Complex -> Complex
  (-) a b = { re: a.re - b.re, im: a.im - b.im }
  
  (*) : Complex -> Complex -> Complex
  (*) a b = {
    re: a.re * b.re - a.im * b.im,
    im: a.re * b.im + a.im * b.re
  }
  
  (/) : Complex -> Scalar -> Complex
  (/) z s = { re: z.re / s, im: z.im / s }
}
```

## Helper Functions

```aivi
fromPolar : Float -> Float -> Complex
fromPolar r theta = { re: r * cos theta, im: r * sin theta }

conjugate : Complex -> Complex
conjugate z = { re: z.re, im: -z.im }

magnitude : Complex -> Float
magnitude z = sqrt (z.re * z.re + z.im * z.im)

phase : Complex -> Float
phase z = atan2 z.im z.re
```

## Usage Examples

```aivi
use aivi.std.complex

z1 = { re: 3.0, im: 4.0 }
z2 = fromPolar 1.0 1.570796

sum = z1 + z2
product = z1 * z2
unit = z1 / magnitude z1
```
