# Attendance Events API

## Overview

勤怠イベント履歴を取得するAPI。

このAPIは、指定されたユーザーと勤務日の打刻イベントを返し、勤怠一覧・勤怠詳細表示に使用する。

---

## Endpoint

```http
GET /api/attendance/events?user_id=00000000-0000-0000-0000-000000000000&work_date=2026-05-11
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
| user_id | UUID string | Yes | 履歴取得対象ユーザーID |
| work_date | Date string | Yes | 勤務日。形式は YYYY-MM-DD |

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
  "events": [
    {
      "event_id": 1,
      "event_type": "CLOCK_IN",
      "event_at": "2026-05-11T09:00:00Z"
    },
    {
      "event_id": 2,
      "event_type": "GO_OUT",
      "event_at": "2026-05-11T12:00:00Z"
    },
    {
      "event_id": 3,
      "event_type": "RETURN",
      "event_at": "2026-05-11T13:00:00Z"
    },
    {
      "event_id": 4,
      "event_type": "CLOCK_OUT",
      "event_at": "2026-05-11T18:00:00Z"
    }
  ]
}
```

### Event Type Values

| Value | Display Label |
|---|---|
| CLOCK_IN | 出勤 |
| CLOCK_OUT | 退勤 |
| GO_OUT | 外出 |
| RETURN | 戻り |

---

## Validation Rules

- user_id is required
- user_id must be a valid UUID string
- work_date is required
- work_date must be a valid date string in YYYY-MM-DD format
- 対象ユーザーが存在しない場合はエラー

---

## DB Behavior

MVPではDB登録前に、まずJSON APIとして実装する。

将来的には以下を行う。

1. users を参照して対象ユーザーの存在を確認
2. attendance_events から user_id と work_date に一致するイベントを取得
3. event_at の昇順でイベントを返す

---

## Notes

- attendance_events は immutable な打刻履歴
- attendance_events は勤怠履歴の source of truth
- current_attendance_status はこのAPIの主な参照先ではない
- work_date は「どの日の勤務として扱うか」を表す
- MVPでは単日単位の履歴取得を対象とし、月次集計APIは別途検討する
