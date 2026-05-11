# Attendance Check-out API

## Overview

退勤打刻を行うAPI。

このAPIは、指定されたユーザーを「終業」状態にし、打刻イベントとして退勤履歴を記録する。

---

## Endpoint

```http
POST /api/attendance/checkout
```

---

## Authentication

T.B.D.

現時点のMVPでは認証なしで実装する。  
将来的にはJWT認証を使用する。

---

## Request

```json
{
  "user_id": "00000000-0000-0000-0000-000000000000"
}
```

### Fields

| Field | Type | Required | Description |
|---|---|---:|---|
| user_id | UUID string | Yes | 打刻対象ユーザーID |

---

## Response

### Success Response

```http
200 OK
```

```json
{
  "result": "success",
  "status": "FINISHED"
}
```

---

## Validation Rules

- user_id is required
- 勤務中または外出中状態のユーザーのみ退勤可能
- 出勤前の場合はエラー
- 終業済みの場合はエラー

---

## DB Behavior

MVPではDB登録前に、まずJSON APIとして実装する。

将来的には以下を行う。

1. current_attendance_status を参照して現在状態を確認
2. attendance_events に CLOCK_OUT を INSERT
3. current_attendance_status を FINISHED に UPSERT

外出中に退勤した場合、勤務時間計算では未復帰の GO_OUT を無効として扱う。

---

## Notes

- attendance_events は immutable な打刻履歴
- current_attendance_status は mutable な現在状態
- work_date は「どの日の勤務として扱うか」を表す
- 退勤後の追加打刻はMVPでは許可しない
