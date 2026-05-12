# Rich Time Card - Database Setup

This guide describes the minimal PostgreSQL database setup for local development.

The schema baseline is:

- `docs/database/ddl-postgresql.sql`

The master data seed file is:

- `docs/database/seed-master-data.sql`

## 1. Create the Database

Create a PostgreSQL database for the application:

```sh
createdb rich_time_card
```

If your environment requires an explicit user or host, pass the normal `psql` connection options, for example:

```sh
createdb -h localhost -U postgres rich_time_card
```

## 2. Run the DDL

Apply the PostgreSQL schema baseline:

```sh
psql -d rich_time_card -f docs/database/ddl-postgresql.sql
```

This creates the PostgreSQL extension, enum types, tables, indexes, and foreign keys defined by the current reviewed DDL.

## 3. Run the Master Data Seed

Apply the master data seed after the DDL:

```sh
psql -d rich_time_card -f docs/database/seed-master-data.sql
```

The seed file inserts initial records for:

- `time_zones`
- `time_grains`
- `time_conversions`

Initial supported values include:

- `UTC`
- `Asia/Tokyo`
- `America/New_York`
- 1 minute granularity
- 10 minute granularity

## 4. Verify the Setup

List the tables:

```sh
psql -d rich_time_card -c '\dt'
```

Verify the time zone seed records:

```sh
psql -d rich_time_card -c 'SELECT name, offset_minutes FROM time_zones ORDER BY name;'
```

Verify the time grain seed records:

```sh
psql -d rich_time_card -c 'SELECT grain_minutes FROM time_grains ORDER BY grain_minutes;'
```

Verify the conversion seed records:

```sh
psql -d rich_time_card -c '
SELECT g.grain_minutes, c.actual_minutes, c.decimal_value
FROM time_conversions c
JOIN time_grains g ON g.grain_id = c.grain_id
ORDER BY g.grain_minutes, c.actual_minutes;
'
```
