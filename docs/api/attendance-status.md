# Attendance Status API

## Overview

現在の勤怠状態を取得するAPI。

このAPIは、指定されたユーザーの現在状態を返し、勤怠画面の初期表示およびボタン表示制御に使用する。

---

## Endpoint

```http
GET /api/attendance/status?user_id=00000000-0000-0000-0000-000000000000
```

---

## Authentication

T.B.D.

現時点のMVPでは認証なしで実装する。  
将来的にはJWT認証を使用する。

---

## Request

Query Parameters:

| Field | Type | Required | Description |
|---|---|---:|---|
| user_id | UUID string | Yes | 状態取得対象ユーザーID |

---

## Response

### Success Response

```http
200 OK
```

```json
{
  "user_id": "00000000-0000-0000-0000-000000000000",
  "work_date": "2026-05-11",
  "status": "BEFORE_WORK"
}
```

### Status Values

| Value | Display Label |
|---|---|
| BEFORE_WORK | 出勤前 |
| WORKING | 勤務中 |
| AWAY | 外出中 |
| FINISHED | 終業 |

---

## Validation Rules

- user_id is required
- user_id must be a valid UUID string
- 対象ユーザーが存在しない場合はエラー

---

## DB Behavior

MVPではDB登録前に、まずJSON APIとして実装する。

将来的には以下を行う。

1. users を参照して対象ユーザーの存在を確認
2. current_attendance_status から現在状態を取得
3. current_attendance_status が存在しない場合は BEFORE_WORK として扱う
4. work_date はユーザー所属グループのタイムゾーンに従って決定する

---

## Notes

- current_attendance_status は勤怠画面の高速表示のために使用する
- current_attendance_status は履歴の source of truth ではない
- attendance_events が immutable な source of truth である
- MVPでは夜勤・日跨ぎ勤務は対象外
