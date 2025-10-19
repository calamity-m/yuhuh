-- Add down migration script here
drop function if exists trigger_updated_at(tablename regclass);
drop function if exists set_updated_at();
