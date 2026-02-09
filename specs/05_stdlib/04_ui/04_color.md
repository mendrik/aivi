# Color Domain

The `Color` domain helps you work with **Colors** the way humans do.

Screens think in Red, Green, and Blue, but people think in **Hue**, **Saturation**, and **Lightness**. This domain lets you mix colors mathematically (e.g., `primary + 10% lightness` for a hover state) without the mud that comes from raw RGB math.

## Overview

```aivi
import aivi.std.ui.color use { Color }

primary = #007bff
// Mathematically correct lightening
lighter = primary + 10`lightness`
```

## Features

```aivi
Rgb = { r: Int, g: Int, b: Int }  // 0-255
Hsl = { h: Float, s: Float, l: Float }  // h: 0-360, s/l: 0-1
Hex = Text  // "#rrggbb"
```

## Domain Definition

```aivi
domain Color over Rgb = {
  type Delta = Lightness Int | Saturation Int | Hue Int
  
  (+) : Rgb -> Delta -> Rgb
  (+) col (Lightness n) = adjustLightness col n
  (+) col (Saturation n) = adjustSaturation col n
  (+) col (Hue n) = adjustHue col n
  
  (-) : Rgb -> Delta -> Rgb
  (-) col delta = col + (negateDelta delta)
  
  // Delta literals
  1l = Lightness 1
  1s = Saturation 1
  1h = Hue 1
}
```

## Helper Functions

```aivi
adjustLightness : Rgb -> Int -> Rgb
adjustLightness col n = 
  col
    |> toHsl
    |> (hsl => hsl <| { l: clamp 0 1 (hsl.l + n / 100) })
    |> toRgb

adjustSaturation : Rgb -> Int -> Rgb
adjustSaturation col n =
  col
    |> toHsl
    |> (hsl => hsl <| { s: clamp 0 1 (hsl.s + n / 100) })
    |> toRgb

adjustHue : Rgb -> Int -> Rgb
adjustHue col n =
  col
    |> toHsl
    |> (hsl => hsl <| { h: (hsl.h + n) % 360 })
    |> toRgb

toRgb : Hsl -> Rgb
toRgb { h, s, l } = // HSL to RGB conversion

toHsl : Rgb -> Hsl
toHsl { r, g, b } = // RGB to HSL conversion

toHex : Rgb -> Hex
toHex { r, g, b } = // Format as "#rrggbb"
```

## Usage Examples

```aivi
use aivi.std.color

primary = { r: 255, g: 85, b: 0 }  // Orange

lighter = primary + 10l
darker = primary - 20l
muted = primary - 30s
shifted = primary + 30h
```
