CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  name VARCHAR(50),
  surname VARCHAR(50),
  age INTEGER,
  profession VARCHAR(50), 
  email VARCHAR(100)
);

INSERT INTO users (name, surname, age, profession, email) VALUES ('John', 'Doe', 28, 'Software Engineer', 'John.Doe@gmail.com');
INSERT INTO users (name, surname, age, profession, email) VALUES ('Jane', 'Smith', 34, 'Data Scientist', 'Jane.Smith@gmail.com');
INSERT INTO users (name, surname, age, profession, email) VALUES ('Jamaican', 'brotha', 28, 'Software Engineer', 'Jamaican.brotha@gmail.com');


