#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod greeting_chat {
    /// Defines the storage of your contract.
    /// This struct will store the greeting message.
    #[ink(storage)]
    pub struct GreetingChat {
        /// Stores a greeting message as a `String`.
        greeting: ink::prelude::string::String,
    }

    impl GreetingChat {
        /// Constructor that initializes the greeting message.
        #[ink(constructor)]
        pub fn new(init_message: ink::prelude::string::String) -> Self {
            Self { greeting: init_message }
        }

        /// Constructor that initializes the greeting to a default message.
        #[ink(constructor)]
        pub fn default() -> Self {
            Self::new("Hello, welcome to the chat!".into())
        }

        /// Message to set a new greeting message.
        #[ink(message)]
        pub fn set_greeting(&mut self, new_message: ink::prelude::string::String) {
            self.greeting = new_message;
        }

        /// Message to get the current greeting message.
        #[ink(message)]
        pub fn get_greeting(&self) -> ink::prelude::string::String {
            self.greeting.clone()
        }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    #[cfg(test)]
    mod tests {
        /// Imports all definitions from the outer scope to use here.
        use super::*;

        /// Tests if the default constructor works correctly.
        #[ink::test]
        fn default_works() {
            let greeting_chat = GreetingChat::default();
            assert_eq!(greeting_chat.get_greeting(), "Hello, welcome to the chat!");
        }

        /// Tests setting a new greeting.
        #[ink::test]
        fn it_works() {
            let mut greeting_chat = GreetingChat::new("Hi there!".into());
            assert_eq!(greeting_chat.get_greeting(), "Hi there!");
            greeting_chat.set_greeting("Hello, Ink!".into());
            assert_eq!(greeting_chat.get_greeting(), "Hello, Ink!");
        }
    }

    /// End-to-end tests for the greeting chat contract.
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;
        use ink_e2e::ContractsBackend;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<(), ()>) -> E2EResult<()> {
            // Instantiate contract with default constructor.
            let mut constructor = GreetingChatRef::default();
            let contract = client
                .instantiate("greeting_chat", &ink_e2e::alice(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let call_builder = contract.call_builder::<GreetingChat>();

            // Verify default greeting.
            let get_greeting = call_builder.get_greeting();
            let get_result = client.call(&ink_e2e::alice(), &get_greeting).dry_run().await?;
            assert_eq!(get_result.return_value(), "Hello, welcome to the chat!");

            Ok(())
        }

        #[ink_e2e::test]
        async fn set_greeting_works(mut client: ink_e2e::Client<(), ()>) -> E2EResult<()> {
            // Instantiate with a custom greeting.
            let mut constructor = GreetingChatRef::new("Hello, Ink!".into());
            let contract = client
                .instantiate("greeting_chat", &ink_e2e::bob(), &mut constructor)
                .submit()
                .await
                .expect("instantiate failed");
            let mut call_builder = contract.call_builder::<GreetingChat>();

            // Verify initial greeting.
            let get_greeting = call_builder.get_greeting();
            let get_result = client.call(&ink_e2e::bob(), &get_greeting).dry_run().await?;
            assert_eq!(get_result.return_value(), "Hello, Ink!");

            // Update the greeting.
            let set_greeting = call_builder.set_greeting("Good day!".into());
            client.call(&ink_e2e::bob(), &set_greeting).submit().await.expect("set greeting failed");

            // Verify updated greeting.
            let get_greeting = call_builder.get_greeting();
            let get_result = client.call(&ink_e2e::bob(), &get_greeting).dry_run().await?;
            assert_eq!(get_result.return_value(), "Good day!");

            Ok(())
        }
    }
}
