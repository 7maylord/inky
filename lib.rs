#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod inky_todo {
    use ink::prelude::string::{String, ToString};
    use ink::storage::Mapping;

    /// Represents the status of a todo item
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub enum TodoStatus {
        Pending,
        Completed,
        Cancelled,
    }

    /// Represents a todo item
    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout))]
    pub struct Todo {
        pub id: u32,
        pub title: String,
        pub description: String,
        pub status: TodoStatus,
    }

    /// Defines the storage of your contract.
    #[ink(storage)]
    pub struct InkyTodo {
        next_todo_id: u32,
        todos: Mapping<u32, Todo>,
    }

    /// Events 
    #[ink(event)]
    pub struct TodoCreated {
        #[ink(topic)]
        todo_id: u32,
        title: String,
    }

    #[ink(event)]
    pub struct TodoUpdated {
        #[ink(topic)]
        todo_id: u32,
        new_status: TodoStatus,
    }

    #[ink(event)]
    pub struct TodoDeleted {
        #[ink(topic)]
        todo_id: u32,
        title: String,
    }

    impl Default for InkyTodo {
        fn default() -> Self {
            Self::new()
        }
    }

    impl InkyTodo {
        /// Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                next_todo_id: 1,
                todos: Mapping::new(),
            }
        }

        /// Create a new todo item
        #[ink(message)]
        pub fn create_todo(&mut self, title: String, description: String) -> Result<u32, String> {
        
            let todo_id = self.next_todo_id;
            
            // Validate input
            if title.is_empty() {
                return Err("Title cannot be empty".to_string());
            }

            let todo = Todo {
                id: todo_id,
                title: title.clone(),
                description,
                status: TodoStatus::Pending,
            };

            // Store the todo
            self.todos.insert(todo_id, &todo);

            // Increment next todo ID
            self.next_todo_id = self.next_todo_id.saturating_add(1);

            // Emit event
            self.env().emit_event(TodoCreated {
                todo_id,
                title,
            });

            Ok(todo_id)
        }

        /// Get a specific todo by ID
        #[ink(message)]
        pub fn get_todo(&self, todo_id: u32) -> Option<Todo> {
            self.todos.get(todo_id)
        }

        /// Update a todo item's status
        #[ink(message)]
        pub fn update_todo_status( &mut self, todo_id: u32, new_status: TodoStatus ) -> Result<(), String> {
            
            // Check if todo exists
            let mut todo = self.todos.get(todo_id)
                .ok_or("Todo not found")?;

            // Update the todo
            todo.status = new_status.clone();
            self.todos.insert(todo_id, &todo);

            // Emit event
            self.env().emit_event(TodoUpdated {
                todo_id,
                new_status,
            });

            Ok(())
        }

        /// Delete a todo item
        #[ink(message)]
        pub fn delete_todo(&mut self, todo_id: u32) -> Result<(), String> {
            // Check if todo exists and get its title
            let todo = self.todos.get(todo_id)
                .ok_or("Todo not found")?;
            let title = todo.title.clone();

            // Remove from storage
            self.todos.remove(todo_id);

            // Emit event
            self.env().emit_event(TodoDeleted {
                todo_id,
                title,
            });

            Ok(())
        }

    }
}