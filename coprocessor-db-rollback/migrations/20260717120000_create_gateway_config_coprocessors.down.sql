-- Down for 20260717120000_create_gateway_config_coprocessors.
-- Drops the table. The cached coprocessor registry is lost; the v0.15 listener
-- rebuilds it from the gateway when it runs again.

DROP TABLE IF EXISTS public.gateway_config_coprocessors;
