-- Mood records
create table mood_records
(
    -- ID of the mood entry
    mood_record_id      uuid    primary key default uuidv7(),

    -- User this entry belongs to
    user_id             uuid    not null,

    -- Time the mood was entry was created
    created_at          timestamptz not null default now(),

    -- Last time the mood was updated, pretty self explanatory
    updated_at          timestamptz,

    -- Mood assignemnt
    mood                smallint,

    -- Energy assignment
    energy             smallint,

    -- Sleep assignment
    sleep               smallint,

    -- Optional notes
    notes               text,

    -- Time the mood was logged, or otherwise inserted
    logged_at      timestamptz not null default now(),
    
    CONSTRAINT fk_mood_records_user_id FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- Apply our `updated_at` trigger from the first migration
SELECT trigger_updated_at('"mood_records"');