-- Create role
DO $$
  BEGIN
  CREATE ROLE r_indexer;
  EXCEPTION WHEN duplicate_object THEN RAISE NOTICE '%, skipping', SQLERRM USING ERRCODE = SQLSTATE;
  END
$$;

-- Create user
DO $$
  BEGIN
  CREATE USER indexer;
  EXCEPTION WHEN duplicate_object THEN RAISE NOTICE '%, skipping', SQLERRM USING ERRCODE = SQLSTATE;
  END
$$;

-- Assign roles
GRANT r_indexer TO indexer;
