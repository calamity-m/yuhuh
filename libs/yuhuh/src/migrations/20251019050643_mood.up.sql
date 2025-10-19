-- Assignents tables, which can be set by the user to customize
-- how they record their mood.
create table mood_assignments
(
    mood_assignment_id  uuid        primary key default uuidv7(),
    user_id             uuid        not null,
    val                 text        not null,
    -- lower is worse, higher is better
    idx                 smallint    not null            
);

create table energy_assignments
(
    energy_assignment_id uuid       primary key default uuidv7(),
    user_id               uuid      not null,
    val                   text      not null,
    -- lower is worse, higher is better
    idx                   smallint  not null
);

create table sleep_assignments
(
    sleep_assignment_id uuid        primary key default uuidv7(),
    user_id             uuid        not null,
    val                 text        not null,
    -- lower is worse, higher is better
    idx                 smallint    not null
);

-- Mood records
create table mood_records
(
    -- ID of the mood entry
    mood_record_id          uuid    primary key default uuidv7(),

    -- User this entry belongs to
    user_id                 uuid    not null,

    -- Time the mood was logged, or otherwise created into the db,
    created_at              timestamptz not null default now(),

    -- Last time the mood was updated, pretty self explanatory
    updated_at              timestamptz,

    -- Mood assignemnt
    mood_assignment_id      uuid not null,

    -- Energy assignment
    energy_assignment_id    uuid not null,

    -- Sleep assignment
    sleep_assignment_id     uuid not null,

    -- Optional notes
    notes                   text,
    
    CONSTRAINT fk_mood_records_user_id FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    CONSTRAINT mood_assignment_id FOREIGN KEY(mood_assignment_id) REFERENCES mood_assignments(mood_assignment_id) ON DELETE CASCADE,
    CONSTRAINT energy_assignment_id FOREIGN KEY(energy_assignment_id) REFERENCES energy_assignments(energy_assignment_id) ON DELETE CASCADE,
    CONSTRAINT sleep_assignment_id FOREIGN KEY(sleep_assignment_id) REFERENCES sleep_assignments(sleep_assignment_id) ON DELETE CASCADE
);

-- Apply our `updated_at` trigger from the first migration
SELECT trigger_updated_at('"mood_records"');