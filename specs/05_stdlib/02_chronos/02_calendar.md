# Calendar Domain

The `Calendar` domain gives you robust tools for handling **Dates** and **Human Time**.

Handling time is deceptively hard. Ideally, a day is 24 hours. In reality, months have 28-31 days, years have 365 or 366 days, and timezones shift clocks back and forth.

The `Calendar` domain hides this chaos. Writing `timestamp + 86400` works until a leap second deletes your data. This domain ensures that when you say "Next Month," it handles the math correctly—whether it's February or July—making your scheduling logic reliable and legible.

## Overview

```aivi
use aivi.std.chronos.calendar (Date, DateTime)

now = DateTime.now()

birthday = ~d(1990-12-31)
timestamp = ~dt(2025-02-08T12:34:56Z)

// "Human" math: Add 7 days, regardless of seconds
next_week = now + 7days
```

## Features

```aivi
Date = { year: Int, month: Int, day: Int }

EndOfMonth = EndOfMonth
```

## Domain Definition

```aivi
domain Calendar over Date = {
  type Delta = Day Int | Month Int | Year Int | End EndOfMonth
  
  // Add delta to date
  (+) : Date -> Delta -> Date
  (+) date (Day n)   = addDays date n
  (+) date (Month n) = addMonths date n
  (+) date (Year n)  = addYears date n
  (+) date End       = endOfMonth date
  
  // Subtract delta from date
  (-) : Date -> Delta -> Date
  (-) date delta = date + (negateDelta delta)
  
  // Delta literals
  1d = Day 1
  1m = Month 1
  1y = Year 1
  eom = End
}
```

## Helper Functions

```aivi
isLeapYear : Date -> Bool
isLeapYear { year } = 
  (year % 4 == 0) && ((year % 100 != 0) || (year % 400 == 0))

daysInMonth : Date -> Int
daysInMonth { year, month } =
  month ?
  | 2  => if isLeapYear { year } then 29 else 28
  | 4  => 30
  | 6  => 30
  | 9  => 30
  | 11 => 30
  | _  => 31

endOfMonth : Date -> Date
endOfMonth date = date <| { day: daysInMonth date }

addDays : Date -> Int -> Date
addDays date n = // normalize day overflow/underflow

addMonths : Date -> Int -> Date
addMonths date n = // normalize month overflow, clamp days

addYears : Date -> Int -> Date
addYears { year, month, day } n = { year: year + n, month, day }

negateDelta : Delta -> Delta
negateDelta (Day n)   = Day (-n)
negateDelta (Month n) = Month (-n)
negateDelta (Year n)  = Year (-n)
negateDelta End       = End  // idempotent
```

## Usage Examples

```aivi
use aivi.std.calendar

today = { year: 2025, month: 2, day: 8 }

tomorrow = today + 1d
nextMonth = today + 1m
lastYear = today - 1y
monthEnd = today + eom
```
