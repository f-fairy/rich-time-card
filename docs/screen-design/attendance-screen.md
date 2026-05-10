# Rich Time Card - Attendance Screen

## 1. Overview

This document defines attendance screen specification.

The attendance screen is the primary operational screen of Rich Time Card.

Users perform attendance stamping from this screen.

Supported attendance actions:

- Clock In
- Clock Out
- Go Out
- Return

The screen behavior changes according to current attendance status.

---

## 2. Purpose

The attendance screen provides:

- Current attendance status
- Attendance stamping
- Current date and time
- User information
- Group information

The screen shall provide a simple and intuitive user experience.

The screen shall support both:

- PC browser
- Smartphone browser

---

## 3. Display Components

### Header Area

Displayed information:

- Group Name
- Application Name

---

### User Information Area

Displayed information:

- User Name
- User ID
- Attendance Status

Displayed attendance status:

- 出勤前
- 勤務中
- 外出中
- 終業

Internal values:

- BEFORE_WORK
- WORKING
- AWAY
- FINISHED

---

### Clock Area

Displayed information:

- Current Date
- Current Time

The current time shall automatically refresh.

---

### Action Button Area

Displayed buttons depend on attendance status.

## 4. State-based Button Display

Displayed action buttons shall change according to attendance status.

### BEFORE_WORK

Displayed buttons:

- 出勤 (Clock In)

Hidden buttons:

- 退勤
- 外出
- 戻り

Allowed action:

- CLOCK_IN

---

### WORKING

Displayed buttons:

- 退勤 (Clock Out)
- 外出 (Go Out)

Hidden buttons:

- 出勤
- 戻り

Allowed actions:

- CLOCK_OUT
- GO_OUT

---

### AWAY

Displayed buttons:

- 戻り (Return)
- 退勤 (Clock Out)

Hidden buttons:

- 出勤
- 外出

Allowed actions:

- RETURN
- CLOCK_OUT

---

### FINISHED

Displayed buttons:

None

Hidden buttons:

- 出勤
- 退勤
- 外出
- 戻り

Allowed actions:

None

---

## 5. Validation Rules

Attendance action validation shall follow attendance rules.

### Clock In

Allowed:

- BEFORE_WORK

Rejected:

- WORKING
- AWAY
- FINISHED

Error example:

Already working.

---

### Clock Out

Allowed:

- WORKING
- AWAY

Rejected:

- BEFORE_WORK

Error example:

Cannot clock out before work starts.

---

### Go Out

Allowed:

- WORKING

Rejected:

- BEFORE_WORK
- AWAY
- FINISHED

Error example:

Cannot go out now.

---

### Return

Allowed:

- AWAY

Rejected:

- BEFORE_WORK
- WORKING
- FINISHED

Error example:

Cannot return now.

## 6. Screen Behavior

### Initial Screen Load

When the attendance screen is opened:

1. Load current attendance status
2. Display user information
3. Display current date and time
4. Display action buttons based on attendance status

The screen shall always reflect latest attendance state.

---

### Attendance Action Flow

Attendance action flow:

1. User presses action button
2. API request is sent
3. Server validates action
4. Attendance event is recorded
5. Current attendance status is updated
6. Screen refreshes

The screen shall update automatically after successful action.

---

### Error Handling

When an invalid action is attempted:

- Error message shall be displayed.
- Current screen state shall remain unchanged.

Examples:

- Already working
- Cannot clock out before work starts
- Invalid attendance action

---

### Responsive Behavior

The attendance screen shall support:

- Desktop browser
- Smartphone browser

For smartphone:

- Large action buttons
- Mobile-friendly layout
- Touch-friendly operation

---

## 7. Future Scope

Planned future enhancements:

- GPS location display
- Clock-in confirmation dialog
- Widget integration
- Push notification support

---

## 8. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-05-04 | Initial version |
