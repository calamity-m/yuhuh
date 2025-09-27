-- Food records
create table food_records
(
    -- ID of the food entry
    food_record_id  uuid    primary key default uuidv7(),

    -- User this entry belongs to
    user_id         uuid    not null,

    -- Time the food was eaten, or otherwise created into the db,
    created_at      timestamptz not null default now(),

    -- Description of the food entry
    description     text    not null,
    
    -- Calories/kilojules
    calories        real,

    -- Macronutrients
    carbs           real,
    protein         real,
    fats            real,

    -- Unstructured json around micronutirents
    micronutrients  jsonb,
    
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES users(user_id) ON DELETE CASCADE
);