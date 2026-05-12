-- Rich Time Card PostgreSQL master data seed
--
-- Run this after docs/database/ddl-postgresql.sql.
-- Fixed UUIDs keep this seed repeatable across local and shared environments.

-- =========================================================
-- Time zone master
-- =========================================================

INSERT INTO time_zones (timezone_id, name, offset_minutes) VALUES
    ('11111111-1111-1111-1111-111111111111', 'UTC', 0),
    ('11111111-1111-1111-1111-111111111112', 'Asia/Tokyo', 540),
    ('11111111-1111-1111-1111-111111111113', 'America/New_York', -300)
ON CONFLICT (timezone_id) DO UPDATE SET
    name = EXCLUDED.name,
    offset_minutes = EXCLUDED.offset_minutes;

-- =========================================================
-- Time grain master
-- =========================================================

INSERT INTO time_grains (grain_id, grain_minutes) VALUES
    ('22222222-2222-2222-2222-222222222221', 1),
    ('22222222-2222-2222-2222-222222222222', 10)
ON CONFLICT (grain_id) DO UPDATE SET
    grain_minutes = EXCLUDED.grain_minutes;

-- =========================================================
-- Time conversion master
-- =========================================================

INSERT INTO time_conversions (conversion_id, grain_id, actual_minutes, decimal_value) VALUES
    ('33333333-3333-3333-3333-333333333301', '22222222-2222-2222-2222-222222222221', 1, 0.02),
    ('33333333-3333-3333-3333-333333333302', '22222222-2222-2222-2222-222222222221', 30, 0.50),
    ('33333333-3333-3333-3333-333333333303', '22222222-2222-2222-2222-222222222221', 60, 1.00),
    ('33333333-3333-3333-3333-333333333310', '22222222-2222-2222-2222-222222222222', 10, 0.17),
    ('33333333-3333-3333-3333-333333333311', '22222222-2222-2222-2222-222222222222', 20, 0.33),
    ('33333333-3333-3333-3333-333333333312', '22222222-2222-2222-2222-222222222222', 30, 0.50),
    ('33333333-3333-3333-3333-333333333313', '22222222-2222-2222-2222-222222222222', 40, 0.67),
    ('33333333-3333-3333-3333-333333333314', '22222222-2222-2222-2222-222222222222', 50, 0.83),
    ('33333333-3333-3333-3333-333333333315', '22222222-2222-2222-2222-222222222222', 60, 1.00)
ON CONFLICT (conversion_id) DO UPDATE SET
    grain_id = EXCLUDED.grain_id,
    actual_minutes = EXCLUDED.actual_minutes,
    decimal_value = EXCLUDED.decimal_value;
