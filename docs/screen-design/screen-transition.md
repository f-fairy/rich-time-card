# Rich Time Card - Screen Transition

## 1. Overview

This document defines screen transition flow for the Rich Time Card web application.

The system supports:

- Authentication
- User registration
- Attendance stamping
- Attendance history viewing
- Group management
- Password reset

---

## 2. Screen List

| Screen ID | Screen Name | Description |
|------------|-------------|-------------|
| AUTH | Authentication Screen | User login |
| USER_REGISTER | User Registration Screen | User registration |
| PASSWORD_RESET | Password Reset Screen | Reset password |
| ATTENDANCE | Attendance Screen | Attendance stamping |
| ATTENDANCE_LIST | Attendance List Screen | Monthly attendance history |
| USER_MAINTENANCE | User Maintenance Screen | User management |
| GROUP_MAINTENANCE | Group Maintenance Screen | Group management |

---

## 3. Screen Transition Overview

### Authentication Flow

```text
Authentication Screen
        │
        ├── Login Success
        │         ↓
        │   Attendance Screen
        │
        ├── User Not Registered
        │         ↓
        │ User Registration Screen
        │
        └── Forgot Password
                  ↓
         Password Reset Screen

### Main Navigation Flow

Attendance Screen
        │
        ├── Attendance History
        │         ↓
        │ Attendance List Screen
        │
        ├── User Maintenance
        │         ↓
        │ User Maintenance Screen
        │
        └── Group Maintenance
                  ↓
         Group Maintenance Screen

## 4. Authentication Rules

### Login

Authentication shall support:

- User ID
- Email Address

The following credentials are required:

- Email or User ID
- Password

---

### New Registration

If user registration is not completed:

Transition:

Authentication Screen  
↓  
User Registration Screen

User registration may also be supported inside authentication flow.

---

### Password Reset

Password reset flow:

Authentication Screen  
↓  
Password Reset Screen

Password reset requires:

- Registered Email Address

Only users who can receive email may reset password.

## 5. Authorization Rules

### Group Administrator

Group administrators may access:

- User Maintenance Screen
- Group Maintenance Screen

Additional capabilities:

- Register users
- Update group settings
- Group attendance operation

---

### Normal User

Normal users may access:

- Attendance Screen
- Attendance List Screen

Restrictions:

- No access to group maintenance
- No access to user maintenance

## 6. Future Scope

Planned future enhancements:

- Additional admin dashboard
- Notification screen
- Approval workflow

---

## 7. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-05-04 | Initial version |
