-- Add display_name and triggers to personas table
-- display_name: custom name the persona responds to (if NULL, uses bot's default name)
-- triggers: comma-separated keywords that activate this persona

ALTER TABLE personas ADD COLUMN display_name TEXT;
ALTER TABLE personas ADD COLUMN triggers TEXT;

-- Update existing personas with their names as display_name
UPDATE personas SET display_name = name WHERE display_name IS NULL;
