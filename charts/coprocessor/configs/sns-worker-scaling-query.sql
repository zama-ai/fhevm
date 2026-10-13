-- SnS tasks a worker can run now: not completed, input ciphertext present.
SELECT count(*)
FROM pbs_computations a
JOIN ciphertexts c ON a.handle = c.handle
WHERE a.is_completed = FALSE AND c.ciphertext IS NOT NULL
