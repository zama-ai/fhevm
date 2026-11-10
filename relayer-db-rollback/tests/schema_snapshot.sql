-- Order-insensitive description of the public schema, used by roundtrip.sh to
-- compare the database before a migration and after its down.
--
-- Column positions are deliberately left out: a column that is dropped and
-- added back lands at the end of its table, which no query can observe unless
-- it relies on `SELECT *` ordering. The sqlx bookkeeping table is left out too.

SELECT kind, parent, name, detail
FROM (
    SELECT 'column' AS kind, c.relname::text AS parent, a.attname::text AS name,
           format_type(a.atttypid, a.atttypmod)
               || ' not_null=' || a.attnotnull
               || ' default=' || coalesce(pg_get_expr(d.adbin, d.adrelid), '')
               || ' identity=' || a.attidentity::text || ' generated=' || a.attgenerated::text
               || ' collation=' || a.attcollation::regcollation AS detail
    FROM pg_attribute a
    JOIN pg_class c ON c.oid = a.attrelid AND c.relkind = 'r'
    LEFT JOIN pg_attrdef d ON d.adrelid = a.attrelid AND d.adnum = a.attnum
    WHERE c.relnamespace = 'public'::regnamespace AND a.attnum > 0 AND NOT a.attisdropped

    UNION ALL
    SELECT 'table', '', c.relname::text,
           'persistence=' || c.relpersistence::text || ' options=' || coalesce(c.reloptions::text, '')
               || ' rls=' || c.relrowsecurity
    FROM pg_class c
    WHERE c.relnamespace = 'public'::regnamespace AND c.relkind = 'r'

    UNION ALL
    SELECT 'index', tablename::text, indexname::text, indexdef
    FROM pg_indexes
    WHERE schemaname = 'public'

    UNION ALL
    SELECT 'constraint', conrelid::regclass::text, conname::text, pg_get_constraintdef(oid)
    FROM pg_constraint
    WHERE connamespace = 'public'::regnamespace AND conrelid <> 0

    UNION ALL
    SELECT 'trigger', tgrelid::regclass::text, tgname::text, 'enabled=' || tgenabled::text || ' ' || pg_get_triggerdef(oid)
    FROM pg_trigger
    WHERE NOT tgisinternal

    UNION ALL
    SELECT 'function', '', p.proname::text,
           'args=(' || pg_get_function_identity_arguments(p.oid) || ') returns=' || pg_get_function_result(p.oid)
               || ' language=' || l.lanname || ' volatility=' || p.provolatile::text || ' security_definer=' || p.prosecdef
               || ' comment=' || coalesce(obj_description(p.oid, 'pg_proc'), '')
               || ' body=' || regexp_replace(p.prosrc, '\s+', ' ', 'g')
    FROM pg_proc p
    JOIN pg_language l ON l.oid = p.prolang
    WHERE p.pronamespace = 'public'::regnamespace

    UNION ALL
    SELECT 'enum', t.typname::text, e.enumlabel::text,
           'position=' || rank() OVER (PARTITION BY t.oid ORDER BY e.enumsortorder)
    FROM pg_enum e
    JOIN pg_type t ON t.oid = e.enumtypid
    WHERE t.typnamespace = 'public'::regnamespace

    UNION ALL
    SELECT 'sequence', '', sequencename::text,
           data_type::text || ' start=' || start_value || ' min=' || min_value || ' max=' || max_value
               || ' increment=' || increment_by || ' cache=' || cache_size || ' cycle=' || cycle
    FROM pg_sequences
    WHERE schemaname = 'public'

    UNION ALL
    SELECT 'comment', 'relation', c.relname::text, obj_description(c.oid, 'pg_class')
    FROM pg_class c
    WHERE c.relnamespace = 'public'::regnamespace AND obj_description(c.oid, 'pg_class') IS NOT NULL

    UNION ALL
    SELECT 'comment', 'type', t.typname::text, obj_description(t.oid, 'pg_type')
    FROM pg_type t
    WHERE t.typnamespace = 'public'::regnamespace AND obj_description(t.oid, 'pg_type') IS NOT NULL

    UNION ALL
    SELECT 'comment', c.relname::text, a.attname::text, col_description(c.oid, a.attnum)
    FROM pg_class c
    JOIN pg_attribute a ON a.attrelid = c.oid AND a.attnum > 0 AND NOT a.attisdropped
    WHERE c.relnamespace = 'public'::regnamespace AND col_description(c.oid, a.attnum) IS NOT NULL
) AS schema_description
WHERE parent <> '_sqlx_migrations'
ORDER BY kind, parent, name, detail;
