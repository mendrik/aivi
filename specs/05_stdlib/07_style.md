# Standard Library: Style Domain

## Module

```aivi
module aivi.std.style = {
  export domain Style
  export CssValue
  // Absolute length units
  export Px, Cm, Mm, Q, In, Pt, Pc
  // Font-relative length units
  export Em, Rem, Ex, Ch, Cap, Ic, Lh, Rlh
  // Viewport length units
  export Vw, Vh, Vi, Vb, Vmin, Vmax
  export Svw, Svh, Svi, Svb, Svmin, Svmax
  export Lvw, Lvh, Lvi, Lvb, Lvmin, Lvmax
  export Dvw, Dvh, Dvi, Dvb, Dvmin, Dvmax
  // Container query length units
  export Cqw, Cqh, Cqi, Cqb, Cqmin, Cqmax
  // Percentage
  export Pct
  export StyleSheet
}
```

## Types

```aivi
CssValue =
  // Absolute length units
  Px Int
  | Cm Float | Mm Float | Q Float | In Float | Pt Float | Pc Float
  // Font-relative length units
  | Em Float | Rem Float | Ex Float | Ch Float | Cap Float | Ic Float | Lh Float | Rlh Float
  // Viewport length units (classic + logical + dynamic viewport variants)
  | Vw Float | Vh Float | Vi Float | Vb Float | Vmin Float | Vmax Float
  | Svw Float | Svh Float | Svi Float | Svb Float | Svmin Float | Svmax Float
  | Lvw Float | Lvh Float | Lvi Float | Lvb Float | Lvmin Float | Lvmax Float
  | Dvw Float | Dvh Float | Dvi Float | Dvb Float | Dvmin Float | Dvmax Float
  // Container query length units
  | Cqw Float | Cqh Float | Cqi Float | Cqb Float | Cqmin Float | Cqmax Float
  // Percentage + keywords
  | Pct Float | Auto

StyleSheet = List (Text, CssValue)  // property -> value
```

## Domain Definition

```aivi
domain Style over StyleSheet = {
  // Merge style sheets
  (+) : StyleSheet -> StyleSheet -> StyleSheet
  (+) base override = mergeStyles base override
  
  // Add single property
  (+) : StyleSheet -> (Text, CssValue) -> StyleSheet
  (+) sheet prop = sheet + [prop]
}
```

## Delta Literals (CSS Units)

```aivi
// Pixels
10px = Px 10

// Absolute length units
1cm = Cm 1.0
1mm = Mm 1.0
1q = Q 1.0
1in = In 1.0
1pt = Pt 1.0
1pc = Pc 1.0

// Relative units
1em = Em 1.0
1rem = Rem 1.0
1ex = Ex 1.0
1ch = Ch 1.0
1cap = Cap 1.0
1ic = Ic 1.0
1lh = Lh 1.0
1rlh = Rlh 1.0

// Percentage (preferred)
50% = Pct 50.0

// Percentage (alias)
50pct = Pct 50.0

// Viewport units
100vh = Vh 100.0
100vw = Vw 100.0
100vi = Vi 100.0
100vb = Vb 100.0
100vmin = Vmin 100.0
100vmax = Vmax 100.0

// Small / large / dynamic viewport units
100svh = Svh 100.0
100svw = Svw 100.0
100lvh = Lvh 100.0
100lvw = Lvw 100.0
100dvh = Dvh 100.0
100dvw = Dvw 100.0

// Container query units
100cqw = Cqw 100.0
100cqh = Cqh 100.0
100cqi = Cqi 100.0
100cqb = Cqb 100.0
100cqmin = Cqmin 100.0
100cqmax = Cqmax 100.0
```

## Usage Examples

```aivi
use aivi.std.style

baseCard = [
  ("padding", 16px)
  ("margin", 8px)
  ("borderRadius", 4px)
]

primaryCard = baseCard + [
  ("backgroundColor", "#007bff")
  ("color", "white")
]

fullWidth = [
  ("width", 100%)
  ("maxWidth", 1200px)
]

container = baseCard + fullWidth
```

## Type Safety

```aivi
// Type error: cannot add incompatible units
bad = 10px + 5  // Error: Int is not CssValue

// Type error: wrong context
bad2 = someDate + 10px  // Error: Date is not StyleSheet carrier
```
