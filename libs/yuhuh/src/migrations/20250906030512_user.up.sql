-- Users that are specific to this app
create table users
(
    -- ID of the user, specific to yuhuh
    user_id         uuid primary key default uuidv7(),

    -- A personalisation we can just easily store here on the user and
    -- provide to the models
    personalisation text,

    contact_email   text,
    contact_name    text,
    
    -- Time the user was generated, irrerespective of any later joins
    -- or additions.
    created_at      timestamptz not null default now(),
    -- Last time the user was updated, pretty self explanatory
    updated_at      timestamptz,
    -- String encoded timezone, as some clients talk to yuhuh in utc we
    -- need to allow them to store, and translate into the desired
    -- user's timezone.
    timezone        text
);

-- Apply our `updated_at` trigger from the first migration
SELECT trigger_updated_at('"users"');

-- Records users who signed on with discord as their driver
create table discord_users
(
    -- ID from discord
    discord_id  bigint,
    -- Name of the discord user
    username    text,
    -- Reference to the yuhuh user
    user_id     uuid        not null references "users" (user_id)
);