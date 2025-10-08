-- Punch-in tabela za loge vseh dogodkov prijave/odjave
CREATE TABLE IF NOT EXISTS punches (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    card_id INTEGER NOT NULL,
    status TEXT NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    FOREIGN KEY(card_id) REFERENCES cards(id)
);
