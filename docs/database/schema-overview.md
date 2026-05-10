# Rich Time Card - Database Schema Overview

## 1. Overview

This document defines the database schema overview for Rich Time Card.

The database is designed for:

- User management
- Group management
- Time zone management
- Attendance event history
- Current attendance status
- Daily comments
- Attendance aggregation support

The database engine is:

- PostgreSQL

---

## 2. Design Principles

### Source of Truth

Attendance event history shall be the source of truth.

The following table stores immutable attendance history:

- `attendance_events`

---

### Current State

Current attendance status shall be stored separately for efficient lookup.

The following table stores mutable current state:

- `current_attendance_status`

---

### Master Data

The following tables are master data:

- `time_zones`
- `time_grains`
- `time_conversions`

These master tables may be initialized by SQL seed data.

Initial MVP does not require maintenance screens for these master tables.

---

## 3. Main Entity Groups

### User and Group

Primary tables:

- `users`
- `user_groups`

Relationship:

- One group has many users.
- One user belongs to one group.

Rules:

- Email address is unique system-wide.
- User ID is unique within group.
- Every user belongs to a group.

---

### Attendance

Primary tables:

- `attendance_events`
- `current_attendance_status`
- `daily_remarks`

Responsibilities:

- `attendance_events` stores immutable attendance stamp events.
- `current_attendance_status` stores latest attendance status.
- `daily_remarks` stores one daily comment per user and work date.

---

### Time Management

Primary tables:

- `time_zones`
- `time_grains`
- `time_conversions`

Responsibilities:

- `time_zones` defines available time zones.
- `time_grains` defines aggregation granularity.
- `time_conversions` defines minute-to-decimal conversion rules.

---

## 4. Conceptual Relationship

User and group relationship:

User Group  
↓  
Users

Attendance relationship:

Users  
↓  
Attendance Events  
↓  
Current Attendance Status

Daily comment relationship:

Users  
↓  
Daily Remarks

Time management relationship:

User Group  
↓  
Time Zone  
↓  
Time Grain  
↓  
Time Conversion

---

## 5. Table List

| Table Name | Purpose |
|-----------|---------|
| `time_zones` | Time zone master |
| `time_grains` | Time granularity master |
| `time_conversions` | Time conversion master |
| `user_groups` | Group management |
| `users` | User management |
| `attendance_events` | Immutable attendance event history |
| `current_attendance_status` | Mutable current attendance state |
| `daily_remarks` | Daily user comments |

---

## 6. Naming Policy

### Table Names

Table names shall use:

- snake_case
- plural or domain-specific names

Examples:

- `users`
- `user_groups`
- `attendance_events`

---

### ID Columns

Primary key columns shall use:

- UUID for domain entities
- BIGINT identity for event/log style records

Examples:

- `user_id`
- `group_id`
- `event_id`

---

### Timestamp Columns

Timestamp columns shall use:

- `TIMESTAMPTZ`

The system shall store actual timestamps with time zone awareness.

---

## 7. PostgreSQL Policy

The database shall use PostgreSQL-native types where appropriate.

Examples:

- `UUID`
- `TIMESTAMPTZ`
- `BOOLEAN`
- `ENUM`

UUID values shall be generated using:

- `gen_random_uuid()`

Extension:

- `pgcrypto`

---

## 8. Future Scope

Planned future database enhancements:

- Monthly fixed aggregation table
- Attendance correction table
- Audit log table
- Password reset token table
- Email verification token table
- Soft delete support

---

## 9. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-05-04 | Initial version |
