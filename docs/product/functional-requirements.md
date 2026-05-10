# Rich Time Card - Functional Requirements

## 1. Overview

Rich Time Card is a web-based time card application designed for both individual and group attendance management.

The application supports attendance stamping, attendance aggregation, and export functionality.

The system provides both:

- Web Client (PC / Smartphone)
- Desktop Widget Client (Windows / Mac)

The desktop widget provides lightweight attendance stamping, while the web client provides full functionality.

---

## 2. Product Goals

The primary goals of Rich Time Card are:

- Simple and intuitive attendance stamping
- Group-based attendance management
- Time-zone aware attendance control
- Rich and modern UI/UX
- Cross-platform support
- Lightweight desktop attendance widget
- Accurate monthly aggregation

---

## 3. Supported Clients

### 3.1 Web Client

Supported platforms:

- PC browser
- Smartphone browser

The web client provides full functionality.

Capabilities:

- Authentication
- User registration
- Group registration
- Attendance stamping
- Attendance aggregation
- Attendance history viewing
- Export functionality
- Group management

---

### 3.2 Desktop Widget Client

Supported platforms:

- Windows
- Mac

The desktop widget provides lightweight attendance operations.

Capabilities:

- Clock-in
- Clock-out

---

## 4. Functional Requirements

### 4.1 Group Management

The system shall support both:

- Individual use
- Group use

#### Rules

- Group administrators can manage group settings.
- Group administrators can register additional users in the same group.
- Group administrators can perform batch attendance operations for group members.

#### Group Settings

A group shall contain:

- Group Name
- Time Zone
- Closing Day
- Time Granularity
- Working Hours Settings

---

### 4.2 User Management

Users shall be managed using:

- User ID (optional)
- Email Address (required)

#### Authentication Rules

- Email address must be unique system-wide.
- User ID must be unique within the same group.
- Email address is mandatory.

#### User Profile

A user shall contain:

- First Name
- Last Name
- User ID
- Email Address
- Password
- Administrator Flag

#### Authentication Features

The system shall support:

- Login authentication
- Password authentication
- Initial password
- Password reset

#### Future Scope

The following feature is TBD:

- Two-factor authentication (2FA)

---

### 4.3 Time Zone Management

The system shall support time-zone aware attendance control.

#### Rules

- Time zone shall be configured at group level.
- Attendance time management shall follow group time zone.
- Time zone cannot be changed once configured.

---

### 4.4 Working Time Management

Working conditions shall be configurable at group level.

#### Configurable Settings

- Work Start Time
- Work End Time
- Lunch Break Start Time
- Lunch Break End Time
- Time Granularity

#### Time Granularity

The system shall support configurable time aggregation units.

Examples:

- 1 minute
- 10 minutes
- Other configurable units

---

### 4.5 Attendance Stamping

The system shall support attendance stamping.

#### Attendance Types

Supported stamp types:

- CLOCK_IN
- CLOCK_OUT
- GO_OUT
- RETURN

Displayed labels:

- 出勤
- 退勤
- 外出
- 戻り

---

#### Attendance State

The system shall manage attendance state.

Supported states:

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

#### Validation Rules

##### CLOCK_IN

Allowed:

- BEFORE_WORK

Rejected:

- WORKING
- AWAY
- FINISHED

Cancellation:

- Clock-in cancellation is allowed while working.

---

##### CLOCK_OUT

Allowed:

- WORKING
- AWAY

Rejected:

- BEFORE_WORK

Special Rule:

- If clock-out is performed during AWAY state,
  GO_OUT shall be invalidated.

---

##### GO_OUT

Allowed:

- WORKING

Rejected:

- BEFORE_WORK
- AWAY
- FINISHED

---

##### RETURN

Allowed:

- AWAY

Rejected:

- BEFORE_WORK
- WORKING
- FINISHED

---

#### Supplementary Inputs

Attendance stamping may support:

- Late arrival information
- Early leave information
- Daily comment

---

#### Future Scope

The following feature is TBD:

- Location recording at attendance stamping

---

### 4.6 Attendance Aggregation

The system shall automatically aggregate attendance data.

#### Aggregation Period

Aggregation period shall be:

- One month

The month definition shall follow:

Closing day + next day → next closing day

Example:

Closing day: 5th

Aggregation period:

2026/05/06 ～ 2026/06/05

---

#### Aggregation Items

Supported aggregation:

- Working Hours

---

#### Finalization

Aggregated monthly data shall become fixed after finalization.

Finalization shall be automatically executed by calendar schedule.

---

### 4.7 Export Functionality

The system shall support external export of attendance data.

Export period:

- Monthly

Supported formats (T.B.D.):

- CSV
- Excel
- Google Spreadsheet

---

## 5. Business Constraints

### Overnight Work

Overnight work is not supported.

Attendance shall be judged by daily date boundary.

---

### Group Authority

Only group administrators can:

- Update group settings
- Register additional users
- Perform group attendance operations

---

## 6. Future Scope (T.B.D.)

Planned future enhancements:

- Two-factor authentication
- Attendance location recording
- Additional export formats

---

## 7. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-04-29 | Initial version |
| 1.1 | 2026-05-04 | Added Closing Day |
