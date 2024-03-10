CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE public.TWEET
(
    ID      UUID        PRIMARY KEY DEFAULT uuid_generate_v4(),
    BODY    TEXT        NOT NULL
);

-- Inserting a single tweet with a specific ID and body
INSERT INTO public.TWEET (ID, BODY) VALUES ('6d1961d2-8c8e-4f58-a2c3-6214ae0b4b35', 'This is a sample tweet.');

-- Inserting multiple tweets using a single query
INSERT INTO public.TWEET (ID, BODY) 
VALUES 
    ('a0b7e3f6-3c9d-4a7b-b0fe-68e0d2e3c294', 'Another tweet here.'),
    ('e2f91ad7-5f0e-4ee6-a0c6-8c3d9db64299', 'Yet another tweet.');

-- Inserting a tweet with a very long body
INSERT INTO public.TWEET (ID, BODY) VALUES ('fa0829ac-55ac-4b12-bc58-65abec9e9248', 'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat. Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur. Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum.');
