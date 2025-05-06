-- First, let's define a better text analyzer
DEFINE ANALYZER custom_analyzer TYPE custom TOKENIZERS whitespace, class FILTERS lowercase, ascii;

-- Schema definitions with added search indexes
DEFINE TABLE markdown;
DEFINE FIELD created_at ON markdown TYPE datetime;
DEFINE FIELD updated_at ON markdown TYPE datetime;
DEFINE FIELD md ON markdown TYPE string;
DEFINE INDEX markdown_content ON markdown FIELDS md SEARCH ANALYZER custom_analyzer BM25 HIGHLIGHTS;

DEFINE TABLE definition;
DEFINE FIELD created_at ON definition TYPE datetime;
DEFINE FIELD updated_at ON definition TYPE datetime;
DEFINE FIELD term ON definition TYPE string;
DEFINE FIELD explanation ON definition TYPE string;
DEFINE INDEX definition_term ON definition FIELDS term SEARCH ANALYZER custom_analyzer BM25 HIGHLIGHTS;
DEFINE INDEX definition_explanation ON definition FIELDS explanation SEARCH ANALYZER custom_analyzer BM25 HIGHLIGHTS;

DEFINE TABLE definition_list;
DEFINE FIELD created_at ON definition_list TYPE datetime;
DEFINE FIELD updated_at ON definition_list TYPE datetime;
DEFINE FIELD definitions ON definition_list TYPE array<record<definition>>;
DEFINE FIELD title ON definition_list TYPE string;
DEFINE INDEX definition_list_title ON definition_list FIELDS title SEARCH ANALYZER custom_analyzer BM25 HIGHLIGHTS;

-- Sample data insertion
-- Markdown entries
INSERT INTO markdown {
    created_at: time::now(),
    updated_at: time::now(),
    md: "# Getting Started with Development Tools\n\nThis guide covers essential development tools and practices..."
};

INSERT INTO markdown {
    created_at: time::now(),
    updated_at: time::now(),
    md: "## Best Practices for Code Review\n\nCode reviews are essential for maintaining code quality..."
};

-- Definitions
INSERT INTO definition {
    created_at: time::now(),
    updated_at: time::now(),
    term: "Git",
    explanation: "A distributed version control system for tracking changes in source code during software development."
};

INSERT INTO definition {
    created_at: time::now(),
    updated_at: time::now(),
    term: "Docker",
    explanation: "A platform for developing, shipping, and running applications in containers."
};

INSERT INTO definition {
    created_at: time::now(),
    updated_at: time::now(),
    term: "CI/CD",
    explanation: "Continuous Integration and Continuous Deployment - automated processes for building, testing, and deploying code."
};

-- Definition Lists
INSERT INTO definition_list {
    created_at: time::now(),
    updated_at: time::now(),
    title: "Development Tools Glossary",
    definitions: (SELECT * FROM definition WHERE term IN ["Git", "Docker"])
};

-- Example search queries
-- Search across all content
SELECT 
    'markdown' as type,
    id,
    md as content,
    search::score() as score
FROM markdown 
WHERE md @@ 'development'
UNION SELECT
    'definition' as type,
    id,
    explanation as content,
    search::score() as score
FROM definition
WHERE term @@ 'git' OR explanation @@ 'development'
ORDER BY score DESC;