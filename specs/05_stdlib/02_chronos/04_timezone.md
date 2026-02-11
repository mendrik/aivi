# TimeZone and ZonedDateTime

The `TimeZone` and `ZonedDateTime` domains handle geographic time offsets, daylight saving transitions, and global time coordination.

## Overview

```aivi
use aivi.calendar (DateTime)
use aivi.chronos.timezone (TimeZone, ZonedDateTime)

// Instantiation
paris = ~tz(Europe/Paris)
tokyo = ~tz(Asia/Tokyo)

// Creating a ZonedDateTime
meeting = ~zdt(2024-05-21T12:00:00[Europe/Paris])

// Conversion
local_time = meeting.dateTime
utc_moment = meeting.instant
```

## Features

```aivi
TimeZone = { id: Text }

ZonedDateTime = {
  dateTime: DateTime,
  zone: TimeZone,
  offset: Duration
}
```

## Domain Definition

```aivi
domain TimeZone over TimeZone = {
  // Returns the offset at a specific instant
  getOffset : TimeZone -> Instant -> Duration
}

domain ZonedDateTime over ZonedDateTime = {
  // Conversion to physics time (UTC)
  toInstant : ZonedDateTime -> Instant
  
  // Changing zones (keeping the same instant)
  atZone : ZonedDateTime -> TimeZone -> ZonedDateTime
}
```

## Usage Examples

```aivi
use aivi.calendar
use aivi.chronos.timezone

original = ~zdt(2024-01-01T10:00:00[Europe/London])
travel   = original |> ZonedDateTime.atZone ~tz(America/New_York)

// travel is ~zdt(2024-01-01T05:00:00[America/New_York])
```
