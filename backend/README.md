## Backend

* Layered Architecture ✅
* sqlx migrate (sqlx-cli)
* Service key management
* Unit test

DB Migration

1. Install
```
$ cargo install sqlx-cli
```

2. Create .env and replace DATABASE_URL

```
$ cp .env.sample .env
```

3. Generate migration file

```
$ sqlx migrate add <name>
```

4. Execute migration

```
$ sqlx migrate run
```

