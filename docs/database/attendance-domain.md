# Rich Time Card - Attendance Domain Design

## 1. Overview

This document defines the attendance domain design for Rich Time Card.

The attendance domain is based on two core concepts:

- Immutable attendance event history
- Mutable current attendance status

This separation improves:

- Readability
- Maintainability
- Auditability
- API implementation clarity

---

## 2. Core Tables

### attendance_events

Purpose:

- Store attendance stamping history
- Preserve immutable event records
- Act as source of truth for attendance history

Characteristics:

- Append-only
- Not updated
- Not deleted

---

### current_attendance_status

Purpose:

- Store latest attendance status per user
- Support fast UI display
- Support validation before stamping

Characteristics:

- Mutable
- Updated by attendance actions
- Rebuildable from attendance_events

---

## 3. Source of Truth

The source of truth is:

- `attendance_events`

All attendance history shall be derived from attendance events.

The following data can be derived from attendance events:

- Clock-in time
- Clock-out time
- Go-out time
- Return time
- Working hours
- Daily attendance history

---

## 4. Current Status as Cache

The current attendance status is stored in:

- `current_attendance_status`

This table is a mutable cache of latest state.

Rules:

- It is updated whenever an attendance action succeeds.
- It is used for fast lookup.
- It may be rebuilt from `attendance_events`.

The system shall not treat current status as historical truth.

---

## 5. Attendance Event Types

Supported event types:

| Event Type | Display Label |
|------------|---------------|
| CLOCK_IN | 出勤 |
| CLOCK_OUT | 退勤 |
| GO_OUT | 外出 |
| RETURN | 戻り |

---

## 6. Attendance Status Types

Supported status types:

| Status Type | Display Label |
|-------------|---------------|
| BEFORE_WORK | 出勤前 |
| WORKING | 勤務中 |
| AWAY | 外出中 |
| FINISHED | 終業 |

---

## 7. Event Processing Flow

When an attendance action is requested:

1. Read current status from `current_attendance_status`
2. Validate requested action
3. Insert new event into `attendance_events`
4. Update `current_attendance_status`
5. Return latest status to client

---

## 8. Clock In Flow

Action:

- CLOCK_IN

Allowed current status:

- BEFORE_WORK

Database behavior:

1. Insert CLOCK_IN event into `attendance_events`
2. Update `current_attendance_status` to WORKING

Next status:

- WORKING

---

## 9. Clock Out Flow

Action:

- CLOCK_OUT

Allowed current status:

- WORKING
- AWAY

Database behavior:

1. Insert CLOCK_OUT event into `attendance_events`
2. Update `current_attendance_status` to FINISHED

Next status:

- FINISHED

Special rule:

If current status is AWAY, GO_OUT shall be invalidated for working hour calculation.

---

## 10. Go Out Flow

Action:

- GO_OUT

Allowed current status:

- WORKING

Database behavior:

1. Insert GO_OUT event into `attendance_events`
2. Update `current_attendance_status` to AWAY

Next status:

- AWAY

---

## 11. Return Flow

Action:

- RETURN

Allowed current status:

- AWAY

Database behavior:

1. Insert RETURN event into `attendance_events`
2. Update `current_attendance_status` to WORKING

Next status:

- WORKING

---

## 12. Work Date

The system uses `work_date` to represent the attendance date.

Purpose:

- Identify which daily attendance row the event belongs to
- Support daily attendance list
- Support monthly aggregation

Rules:

- `work_date` is not necessarily identical to raw timestamp date in future extensions.
- For MVP, `work_date` follows group time zone date.
- Overnight work is out of scope.

---

## 13. Transaction Boundary

Attendance action shall be processed in a single database transaction.

Transaction steps:

1. Validate current status
2. Insert attendance event
3. Update current status
4. Commit transaction

If any step fails:

- Rollback all changes

---

## 14. Rebuild Policy

`current_attendance_status` may be rebuilt from `attendance_events`.

Rebuild use cases:

- Data recovery
- Cache inconsistency
- Debugging
- Migration

Rebuild process:

1. Read latest attendance events per user
2. Derive latest status
3. Replace current status

---

## 15. Future Scope

Planned future enhancements:

- Attendance correction
- Cancellation event
- Audit log
- Multiple go-out intervals
- Overnight work
- Monthly fixed aggregation table

---

## 16. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-05-04 | Initial version |
