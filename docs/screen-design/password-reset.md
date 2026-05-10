# Rich Time Card - Password Reset Screen

## 1. Overview

This document defines password reset screen specification.

The password reset screen is used when a user forgets password.

Password reset uses registered email address verification.

Only users who can receive email may reset password.

---

## 2. Purpose

The password reset screen provides:

- Password reset request
- Email verification
- Initial password reissue

The screen shall support both:

- PC browser
- Smartphone browser

---

## 3. Display Components

### Header Area

Displayed information:

- Application Name

---

### Reset Request Area

Input fields:

- Email Address

Buttons:

- Reset Password
- Cancel

---

### Validation Message Area

The screen shall display validation messages.

Examples:

- Email address required
- User not found
- Password reset email sent

---

## 4. Password Reset Rules

### Reset Method

Password reset shall use:

- Registered Email Address

Only registered users may request password reset.

---

### Reset Flow

Password reset flow:

1. User enters email address
2. Reset request is submitted
3. Email address is validated
4. Initial password is generated
5. Password reset email is sent

---

### Successful Reset

When password reset succeeds:

Transition:

Password Reset Screen  
↓  
Authentication Screen

User logs in using:

- Initial password

User shall change password after login.

---

### Failed Reset

Rejected cases:

- Unknown email address
- Invalid email format
- Email delivery failure

Examples:

- User not found
- Invalid email address

---

## 5. Validation Rules

Required:

- Email Address

Email Address:

- Must exist in system
- Must be valid format

Rejected:

- Empty email
- Invalid format
- Unknown email

---

## 6. Screen Behavior

### Initial Screen Load

When the screen opens:

1. Display reset form
2. Focus email input field

---

### Reset Flow

Reset behavior:

1. User enters email address
2. User presses Reset Password
3. Validation is executed
4. Reset API is called
5. Initial password is issued
6. Email is sent

Success:

Transition to Authentication Screen.

Failure:

Display validation error.

---

### Responsive Behavior

The password reset screen shall support:

- Desktop browser
- Smartphone browser

For smartphone:

- Vertical layout
- Touch-friendly inputs
- Large buttons

---

## 7. Future Scope

Planned future enhancements:

- Reset link authentication
- Temporary token authentication
- Two-factor authentication (2FA)

---

## 8. Revision History

| Version | Date | Description |
|----------|------|-------------|
| 1.0 | 2026-05-04 | Initial version |
