ALTER TABLE computations
    DROP CONSTRAINT computations_pkey;

ALTER TABLE computations
    ADD PRIMARY KEY (tenant_id, output_handle, transaction_id);
