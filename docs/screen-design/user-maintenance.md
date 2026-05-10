# Rich Time Card - User Maintenance Screen

## 1. Overview

This document defines user maintenance screen specification.

The user maintenance screen is used to manage users within the same group.

Only group administrators may access this screen.

The screen supports:

- User list viewing
- User registration
- User update
- User deletion

---

## 2. Purpose

The user maintenance screen provides:

- User management
- Group member management
- User information update
- User authority management

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

### User Search Area

Input fields:

- User Name
- User ID

Buttons:

- Search
- Clear

---

### User List Area

Displayed columns:

- User Name
- User ID
- Email Address
- Administrator Flag
- Status

Actions:

- Edit
- Delete

---

### Action Area

Buttons:

- Add User

## 4. User Management Rules

### User Scope

Administrators may manage:

- Users within the same group only

Cross-group access is prohibited.

---

### Add User

When Add User is selected:

Transition:

User Maintenance Screen  
↓  
User Registration Screen

New users shall belong to:

- Current administrator group

The administrator shall not create another group.

---

### Update User

Editable fields:

- First Name
- Last Name
- User ID
- Email Address
- Administrator Flag

Non-editable fields:

- User UUID
- Group UUID

---

### Delete User

User deletion shall be supported.

Rules:

- Deleted user shall no longer log in.
- Administrator deletion may be restricted.
- Self-deletion is prohibited.

Deletion policy:

- Soft delete is preferred.

---

### Administrator Authority

Administrators may:

- Add users
- Update users
- Delete users
- Grant administrator role

Normal users may not access this screen.

---

## 5. Validation Rules

### Required Fields

Required:

- First Name
- Last Name
- Email Address

---

### User ID Rules

User ID:

- Optional
- Must be unique within group

Rejected:

- Duplicate User ID

---

### Email Rules

Email Address:

- Required
- Must be unique system-wide

Rejected:

- Duplicate email
- Invalid email format

---

### Administrator Rules

At least one administrator shall remain in the group.

Rejected cases:

- Removing last administrator
- Self-demotion of last administrator

---

### Error Handling

Examples:

- User already exists
- Duplicate User ID
- Invalid email format
- Permission denied

## 6. Screen Behavior

### Initial Screen Load

When the screen opens:

1. Load user list
2. Display search conditions
3. Display action buttons

The default scope shall be:

- Current group users only

---

### Search Flow

Search behavior:

1. User enters search conditions
2. User presses Search button
3. User list is filtered
4. Screen refreshes

Search conditions:

- User Name
- User ID

---

### Add User Flow

Add user behavior:

1. Administrator presses Add User
2. User Registration Screen opens
3. User is registered
4. User list refreshes

---

### Update User Flow

Update behavior:

1. Administrator selects Edit
2. User information screen opens
3. User data is updated
4. User list refreshes

---

### Delete User Flow

Delete behavior:

1. Administrator selects Delete
2. Confirmation dialog appears
3. Deletion is executed
4. User list refreshes

---

### Responsive Behavior

The user maintenance screen shall support:

- Desktop browser

Smartphone support is optional.

---

## 7. Future Scope

Planned future enhancements:

- User bulk import
- User CSV upload
- User export
- Advanced search filter

---

## 8. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-05-04 | Initial version |
