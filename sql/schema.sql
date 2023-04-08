DROP SCHEMA IF EXISTS psh CASCADE;
CREATE SCHEMA psh;

CREATE TABLE psh.posits (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  title VARCHAR(128) NOT NULL UNIQUE,
  content TEXT NOT NULL,
  topic VARCHAR(72) NOT NULL,
  author_name VARCHAR(72) NOT NULL,
  UNIQUE (title)
);

CREATE TABLE psh.users (
  id UUID PRIMARY KEY DEFAULT gen_random_uuid (),
  created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
  fullname VARCHAR(72) NOT NULL,
  password VARCHAR(72) NOT NULL,
  username VARCHAR(72) NOT NULL UNIQUE,
  email VARCHAR(128) NOT NULL UNIQUE,
  UNIQUE (username, email)
);