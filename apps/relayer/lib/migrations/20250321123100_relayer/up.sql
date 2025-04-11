CREATE TYPE Gateway_Operation_Status_Type AS ENUM ('TXRequested', 'TXFulfilled', 'ResponseFulfilled');

CREATE TYPE Gateway_Operation_Type AS ENUM ('PublicDecryption', 'PrivateDecryption', 'InputRegistration');

CREATE TABLE "gateway_requests" (
	"request_id" UUID NOT NULL PRIMARY KEY,
	"on_chain_request_id" BYTEA,
	"op" Gateway_Operation_Type NOT NULL,
	"status" Gateway_Operation_Status_Type NOT NULL,
	UNIQUE (op, on_chain_request_id)
);

CREATE TABLE "httpz_host_events"(
	"request_id" UUID NOT NULL PRIMARY KEY,
	"event_log" JSONB NOT NULL
);

-- Gateway events aka gateway responses
CREATE TABLE "gateway_responses" (
	id SERIAL PRIMARY KEY,
	"on_chain_request_id" BYTEA NOT NULL,
	"op" Gateway_Operation_Type NOT NULL,
	"event_log" JSONB NOT NULL
);

