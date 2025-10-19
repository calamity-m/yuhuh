-- Activity records
create table activity_records
(
    -- ID of the activity entry
    activity_record_id  uuid    primary key default uuidv7(),

    -- User this entry belongs to
    user_id             uuid    not null,

    -- Time the activity was logged, or otherwise created into the db,
    created_at          timestamptz not null default now(),

    -- Last time the activity was updated, pretty self explanatory
    updated_at          timestamptz,

    -- Name of activity
    activity            text not null,

    -- Type of activity. Ideally this would be a separated table for performance
    -- and just good design - but honestly at the level this thing will operate at
    -- it doesn't matter, and if it ever does grow to that level... the migration
    -- to follow the proper handling of this won't be that difficult to do.
    -- At least, I think so?
    activity_type       text not null,

    -- JSON data related to the activity. This can store extremely varied information dependinging
    -- on activity_type, and even within the same type the data may be completely different.
    -- For example, cycling may be an activity_type, but the info will change if someone is doing
    -- a casual group ride, versus zone 2 exercise.
    activity_info       jsonb not null,
    
    CONSTRAINT fk_activity_records_user_id FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE
);

-- Apply our `updated_at` trigger from the first migration
SELECT trigger_updated_at('"activity_records"');