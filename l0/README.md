# Демонстрационный сервис, отображающий данные о заказе
Необходимо разработать демонстрационный сервис с простейшим интерфейсом, возвращающий данные о заказе.

## Run
```shell
# postgres db
docker compose -f ./docker-compose.yml rm && \
docker compose -f ./docker-compose.yml build --no-cache && \
docker compose -f ./docker-compose.yml up

# app
cargo run
```

**Модель данных в формате JSON**
```json
{
  "order_uid": "b563feb7b2b84b6test",
  "track_number": "WBILMTESTTRACK",
  "entry": "WBIL",
  "delivery": {
    "name": "Test Testov",
    "phone": "+9720000000",
    "zip": "2639809",
    "city": "Kiryat Mozkin",
    "address": "Ploshad Mira 15",
    "region": "Kraiot",
    "email": "test@gmail.com"
  },
  "payment": {
    "transaction": "b563feb7b2b84b6test",
    "request_id": "",
    "currency": "USD",
    "provider": "wbpay",
    "amount": 1817,
    "payment_dt": 1637907727,
    "bank": "alpha",
    "delivery_cost": 1500,
    "goods_total": 317,
    "custom_fee": 0
  },
  "items": [
    {
      "chrt_id": 9934930,
      "track_number": "WBILMTESTTRACK",
      "price": 453,
      "rid": "ab4219087a764ae0btest",
      "name": "Mascaras",
      "sale": 30,
      "size": "0",
      "total_price": 317,
      "nm_id": 2389212,
      "brand": "Vivienne Sabo",
      "status": 202
    }
  ],
  "locale": "en",
  "internal_signature": "",
  "customer_id": "test",
  "delivery_service": "meest",
  "shardkey": "9",
  "sm_id": 99,
  "date_created": "2021-11-26T06:22:19Z",
  "oof_shard": "1"
}
```

## API Description
```http request
### Example order
GET http://localhost:8000/orders/b563feb7b2b84b6test HTTP/1.1
Host: localhost:8000
Content-Type: application/json
Accept: */*
```

```http request
POST http://localhost:8000/orders HTTP/1.1
Host: localhost:8000
Content-Type: application/json
Accept: */*

{
  "order_uid": "test1september2024",
  "track_number": "MRBEAST",
  "entry": "OZON",
  ...
}
```

## DB Schema
```mermaid
erDiagram
%%    Orders ||--o{ NAMED-DRIVER : allows
    Orders {
        VARCHAR order_uid PK
        VARCHAR track_number
        VARCHAR entry
        VARCHAR locale
        VARCHAR internal_signature
        VARCHAR customer_id
        VARCHAR delivery_service
        VARCHAR shardkey
        TIMESTAMPZ date_created
        VARCHAR oof_shard
    }
    Deliveries ||--o{ Orders : is
    Deliveries {
        VARCHAR order_uid PK,FK
        VARCHAR name
        VARCHAR phone
        VARCHAR zip
        VARCHAR city
        VARCHAR address
        VARCHAR region
        VARCHAR email
    }
    Items ||--o{ Orders : is
    Items {
        VARCHAR order_uid FK
        BIGINT  chrt_id
        VARCHAR  track_number
        BIGINT  price
        VARCHAR  rid
        VARCHAR  name
        BIGINT  sale
        VARCHAR  i_size
        BIGINT  total_price
        BIGINT  nm_id
        VARCHAR  brand
        BIGINT  status
    }
    Payments ||--o{ Orders : is
    Payments {
        VARCHAR transaction_id FK
        VARCHAR request_id
        VARCHAR currency
        VARCHAR provider
        BIGINT amount
        BIGINT payment_dt
        VARCHAR bank
        BIGINT delivery_cost
        BIGINT goods_total
        BIGINT custom_fee
    }
```


# Затронутые темы: Заметки
## Axum (без middleware), разделяемое состояние
## Rust: логирование
## Опционально: Rust: unit-тесты
## Rust: clippy
## Rust: таймауты
## Опционально: бенчмаркинг
## Опционально: обработка аргументов командной строки (clap)
## Сериализация/десериализация (serde, json)
## Опционально: БД/транзакции
## разделяемые данные через Arc


# Refs.
- [Rust CRUD API Example with Axum and PostgreSQL](https://codevoweb.com/rust-crud-api-example-with-axum-and-postgresql/)
- [Rust Axum Full Course - Web Development (GitHub repo updated to Axum 0.7)](https://www.youtube.com/watch?v=XZtlD_m59sM)
- [Build a CRUD REST API with Rust Axum | Tutorial](https://www.youtube.com/watch?v=NJsTgmayHZY)
- [SQLx is my favorite PostgreSQL driver to use with Rust.](https://www.youtube.com/watch?v=TCERYbgvbq0)