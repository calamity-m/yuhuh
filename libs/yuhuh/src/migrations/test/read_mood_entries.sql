-- Create users for read_mood_entries
INSERT INTO
    users (
        user_id,
        personalisation,
        contact_email,
        contact_name,
        created_at,
        updated_at,
        timezone
    )
VALUES
    (
        '11111111-1111-1111-1111-111111111111'::uuid,
        NULL,
        'alice@example.com',
        'Alice',
        now() - interval '1 day',
        now(),
        'UTC'
    ),
    (
        '22222222-2222-2222-2222-222222222222'::uuid,
        NULL,
        'bobat@example.com',
        'Bobat',
        now() - interval '1 day',
        now(),
        'UTC'
    );

-- Create mood entries
--
-- Alice should have two mood entries.
-- One should have nulls where possible,
-- another should have all fields filled
--
-- Bobat should no entries
INSERT INTO
    mood_records (
        mood_record_id,
        user_id,
        created_at,
        updated_at,
        mood,
        energy,
        sleep,
        notes,
        logged_at
    )
VALUES
    (
        '11111111-1111-1111-1111-111111111111'::uuid,
        '11111111-1111-1111-1111-111111111111'::uuid,
        now() - interval '5 day',
        null,
        0::smallint,
        1::smallint,
        2::smallint,
        null,
        now() - interval '5 day'
    ),
    (
        '11111111-1111-1111-1111-222222222222'::uuid,
        '11111111-1111-1111-1111-111111111111'::uuid,
        now() + interval '5 day',
        now(),
        10::smallint,
        10::smallint,
        null,
        'alices mood thoughts',
        now() + interval '5 day'
    );