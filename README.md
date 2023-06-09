# Rust Web Server based on actix-web framework

Is used with the react app found in client_app folder. Read the README of the `client_app` for details about the client.  
Uses diesel for database management. Current implementation uses sqlite. The db folder contains diesel migration files.

## File Structure

Let's briefly have a look at the most important packages:

- `app_common` common folder for shared global app state data. Implements user management backend.
- `server` main server app
- `user_app_common` share user app structures for client requests and response.
- `user-app` implements the user main form
- `user_wasm_lib` wasm module for `user_app_common` structures. It is used by client app. It is built from client app using `npm run build:wasm`
- `jsonforms` crate with jsonforms trait and common structs for reponse to client jsonform requests.
- `jsonforms_derive` crate with jsonforms proc macros that applied over a struct implements JsonFormsSerializable trait

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in
the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without
any additional terms or conditions.