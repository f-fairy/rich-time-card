-- Rich Time Card - User Seed Data
-- Safe to run multiple times because fixed UUIDs are used.

-- =========================================================
-- 4. グループ管理
-- =========================================================

INSERT INTO user_groups (group_id, name, closing_day, timezone_id, grain_id, lunch_start_time, lunch_end_time, work_start_time, work_end_time) VALUES
('00000000-0000-0000-0004-000000000001', 'Default Group 31', 31, '00000000-0000-0000-0001-000000000002', '00000000-0000-0000-0002-000000000010', '12:00', '13:00', '09:00', '18:00' )
ON CONFLICT DO NOTHING;

-- =========================================================
-- 5. ユーザー管理
-- =========================================================

INSERT INTO users ( user_id, group_id, login_id, email, password_hash, display_name, is_admin ) VALUES
( '00000000-0000-0000-0005-000000000001', '00000000-0000-0000-0004-000000000001', 'toshi', 'ma.terakura@gmail.com', 'NOT_USED_YET', 'トシ', true ),
( '00000000-0000-0000-0005-000000000002', '00000000-0000-0000-0004-000000000001', 'chappy', 'da.i@me.com', 'NOT_USED_YET', 'チャッピー', true ),
( '00000000-0000-0000-0005-000000000003', '00000000-0000-0000-0004-000000000001', 'codex', 'ha.na@me.com', 'NOT_USED_YET', 'コーちゃん', false )
ON CONFLICT DO NOTHING;
