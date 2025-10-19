-- Activity records
create table mood_records
(
    -- ID of the food entry
    mood_record_id      uuid    primary key default uuidv7(),

    -- User this entry belongs to
    user_id             uuid    not null,

    -- Time the mood was logged, or otherwise created into the db,
    created_at          timestamptz not null default now(),

    -- Last time the mood was updated, pretty self explanatory
    updated_at          timestamptz,

    -- Mood value
    mood                smallint not null,

    -- Optional description of mood
    mood_description    text,
    
    CONSTRAINT fk_mood_records_user_id FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- Apply our `updated_at` trigger from the first migration
SELECT trigger_updated_at('"mood_records"');