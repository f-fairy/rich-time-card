-- Rich Time Card PostgreSQL DDL
--
-- This DDL is based on:
-- - richTimeCardApp_spec1.1.pdf
-- - richTimeCardApp_screen1.0.pdf
-- - original MySQL DDL created in the previous design phase
--
-- Reviewed and converted for PostgreSQL:
-- - UUID primary keys
-- - PostgreSQL naming conventions
-- - user_groups instead of groups
-- - attendance_events as immutable history
-- - current_attendance_status as mutable current state
--
-- This file is treated as the current reviewed schema baseline.

-- =========================================================
-- Rich Time Card - PostgreSQL Schema
-- =========================================================

-- UUID生成用
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- =========================================================
-- ENUM定義
-- =========================================================

CREATE TYPE attendance_event_type AS ENUM (
    'CLOCK_IN',
    'CLOCK_OUT',
    'GO_OUT',
    'RETURN'
);

CREATE TYPE attendance_status_type AS ENUM (
    'BEFORE_WORK',
    'WORKING',
    'OUTSIDE',
    'FINISHED'
);

-- =========================================================
-- 1. タイムゾーンマスタ
-- =========================================================

CREATE TABLE time_zones (
    timezone_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    offset_minutes INT NOT NULL
);

-- =========================================================
-- 2. 時間粒度マスタ
-- =========================================================

CREATE TABLE time_grains (
    grain_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    grain_minutes INT NOT NULL
);

-- =========================================================
-- 3. 換算マスタ
-- =========================================================

CREATE TABLE time_conversions (
    conversion_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    grain_id UUID NOT NULL,
    actual_minutes INT NOT NULL,
    decimal_value DECIMAL(3,2) NOT NULL,

    CONSTRAINT fk_time_conversions_grain
        FOREIGN KEY (grain_id)
        REFERENCES time_grains(grain_id)
);

-- =========================================================
-- 4. グループ管理
-- =========================================================

CREATE TABLE user_groups (
    group_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    closing_day INT NOT NULL DEFAULT 20,
    timezone_id UUID NOT NULL,
    grain_id UUID NOT NULL,
    lunch_start_time TIME,
    lunch_end_time TIME,
    work_start_time TIME,
    work_end_time TIME,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_user_groups_timezone
        FOREIGN KEY (timezone_id)
        REFERENCES time_zones(timezone_id),

    CONSTRAINT fk_user_groups_grain
        FOREIGN KEY (grain_id)
        REFERENCES time_grains(grain_id)
);

-- =========================================================
-- 5. ユーザー管理
-- =========================================================

CREATE TABLE users (
    user_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    group_id UUID NOT NULL,
    login_id VARCHAR(50) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    display_name VARCHAR(256) NOT null,
    is_admin BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_users_group_login
        UNIQUE (group_id, login_id),

    CONSTRAINT fk_users_group
        FOREIGN KEY (group_id)
        REFERENCES user_groups(group_id)
);

-- =========================================================
-- 6. 打刻イベント（immutable）
-- =========================================================

CREATE TABLE attendance_events (
    event_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id UUID NOT NULL,
    -- 「どの日の勤務として扱うか」
    work_date DATE NOT NULL,
    -- 実際の打刻日時
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    event_type attendance_event_type NOT NULL,
    location VARCHAR(255),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_attendance_events_user
        FOREIGN KEY (user_id)
        REFERENCES users(user_id)
);

CREATE INDEX idx_attendance_events_user_date
    ON attendance_events(user_id, work_date);

CREATE INDEX idx_attendance_events_occurred_at
    ON attendance_events(occurred_at);

-- =========================================================
-- 7. 現在勤務状態（mutable）
-- =========================================================

CREATE TABLE current_attendance_status (
    user_id UUID PRIMARY KEY,
    work_date DATE NOT NULL,
    current_status attendance_status_type NOT NULL,
    last_event_id BIGINT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_current_status_user
        FOREIGN KEY (user_id)
        REFERENCES users(user_id),

    CONSTRAINT fk_current_status_event
        FOREIGN KEY (last_event_id)
        REFERENCES attendance_events(event_id)
);

-- =========================================================
-- 8. 日次コメント
-- =========================================================

CREATE TABLE daily_remarks (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id UUID NOT NULL,
    work_date DATE NOT NULL,
    comment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    CONSTRAINT uq_daily_remarks
        UNIQUE (user_id, work_date),

    CONSTRAINT fk_daily_remarks_user
        FOREIGN KEY (user_id)
        REFERENCES users(user_id)
);
