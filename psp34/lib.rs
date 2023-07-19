#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]
        
#[openbrush::contract]
pub mod my_psp34 {
    
    // imports from openbrush
	use openbrush::contracts::psp34::*;
	use openbrush::traits::Storage;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
    	#[storage_field]
		psp34: psp34::Data,
    }
    
    // Section contains default implementation without any modifications
	impl PSP34 for Contract {}
     
    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            Self {
                owner: Self::env().caller(),
                authenticated_apps: ink_prelude::Vec::new(),
            }
        }

        #[ink(message)]
        pub fn authenticate(&mut self) {
            let caller = Self::env().caller();
            self.authenticated_apps.push(caller);
        }

        #[ink(message)]
        pub fn deauthenticate(&mut self) {
            let caller = Self::env().caller();
            if let Some(index) = self.authenticated_apps.iter().position(|&app| app == caller) {
                self.authenticated_apps.remove(index);
            }
        }

        #[ink(message)]
        pub fn is_authenticated(&self, app: AccountId) -> bool {
            self.authenticated_apps.contains(&app)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[ink::test]
        fn new_identity_has_no_authenticated_apps() {
            // Create a new instance of the Identity contract
            let mut identity = Identity::new();
    
            // Get the caller's account ID
            let caller = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts")
                .alice;
    
            // Assert that the caller is the owner of the identity
            assert_eq!(identity.owner, caller);
    
            // Assert that the identity has no authenticated apps initially
            assert_eq!(identity.authenticated_apps.len(), 0);
        }
    
        #[ink::test]
        fn authenticate_and_deauthenticate_apps() {
            // Create a new instance of the Identity contract
            let mut identity = Identity::new();
    
            // Get some app account IDs for testing
            let app1 = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts")
                .bob;
            let app2 = ink_env::test::default_accounts::<ink_env::DefaultEnvironment>()
                .expect("Cannot get accounts")
                .charlie;
    
            // Authenticate app1
            identity.authenticate(app1);
    
            // Check if app1 is now authenticated
            assert!(identity.is_authenticated(app1));
    
            // Check if app2 is not authenticated
            assert!(!identity.is_authenticated(app2));
    
            // Authenticate app2
            identity.authenticate(app2);
    
            // Check if app2 is now authenticated
            assert!(identity.is_authenticated(app2));
    
            // Deauthenticate app1
            identity.deauthenticate(app1);
    
            // Check if app1 is no longer authenticated
            assert!(!identity.is_authenticated(app1));
    
            // Check if app2 is still authenticated
            assert!(identity.is_authenticated(app2));
        }
    }

}

