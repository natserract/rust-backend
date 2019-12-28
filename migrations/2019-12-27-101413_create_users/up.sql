

CREATE TABLE users (
  id INTEGER AUTO_INCREMENT PRIMARY KEY NOT NULL,
  name varchar(191) NOT NULL,
  email varchar(191) UNIQUE NOT NULL,
  password varchar(191) NOT NULL 
);
