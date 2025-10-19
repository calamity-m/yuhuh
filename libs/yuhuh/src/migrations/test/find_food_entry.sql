-- Create users for find_food_entry
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

-- Create food entries
--
-- Alice should have 200 calories, 15 carbs/fats/proteins and 
-- three food entries.
--
-- Bobat should have one food entry.
INSERT INTO
    food_records (
        food_record_id,
        user_id,
        created_at,
        description,
        calories,
        carbs,
        protein,
        fats,
        micronutrients
    )
VALUES
    (
        '11111111-1111-1111-1111-111111111111'::uuid,
        '11111111-1111-1111-1111-111111111111'::uuid,
        now() - interval '1 day',
        'burger',
        100.0::real,
        5.0::real,
        5.0::real,
        5.0::real,
        '{}'::jsonb
    ),
    (
        '11111111-1111-1111-1111-222222222222'::uuid,
        '11111111-1111-1111-1111-111111111111'::uuid,
        now() + interval '1 day',
        'burger two',
        NULL,
        NULL,
        NULL,
        NULL,
        '{}'::jsonb
    ),
    (
        '11111111-1111-1111-1111-333333333333'::uuid,
        '11111111-1111-1111-1111-111111111111'::uuid,
        now() - interval '5 days',
        'burger three',
        100.0::real,
        5.0::real,
        5.0::real,
        5.0::real,
        '{}'::jsonb
    ),
    (
        '22222222-2222-2222-2222-222222222222'::uuid,
        '22222222-2222-2222-2222-222222222222'::uuid,
        now(),
        'bobats burger',
        100.0::real,
        5.0::real,
        5.0::real,
        5.0::real,
        '{}'::jsonb
    );