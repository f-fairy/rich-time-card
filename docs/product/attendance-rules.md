# Rich Time Card - Attendance Rules

## 1. Overview

This document defines attendance stamping rules and state transitions.

Attendance stamping behavior must always follow these rules.

The system uses:

- Immutable attendance event history
- Mutable current attendance status

Attendance history is recorded in:

- `attendance_events`

Current attendance state is managed in:

- `current_attendance_status`

---

## 2. Attendance Stamp Types

The system supports the following attendance types.

| Internal Value | Display Label |
|----------------|----------------|
| CLOCK_IN | 出勤 |
| CLOCK_OUT | 退勤 |
| GO_OUT | 外出 |
| RETURN | 戻り |

---

## 3. Attendance Status

The system manages user attendance status.

| Internal Value | Display Label |
|----------------|----------------|
| BEFORE_WORK | 出勤前 |
| WORKING | 勤務中 |
| AWAY | 外出中 |
| FINISHED | 終業 |

---

## 4. State Transition Rules

### Overview

Attendance stamping follows strict state transition rules.

Invalid transitions must be rejected.

---

### State Transition Matrix

| Current Status | Action | Allowed | Next Status |
|----------------|--------|----------|--------------|
| BEFORE_WORK | CLOCK_IN | Yes | WORKING |
| BEFORE_WORK | CLOCK_OUT | No | - |
| BEFORE_WORK | GO_OUT | No | - |
| BEFORE_WORK | RETURN | No | - |
| WORKING | CLOCK_IN | No | - |
| WORKING | CLOCK_OUT | Yes | FINISHED |
| WORKING | GO_OUT | Yes | AWAY |
| WORKING | RETURN | No | - |
| AWAY | CLOCK_IN | No | - |
| AWAY | CLOCK_OUT | Yes | FINISHED |
| AWAY | GO_OUT | No | - |
| AWAY | RETURN | Yes | WORKING |
| FINISHED | CLOCK_IN | No | - |
| FINISHED | CLOCK_OUT | No | - |
| FINISHED | GO_OUT | No | - |
| FINISHED | RETURN | No | - |

---

## 5. Detailed Rules

### 5.1 Clock In

Action:

- `CLOCK_IN`

Allowed State:

- `BEFORE_WORK`

Next State:

- `WORKING`

Validation Rules:

- Clock-in during `WORKING` is prohibited.
- Clock-in during `AWAY` is prohibited.
- Clock-in during `FINISHED` is prohibited.

Cancellation Rule:

- Clock-in cancellation may be allowed while in `WORKING`.

---

### 5.2 Clock Out

Action:

- `CLOCK_OUT`

Allowed States:

- `WORKING`
- `AWAY`

Next State:

- `FINISHED`

Validation Rules:

- Clock-out before work is prohibited.

Special Rule:

If a user performs clock-out during `AWAY` state:

- `GO_OUT` shall be invalidated.

Example:

```text
CLOCK_IN
↓
GO_OUT
↓
CLOCK_OUT

GO_OUT becomes invalid.

### 5.3 Go Out

Action:

- `GO_OUT`

Allowed State:

- `WORKING`

Next State:

- `AWAY`

Validation Rules:

- Go-out before work is prohibited.
- Consecutive go-out is prohibited.
- Go-out after finishing work is prohibited.

---

### 5.4 Return

Action:

- `RETURN`

Allowed State:

- `AWAY`

Next State:

- `WORKING`

Validation Rules:

- Return before work is prohibited.
- Return while already working is prohibited.
- Return after finishing work is prohibited.

---

## 6. Daily Comment

The system supports one comment per user per day.

Rules:

- One record per user and attendance date.
- Comment is optional.
- Comment may be updated.

Storage:

- `daily_remarks`

---

## 7. Working Date Rules

Attendance is handled on daily boundaries.

Rules:

- Overnight work is not supported.
- Date switching determines attendance date.
- Cross-date attendance calculation is out of scope.

Example:

```text
2026-05-10 23:59 → Day A
2026-05-11 00:00 → Day B

## 8. Attendance Event Design

Attendance history is immutable.

Rules:

- Attendance events must never be updated.
- Attendance events must never be deleted.
- Corrections are represented as additional events.

Examples:

CLOCK_IN  
GO_OUT  
RETURN  
CLOCK_OUT

Storage Table:

- `attendance_events`

---

## 9. Current Attendance Status Design

Current attendance status is mutable.

Purpose:

- Fast lookup of current user status
- Efficient validation
- Efficient UI display

Example:

User opens attendance screen  
↓  
Read current_attendance_status  
↓  
Display: WORKING

Storage Table:

- `current_attendance_status`

---

## 10. Future Scope

Planned future enhancements:

- Attendance location recording
- GPS validation
- Shift work support
- Overnight work support

---

## 11. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-05-04 | Initial version |
