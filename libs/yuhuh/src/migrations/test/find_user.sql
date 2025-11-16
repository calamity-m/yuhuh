-- Create users for read_food_entries
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
        'personalized',
        'bobat@example.com',
        'Bobat',
        '1999-09-09T09:09:09Z',
        '2000-09-09T09:09:09Z',
        'UTC'
    );

INSERT INTO
	discord_users (discord_id, username, user_id)
VALUES
	(
		100,
		'alicediscord',
		'11111111-1111-1111-1111-111111111111'::uuid
	);