ALTER TABLE computations
ADD COLUMN IF NOT EXISTS schedule_order TIMESTAMP NOT NULL DEFAULT NOW();
