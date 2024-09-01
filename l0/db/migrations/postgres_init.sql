-- +goose Up
-- +goose StatementBegin
SELECT 'up SQL query';

CREATE TABLE IF NOT EXISTS orders
(
   order_uid    VARCHAR NOT NULL PRIMARY KEY,
   track_number VARCHAR,
   entry        VARCHAR
);



INSERT INTO orders (order_uid, track_number, entry)
VALUES ('b563feb7b2b84b6test', 'WBILMTESTTRACK', 'WBIL');

-- +goose StatementEnd

-- +goose Down
-- +goose StatementBegin
-- SELECT 'down SQL query';
-- +goose StatementEnd