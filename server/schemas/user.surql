DEFINE TABLE OVERWRITE user SCHEMAFULL;

    
-- Define fields for the user table with types and constraints
DEFINE FIELD email ON TABLE user TYPE string;
DEFINE FIELD first_name ON TABLE user TYPE option<string>;
DEFINE FIELD last_name ON TABLE user TYPE option<string>;
DEFINE FIELD image_url ON TABLE user TYPE string;
DEFINE FIELD created_at ON TABLE user TYPE datetime DEFAULT time::now();
DEFINE FIELD updated_at ON TABLE user TYPE datetime DEFAULT time::now();
    
-- Define an index for faster lookups
DEFINE INDEX userEmailIndex ON TABLE user COLUMNS email UNIQUE;