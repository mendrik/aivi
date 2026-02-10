# Database Domain

The `Database` domain provides a type-safe, composable way to work with relational data. It treats tables as immutable sequences of records, while compiling predicates and patches into efficient SQL under the hood.

It builds on existing AIVI features:
- **Domains** for operator overloading and delta literals
- **Predicates** for filtering and joins
- **Patching** for declarative updates
- **Effects** for explicit error handling

## Overview

```aivi
use aivi.std.system.database

User = { id: Int, name: Text, email: Text?, active: Bool, loginCount: Int, createdAt: Instant }

@static
userTable : Table User
userTable = db.table "users" User {
  id
  autoIncrement
  name
  varchar(100) notNull
  email
  varchar(255)
  active
  boolean default true
  createdAt
  timestamp default now()
}

getActiveUsers : Effect DbError (List User)
getActiveUsers = effect {
  users <- load userTable
  pure (users |> filter active |> sortBy .createdAt)
}
```

## Types

```aivi
// Tables are sequences of rows
type Table A = List A

// Predicate alias
type Pred A = A => Bool

// Deltas express insert/update/delete
type Delta A =
  | Insert A
  | Update (Pred A) (Patch A)
  | Delete (Pred A)
```

## Domain Definition

```aivi
domain Database over Table A = {
  type Delta = Delta A

  (+) : Table A -> Delta A -> Effect DbError (Table A)
  (+) table delta = applyDeltaToDb table delta

  ins = Insert
  upd = Update
  del = Delete
}
```

### Applying Deltas

```aivi
createUser : User -> Effect DbError User
createUser newUser = effect {
  _ <- userTable + ins newUser
  pure newUser
}

activateUsers : Effect DbError Unit
activateUsers = effect {
  _ <- userTable + upd (not active) { active: true, loginCount: _ + 1 }
  pure Unit
}

deleteOldPosts : Instant -> Effect DbError Unit
deleteOldPosts cutoff = effect {
  _ <- postTable + del (_.createdAt < cutoff)
  pure Unit
}
```

## Querying

Tables behave like lazy sequences. Operations such as `filter`, `find`, `sortBy`, `groupBy`, and `join` build a query plan. The query executes only when observed (e.g. via `load`, `toList`, or a generator).

```aivi
getUserById : Int -> Effect DbError (Option User)
getUserById id = effect {
  users <- load userTable
  pure (users |> find (_.id == id))
}
```

## Joins and Preloading

```aivi
UserWithPosts = { user: User, posts: List Post }

getUsersWithPosts : Effect DbError (List UserWithPosts)
getUsersWithPosts = effect {
  users <- load userTable
  posts <- load postTable
  pure (
    users
    |> join posts on (_.id == _.authorId)
    |> groupBy { userId = _.id, user = _.left, post = _.right }
    |> map { key, group } => {
      user: group.head.user,
      posts: group |> map .post
    }
  )
}
```

For eager loading:

```aivi
usersWithPosts <- load (userTable |> preload posts on (_.id == _.authorId))
```

## Migrations

Schema definitions are typed values. Mark them `@static` to allow compile-time validation and migration planning.

```aivi
migrate : Effect DbError Unit
migrate = effect {
  _ <- db.runMigrations [ userTable ]
  pure Unit
}
```

## Notes

- `Database` compiles predicate expressions into `WHERE` clauses and patch instructions into `SET` clauses.
- Joins are translated into single SQL queries to avoid N+1 patterns.
- Advanced SQL remains available via `db.query` in [External Sources](../../02_syntax/12_external_sources.md).