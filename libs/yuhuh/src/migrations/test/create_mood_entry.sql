-- Create users for create_mood_entry
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