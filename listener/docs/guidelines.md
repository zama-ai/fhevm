# Guidelines

The rationnale is: GIVE NO ROOM TO MISS SOMETHING (a block, a transaction, a receipt, a log), crash or skip a processing is not permitted.

- MUST NEVER panic !
- No uncontrolled unwraps.
- Should retry indefinitely most of the time if there is a encountered error and raise an alert if needed to look after it.
- Be consistent in error management, anyhow or box dyn
- Think alerting.
- Think profiling if needed.
- Think strategy pattern.
- Think separations of concerns regarding logic.
- Think reusable code.

(The rationnale is: IMPOSSIBILITY TO MISS SOMETHING, crash or skip a processing)
