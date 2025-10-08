-- Tabela za shranjevanje kartic userjev
CREATE TABLE IF NOT EXISTS cards (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    card_number TEXT NOT NULL UNIQUE,
    card_name TEXT,
    user_fullname TEXT,
    user_id TEXT,
    is_present BOOLEAN DEFAULT FALSE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- Trigger za posodobitev polja updated_at ob vsakem updateu vrstice
CREATE TRIGGER update_cards_updated_at
AFTER UPDATE ON cards
FOR EACH ROW
BEGIN
    UPDATE cards
    SET updated_at = CURRENT_TIMESTAMP
    WHERE id = OLD.id;
END;
