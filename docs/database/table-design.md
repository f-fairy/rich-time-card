# Rich Time Card - Table Design

## 1. Overview

This document defines database table design for Rich Time Card.

The database consists of:

- Master tables
- User and group tables
- Attendance tables
- Supplementary tables

Database engine:

- PostgreSQL

---

## 2. Master Tables

### 2.1 time_zones

Purpose:

- Store supported time zones

This table is master data.

Example values:

- Asia/Tokyo
- UTC
- America/New_York

Columns:

| Column Name | Type | Nullable | Description |
|-------------|------|----------|-------------|
| time_zone_id | UUID | No | Primary key |
| zone_name | TEXT | No | Time zone name |
| created_at | TIMESTAMPTZ | No | Creation timestamp |

Constraints:

- zone_name must be unique

---

### 2.2 time_grains

Purpose:

- Store supported aggregation granularity

This table is master data.

Example values:

- 1 minute
- 10 minutes

Columns:

| Column Name | Type | Nullable | Description |
|-------------|------|----------|-------------|
| time_grain_id | UUID | No | Primary key |
| grain_minutes | INTEGER | No | Granularity minutes |
| created_at | TIMESTAMPTZ | No | Creation timestamp |

Constraints:

- grain_minutes must be unique

---

### 2.3 time_conversions

Purpose:

- Store minute conversion rules

Example:

- 60 minutes → 1.0 hour
- 30 minutes → 0.5 hour

Columns:

| Column Name | Type | Nullable | Description |
|-------------|------|----------|-------------|
| conversion_id | UUID | No | Primary key |
| minutes | INTEGER | No | Minute value |
| decimal_hours | NUMERIC(5,2) | No | Converted decimal hour |
| created_at | TIMESTAMPTZ | No | Creation timestamp |

Constraints:

- minutes must be unique

## 3. User and Group Tables

### 3.1 user_groups

Purpose:

- Store group information
- Manage attendance calculation settings

Columns:

| Column Name | Type | Nullable | Description |
|-------------|------|----------|-------------|
| group_id | UUID | No | Primary key |
| group_name | TEXT | No | Group name |
| time_zone_id | UUID | No | Time zone reference |
| closing_day | INTEGER | No | Monthly closing day |
| time_grain_id | UUID | No | Time granularity reference |
| work_start_time | TIME | Yes | Work start time |
| work_end_time | TIME | Yes | Work end time |
| lunch_break_start_time | TIME | Yes | Lunch break start |
| lunch_break_end_time | TIME | Yes | Lunch break end |
| created_at | TIMESTAMPTZ | No | Creation timestamp |
| updated_at | TIMESTAMPTZ | No | Update timestamp |

Relationships:

- References `time_zones`
- References `time_grains`

Rules:

- One group has many users.
- Time zone is configured at group level.
- Time granularity is configured at group level.

---

### 3.2 users

Purpose:

- Store user information
- Store authentication information

Columns:

| Column Name | Type | Nullable | Description |
|-------------|------|----------|-------------|
| user_id | UUID | No | Primary key |
| group_id | UUID | No | Group reference |
| first_name | TEXT | No | First name |
| last_name | TEXT | No | Last name |
| user_code | TEXT | Yes | Optional user ID |
| email | TEXT | No | Login email |
| password_hash | TEXT | No | Password hash |
| is_admin | BOOLEAN | No | Administrator flag |
| is_active | BOOLEAN | No | Active flag |
| created_at | TIMESTAMPTZ | No | Creation timestamp |
| updated_at | TIMESTAMPTZ | No | Update timestamp |

Relationships:

- References `user_groups`

Constraints:

- email must be unique system-wide
- user_code must be unique within group

Rules:

- Every user belongs to one group.
- Administrator authority is managed by `is_admin`.
- Soft delete is preferred using `is_active`.

## 4. Attendance Tables

### 4.1 attendance_events

Purpose:

- Store immutable attendance event history
- Act as source of truth

Characteristics:

- Append-only
- Not updated
- Not deleted

Columns:

| Column Name | Type | Nullable | Description |
|-------------|------|----------|-------------|
| event_id | BIGINT | No | Primary key |
| user_id | UUID | No | User reference |
| work_date | DATE | No | Attendance date |
| event_type | TEXT | No | Attendance event type |
| event_at | TIMESTAMPTZ | No | Attendance timestamp |
| created_at | TIMESTAMPTZ | No | Creation timestamp |

Relationships:

- References `users`

Supported event types:

- CLOCK_IN
- CLOCK_OUT
- GO_OUT
- RETURN

Indexes:

- user_id
- work_date
- event_at

Rules:

- Attendance history must remain immutable.
- Events must never be updated.
- Events must never be deleted.

---

### 4.2 current_attendance_status

Purpose:

- Store latest attendance status
- Support fast UI rendering
- Support validation

Characteristics:

- Mutable
- Updated after attendance stamping

Columns:

| Column Name | Type | Nullable | Description |
|-------------|------|----------|-------------|
| user_id | UUID | No | Primary key |
| current_status | TEXT | No | Current attendance state |
| latest_event_at | TIMESTAMPTZ | Yes | Latest event timestamp |
| updated_at | TIMESTAMPTZ | No | Update timestamp |

Relationships:

- References `users`

Supported statuses:

- BEFORE_WORK
- WORKING
- AWAY
- FINISHED

Rules:

- One row per user
- Rebuildable from attendance history
- Not source of truth

---

### 4.3 daily_remarks

Purpose:

- Store one daily comment per user

Columns:

| Column Name | Type | Nullable | Description |
|-------------|------|----------|-------------|
| remark_id | UUID | No | Primary key |
| user_id | UUID | No | User reference |
| work_date | DATE | No | Attendance date |
| remark_text | TEXT | Yes | Daily comment |
| created_at | TIMESTAMPTZ | No | Creation timestamp |
| updated_at | TIMESTAMPTZ | No | Update timestamp |

Relationships:

- References `users`

Constraints:

- One remark per user and work_date

Rules:

- Comment is optional
- Comment may be updated

## 5. Constraints

### User Constraints

Email:

- Must be unique system-wide

User Code:

- Must be unique within group

Every user:

- Must belong to one group

---

### Attendance Constraints

Attendance event history:

- Immutable
- Never updated
- Never deleted

Current attendance status:

- One row per user

Daily remark:

- One row per user and work_date

---

### Group Constraints

At least one administrator:

- Must remain in each group

Time zone:

- Managed at group level

Time granularity:

- Managed at group level

---

## 6. Index Strategy

Recommended indexes:

### users

Indexes:

- email
- group_id
- user_code

---

### attendance_events

Indexes:

- user_id
- work_date
- event_at

Composite index:

- (user_id, work_date)

Purpose:

- Daily attendance lookup
- Monthly aggregation
- Current status rebuild

---

### current_attendance_status

Indexes:

- user_id
- current_status

Purpose:

- Fast attendance screen rendering
- Fast validation

---

### daily_remarks

Composite index:

- (user_id, work_date)

Purpose:

- Daily comment lookup

---

## 7. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-05-04 | Initial version |
