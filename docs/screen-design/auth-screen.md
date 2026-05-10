# Rich Time Card - Authentication Screen

## 1. Overview

This document defines authentication screen specification.

The authentication screen is the entry point of Rich Time Card.

Users authenticate from this screen before accessing the application.

Supported authentication methods:

- User ID
- Email Address

---

## 2. Purpose

The authentication screen provides:

- User login
- Access to user registration
- Password reset entry

The screen shall provide a simple and intuitive user experience.

The screen shall support both:

- PC browser
- Smartphone browser

---

## 3. Display Components

### Header Area

Displayed information:

- Application Name

---

### Login Area

Input fields:

- User ID or Email Address
- Password

Buttons:

- Login

Links:

- User Registration
- Forgot Password

---

### Validation Message Area

The screen shall display validation messages.

Examples:

- Invalid credentials
- User not found
- Password required

## 4. Authentication Rules

### Login Authentication

Authentication shall support:

- User ID
- Email Address

The following credentials are required:

- User ID or Email Address
- Password

Authentication priority:

1. Match User ID
2. Match Email Address

---

### Successful Login

When authentication succeeds:

Transition:

Authentication Screen  
↓  
Attendance Screen

The user session shall start.

---

### Failed Login

When authentication fails:

- Login shall be rejected.
- Error message shall be displayed.
- Input values shall remain.

Examples:

- Invalid credentials
- User not found
- Incorrect password

---

### New User Registration

When user registration is required:

Transition:

Authentication Screen  
↓  
User Registration Screen

User registration may also be supported inline.

---

### Password Reset

When password reset is requested:

Transition:

Authentication Screen  
↓  
Password Reset Screen

Only registered email addresses may request password reset.

---

## 5. Screen Behavior

### Initial Screen Load

When the screen opens:

1. Display login form
2. Focus first input field
3. Clear password field

---

### Login Flow

Login behavior:

1. User enters credentials
2. User presses Login button
3. Authentication API is called
4. Credentials are validated
5. Success or error is returned

Success:

Transition to Attendance Screen.

Failure:

Display validation error.

---

### Validation Rules

Required:

- User ID or Email Address
- Password

Rejected cases:

- Empty credentials
- Invalid credentials
- Unknown user

---

### Responsive Behavior

The authentication screen shall support:

- Desktop browser
- Smartphone browser

For smartphone:

- Vertical layout
- Large input fields
- Touch-friendly buttons

## 6. Future Scope

Planned future enhancements:

- Two-factor authentication (2FA)
- Social login support
- Biometric authentication
- Login notification

---

## 7. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-05-04 | Initial version |
