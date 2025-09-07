SELECT
    u.user_id,
    u.personalisation,
    u.contact_email,
    u.contact_name,
    u.created_at,
    u.updated_at,
    u.timezone,
    to_json(du.*) AS discord_user
FROM
    users u
    LEFT JOIN discord_users du ON u.user_id = du.user_id
WHERE
    u.user_id = $1