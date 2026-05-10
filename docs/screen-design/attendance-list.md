# Rich Time Card - Attendance List Screen

## 1. Overview

This document defines attendance list screen specification.

The attendance list screen displays attendance history and monthly aggregation.

Users may review attendance records and working hours.

The screen supports:

- Daily attendance history
- Monthly attendance aggregation
- Attendance detail viewing

---

## 2. Purpose

The attendance list screen provides:

- Attendance history viewing
- Monthly attendance aggregation
- Daily attendance status review
- Working hour confirmation

The screen shall support both:

- PC browser
- Smartphone browser

---

## 3. Display Components

### Header Area

Displayed information:

- Group Name
- User Name
- Aggregation Period

---

### Search Area

Input fields:

- Aggregation Month

Buttons:

- Search

The aggregation period shall follow:

Closing Day + next day → next closing day

Example:

Closing Day: 5th

Aggregation Period:

2026/05/06 ～ 2026/06/05

---

### Attendance List Area

Displayed columns:

- Date
- Day of Week
- Clock In Time
- Clock Out Time
- Go Out Time
- Return Time
- Working Hours
- Attendance Status
- Daily Comment

Displayed attendance status:

- 出勤前
- 勤務中
- 外出中
- 終業

## 4. Aggregation Rules

### Monthly Aggregation

Attendance data shall be aggregated monthly.

Aggregation period:

Closing Day + next day → next closing day

Example:

Closing Day: 5th

Aggregation Period:

2026/05/06 ～ 2026/06/05

---

### Working Hours Calculation

Working hours shall be calculated using:

- Clock In Time
- Clock Out Time
- Go Out Duration
- Return Time
- Lunch Break Time

Calculation shall follow:

Group Time Granularity

Examples:

- 1 minute
- 10 minutes

---

### Attendance Status Display

Attendance status shall be displayed per day.

Supported statuses:

- BEFORE_WORK
- WORKING
- AWAY
- FINISHED

Displayed labels:

- 出勤前
- 勤務中
- 外出中
- 終業

---

### Monthly Summary Area

Displayed summary:

- Total Working Days
- Total Working Hours
- Total Go Out Count
- Total Go Out Duration

The summary shall refresh automatically.

---

## 5. Attendance Detail Rules

### Daily Attendance Information

Daily attendance shall display:

- Clock In Time
- Clock Out Time
- Go Out Time
- Return Time
- Working Hours
- Daily Comment

---

### Missing Attendance Data

When attendance data is missing:

Display:

- Empty value

Examples:

- No clock-in
- No clock-out
- No go-out

---

### Comment Display

Daily comment:

- One comment per user per day
- Editable
- Optional

Stored in:

- `daily_remarks`

---

### Attendance Correction

Attendance correction is out of scope.

Future support is TBD.

## 6. Screen Behavior

### Initial Screen Load

When the screen opens:

1. Load default aggregation month
2. Load attendance history
3. Display monthly summary
4. Display attendance list

The default aggregation month shall be:

- Current aggregation period

---

### Search Flow

Search behavior:

1. User selects aggregation month
2. User presses Search button
3. Attendance history is loaded
4. Monthly aggregation is recalculated
5. Screen refreshes

---

### Screen Refresh

The screen shall refresh:

- Attendance list
- Monthly summary

After search execution.

---

### Error Handling

When attendance data cannot be loaded:

Display:

- Error message

Examples:

- Failed to load attendance history
- Invalid aggregation period

---

### Responsive Behavior

The attendance list screen shall support:

- Desktop browser
- Smartphone browser

For smartphone:

- Vertical layout
- Scrollable attendance table
- Mobile-friendly summary display

---

## 7. Future Scope

Planned future enhancements:

- CSV export
- Excel export
- Google Spreadsheet export
- Attendance correction
- Filtering support

---

## 8. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-05-04 | Initial version |
