# Rich Time Card - User Registration Screen

## 1. Overview

This document defines user registration screen specification.

The user registration screen is used to register a new user.

The screen supports:

- Individual registration
- Group administrator registration

When administrator mode is enabled, group configuration is also supported.

---

## 2. Purpose

The user registration screen provides:

- User profile registration
- Authentication information registration
- Initial group creation
- Group administrator registration

The screen shall support both:

- PC browser
- Smartphone browser

---

## 3. Display Components

### User Information Area

Input fields:

- First Name
- Last Name
- User ID (optional)
- Email Address
- Password

---

### Administrator Setting Area

Input field:

- isAdmin

When isAdmin is enabled:

Additional group settings shall be displayed.

---

### Group Setting Area

Displayed only when:

- isAdmin = true

Input fields:

- Group Name
- Time Zone
- Closing Day
- Time Granularity
- Work Start Time
- Work End Time
- Lunch Break Start Time
- Lunch Break End Time

---

### Action Area

Buttons:

- Register
- Cancel

## 4. Registration Rules

### Individual Registration

Normal users shall register:

- User profile
- Authentication information

Normal users shall belong to a group.

---

### Administrator Registration

When isAdmin is enabled:

The user becomes:

- Group Administrator

The following group configuration is required:

- Group Name
- Time Zone
- Closing Day
- Time Granularity
- Work Start Time
- Work End Time
- Lunch Break Start Time
- Lunch Break End Time

The system shall create:

1. New Group
2. Administrator User
3. User-to-Group relation

---

### Registration Completion

When registration succeeds:

Transition:

User Registration Screen  
↓  
Authentication Screen

The user may log in after registration.

---

## 5. Validation Rules

### Required Fields

Required:

- First Name
- Last Name
- Email Address
- Password

When isAdmin = true:

Additional required fields:

- Group Name
- Time Zone
- Closing Day
- Time Granularity

---

### User ID Rules

User ID:

- Optional
- Must be unique within group

---

### Email Rules

Email Address:

- Required
- Must be unique system-wide

Rejected cases:

- Duplicate email address
- Invalid email format

---

### Password Rules

Password:

- Required

Future password policy is TBD.

---

### Error Handling

Validation error examples:

- Email already exists
- Invalid email format
- Required field missing
- Duplicate User ID

## 6. Screen Behavior

### Initial Screen Load

When the screen opens:

1. Display registration form
2. Hide administrator-only fields
3. Set default values

---

### isAdmin Toggle Behavior

When isAdmin = true:

The following fields shall become visible:

- Group Name
- Time Zone
- Closing Day
- Time Granularity
- Work Start Time
- Work End Time
- Lunch Break Start Time
- Lunch Break End Time

When isAdmin = false:

Administrator-only fields shall be hidden.

---

### Registration Flow

Registration behavior:

1. User enters information
2. User presses Register button
3. Validation is executed
4. Registration API is called
5. User and group are created
6. Registration completes

Success:

Transition to Authentication Screen.

Failure:

Display validation error.

---

### Responsive Behavior

The user registration screen shall support:

- Desktop browser
- Smartphone browser

For smartphone:

- Vertical layout
- Touch-friendly inputs
- Large buttons

---

## 7. Future Scope

Planned future enhancements:

- Invitation-based group join
- Group invitation code
- Email verification
- Two-factor authentication (2FA)

---

## 8. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-05-04 | Initial version |
