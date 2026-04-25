-- noinspection SqlNoDataSourceInspectionForFile

CREATE SCHEMA web_hs;

CREATE TABLE web_hs.widgets
(
    id         UUID      NOT NULL,
    PRIMARY KEY (id),
    name       TEXT      NOT NULL,
    created_at TIMESTAMP NOT NULL,
    deleted_at TIMESTAMP DEFAULT NULL,
    UNIQUE (name)
);
