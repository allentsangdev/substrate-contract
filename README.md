## Substrate Smart Contract: my_psp34

## Group Member
- Tsang Ho Sang 101448933
- Winston Edwards 101454440 

### Summary:
The "my_psp34" smart contract is developed as a substrate-based blockchain contract using the "openbrush" crate. Its primary goal is to provide a secure authentication and deauthentication mechanism for apps interacting with the blockchain. The contract maintains a list of authenticated app account IDs, allowing certain apps to access specific functionalities or resources within the blockchain ecosystem. By implementing the "PSP34" trait, the contract ensures a standardized interface for authentication-related operations, promoting seamless integration with other contracts and components in the blockchain ecosystem.

### Players:
The primary users of the "my_psp34" smart contract are the **App Users**—entities or individuals who own and use applications (apps) that interact with the blockchain. These app users require access to certain functionalities or resources within the blockchain ecosystem, which is controlled through app authentication.

### Scenario:
In this scenario, **App Users** interact with the "my_psp34" smart contract to authenticate or deauthenticate their apps with the blockchain. The interaction involves invoking specific functions of the contract.

- **Authentication**: When an app user wants to authenticate their app, they interact with their app's user interface or backend. The app communicates with the "my_psp34" contract through contract-specific function calls, providing relevant identification information. For example, the app might prompt the user to enter their unique user credentials or use other authentication mechanisms like cryptographic keys or digital signatures. Once the app user initiates the authentication request, the app calls the "authenticate" function of the "my_psp34" contract, passing the user's account ID as an argument. If the provided account ID is verified and valid, the "authenticate" function adds the user's account ID to the list of authenticated apps, granting the app access to specific functionalities or resources.

- **Deauthentication**: To revoke app access, an app user interacts with their app's user interface or backend to initiate the deauthentication process. The app communicates with the "my_psp34" contract through contract-specific function calls. For instance, the app might provide a "Logout" button or an account management section where the user can choose to deauthenticate the app. Once the user initiates the deauthentication request, the app calls the "deauthenticate" function of the "my_psp34" contract, passing the user's account ID as an argument. If the provided account ID exists in the list of authenticated apps, the "deauthenticate" function removes the user's account ID from the list, revoking access to specific functionalities or resources associated with the app.

### Actions:
The actions taken by **App Users** to interact with the smart contract are straightforward. To authenticate their app, an app user initiates the authentication process through their app's user interface or backend. The app then communicates with the "my_psp34" contract, calling the "authenticate" function and providing the user's account ID for verification. Upon successful verification, the "authenticate" function adds the account ID to the list of authenticated apps, granting access to app-specific functionalities.

Similarly, to deauthenticate an app, an app user initiates the deauthentication process through the app's user interface or backend. The app communicates with the "my_psp34" contract, calling the "deauthenticate" function and providing the user's account ID. If the provided account ID exists in the list of authenticated apps, the "deauthenticate" function removes the account ID from the list, effectively revoking the app's access to certain functionalities or resources.

By using the "my_psp34" smart contract, app developers and users can implement a secure and standardized authentication mechanism for their applications within the substrate-based blockchain ecosystem. The contract acts as a central authority for managing app authentication, providing a seamless and controlled user experience for accessing different functionalities and resources within the blockchain.