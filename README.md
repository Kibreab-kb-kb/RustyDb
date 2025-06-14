kbdb — Simple In-Memory Database in Rust

kbdb is a minimalist in-memory database implemented in Rust. It provides a foundational understanding of database internals by supporting basic SQL-like operations such as inserting and selecting rows, with simple error handling and table size limits.


    Future Plans
    
    Persistent Storage: Implement durable data storage to save and load the database state between sessions.

    Advanced Query Language: Expand beyond basic insert/select to support update, delete, and complex queries with filtering and sorting capabilities.

    Indexing: Build efficient indexing structures (e.g., B-trees) to speed up query performance on large datasets.

    Transaction Support: Add atomic transactions with rollback and commit capabilities to ensure data integrity.

    Concurrency: Enable safe concurrent access and modifications to support multi-threaded environments.

    Query Optimization: Develop query planning and optimization techniques to improve execution speed.

    Extensibility: Design the architecture for easy addition of new data types, functions, and storage engines.

    User Authentication & Access Control: Integrate basic security features for user management and permissions.

    Comprehensive Testing: Increase coverage with unit, integration, and performance tests to ensure reliability.

This project serves as a learning tool to deeply understand how databases work and to explore Rust’s capabilities in systems programming.
