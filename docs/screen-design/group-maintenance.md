# Rich Time Card - Group Maintenance Screen

## 1. Overview

This document defines group maintenance screen specification.

The group maintenance screen is used to manage group configuration.

Only group administrators may access this screen.

The screen supports:

- Group information update
- Working condition configuration
- Time zone management
- Attendance aggregation settings

---

## 2. Purpose

The group maintenance screen provides:

- Group configuration management
- Working hours settings
- Attendance calculation settings
- Time granularity configuration

The screen shall support:

- PC browser

Smartphone support is optional.

---

## 3. Display Components

### Header Area

Displayed information:

- Group Name
- Screen Name

---

### Group Information Area

Input fields:

- Group Name
- Time Zone
- Closing Day

---

### Working Time Area

Input fields:

- Work Start Time
- Work End Time
- Lunch Break Start Time
- Lunch Break End Time

---

### Attendance Calculation Area

Input fields:

- Time Granularity

Examples:

- 1 minute
- 10 minutes

---

### Action Area

Buttons:

- Save
- Cancel

## 4. Group Management Rules

### Access Restriction

Only group administrators may access this screen.

Normal users:

- Cannot access
- Cannot update group settings

---

### Editable Group Information

Editable fields:

- Group Name
- Time Zone
- Closing Day
- Work Start Time
- Work End Time
- Lunch Break Start Time
- Lunch Break End Time
- Time Granularity

Non-editable fields:

- Group UUID

---

### Time Zone Rules

Time Zone:

- Configured at group level
- Shared by all group users

Attendance calculation shall follow:

- Group time zone

Changing time zone after operation starts is discouraged.

---

### Closing Day Rules

Closing Day:

- Defines monthly aggregation period

Example:

Closing Day: 5th

Aggregation Period:

2026/05/06 ～ 2026/06/05

---

### Time Granularity Rules

Supported granularity examples:

- 1 minute
- 10 minutes

Attendance calculation shall follow:

- Group configuration

---

### Save Rules

When Save is executed:

1. Validation is executed
2. Group settings are updated
3. Changes become effective

Updated settings affect:

- Attendance aggregation
- Working hour calculation
- Attendance display

---

## 5. Validation Rules

### Required Fields

Required:

- Group Name
- Time Zone
- Closing Day
- Time Granularity

---

### Closing Day Rules

Closing Day:

- Must be valid calendar date

Examples:

- 1
- 5
- 15
- 31

Rejected:

- Invalid day value

---

### Time Rules

Work Start Time:

- Must be earlier than Work End Time

Lunch Break:

- Must exist within working hours

Rejected:

- Invalid time range
- Overlapping time settings

---

### Error Handling

Examples:

- Required field missing
- Invalid closing day
- Invalid time setting
- Permission denied

## 6. Screen Behavior

### Initial Screen Load

When the screen opens:

1. Load current group settings
2. Display group information
3. Display attendance calculation settings

The displayed settings shall reflect:

- Current group configuration

---

### Save Flow

Save behavior:

1. Administrator updates settings
2. User presses Save button
3. Validation is executed
4. Group settings are updated
5. Screen refreshes

Success:

Display success message.

Failure:

Display validation error.

---

### Cancel Flow

When Cancel is selected:

- Unsaved changes are discarded.

Transition:

Group Maintenance Screen  
↓  
Previous Screen

---

### Responsive Behavior

The group maintenance screen shall support:

- Desktop browser

Smartphone support is optional.

---

## 7. Future Scope

Planned future enhancements:

- Multiple work schedule settings
- Holiday configuration
- Shift work support
- Group-level export settings

---

## 8. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-05-04 | Initial version |
