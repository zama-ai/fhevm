-- We switch to compute on allow and no longer require this event trigger 
DROP TRIGGER work_updated_trigger_from_computations_insertions ON computations;
