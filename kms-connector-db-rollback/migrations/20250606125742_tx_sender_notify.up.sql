-- Create functions to notify TransactionSender when decryption responses are received
CREATE OR REPLACE FUNCTION notify_public_decryption_response()
    RETURNS trigger AS $$
BEGIN
    NOTIFY public_decryption_response_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION notify_user_decryption_response()
    RETURNS trigger AS $$
BEGIN
    NOTIFY user_decryption_response_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create triggers to fire once per statement on decryption responses inserts
CREATE OR REPLACE TRIGGER trigger_from_public_decryption_responses_insertions
    AFTER INSERT
    ON public_decryption_responses
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_public_decryption_response();

CREATE OR REPLACE TRIGGER trigger_from_user_decryption_responses_insertions
    AFTER INSERT
    ON user_decryption_responses
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_user_decryption_response();
