INSERT INTO
  account (account_id, name, account_type, current_balance)
VALUES
  (1, "test account", "Cash", 0);

INSERT INTO
  category (category_id, name, budget, parent_id)
VALUES
  (1, "test category", NULL, NULL);

INSERT INTO
  record (
    account_id,
    record_type,
    amount,
    category_id,
    description,
    created_at,
    updated_at
  )
VALUES
  (
    1,
    2,
    1000,
    NULL,
    "test record",
    '2025-08-24 00:00:00 +00:00',
    '2025-08-24 00:00:00 +00:00'
  );
