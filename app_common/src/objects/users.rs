use super::InfoResponse;
use crate::app_error::AppError;
use crate::app_state::{Data as AppData, DBConnection};
use crate::objects::ApiResponse;
use crate::schema::users as Schema;
use diesel::dsl::insert_into;
use diesel::{AsChangeset, Identifiable, RunQueryDsl};
use diesel::{ExpressionMethods, Insertable, QueryDsl, Queryable};
use hmac::{Hmac, Mac};
use jsonforms::json_forms::*;
use sha2::Sha256;
type HmacSha256 = Hmac<Sha256>;
use core::fmt::Write;
use jsonforms_derive::JsonForms;
use jwt::{SignWithKey, VerifyWithKey};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fmt::Debug;
use Schema::dsl::users as t_users;
static JWT_KEY: &[u8] = b"jdf84h5fb#^239hdei#9furf";

pub struct Users {
    users: Vec<User>,
}

pub(crate) fn db_new_user(
    app_state: &AppData,
    req: NewRequest,
) -> Result<InfoResponse, AppError> {
    let mut db_conn = app_state.db_pool.get().unwrap();
    match t_users
        .filter(Schema::dsl::user.eq(&req.user))
        .first::<UserDB>(&mut db_conn)
    {
        Ok(_) => {
            log::error!("new_user duplicate user {}", req.user);
            Err(AppError::ValidationError {
                field: "user",
                msg: "already exists",
            })
        }
        Err(_) => {
            check_password_requirements(&req.password, &req.confirm_password)?;
            let psha256 = compute_sha256(&req.password)?;

            let new_user = UserNew {
                user: &req.user,
                name: &req.name,
                password: psha256,
                json_state: "",
            };

            insert_into(t_users)
                .values(&new_user)
                .execute(&mut db_conn)?;

            let new_row = t_users
                .order(Schema::dsl::id.desc())
                .first::<UserDB>(&mut db_conn)?;

            let mut users = app_state.user_list.write().unwrap();
            let nuser = users.add_db_user(new_row);
            let token = nuser.get_token();
            log::info!(
                "new_user {} db_id:{:?} token:{token}",
                nuser.db.user,
                nuser.db.id
            );
            nuser.set_state(UserState::new(&nuser.db.id));
            nuser.db_save(&mut db_conn)?;

            Ok(InfoResponse {
                response: Some(ApiResponse::UsersLogin(LoginResponse { token })),
                user_state: nuser.state.clone(),
            })
        }
    }
}

fn check_password_requirements(password: &str, confirm_password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::ValidationError {
            field: "New Password",
            msg: "weak, less then 8 characters",
        });
    }
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(AppError::ValidationError {
            field: "New Password",
            msg: "weak, no upper case letter found",
        });
    }
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(AppError::ValidationError {
            field: "New Password",
            msg: "weak, no lower case letter found",
        });
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err(AppError::ValidationError {
            field: "New Password",
            msg: "weak, no digit found",
        });
    }
    lazy_static! {
        static ref RE_PUNCT: Regex = Regex::new(r#"[.,/#!$%\\^&\*;:{}=\-_`~()@]"#).unwrap();
    }
    if !RE_PUNCT.is_match(password) {
        return Err(AppError::ValidationError {
            field: "New Password",
            msg: r#"weak, no punctuation found. Use some of .,\/#!$%\\^&\*;:{}=\-_`~()@"#,
        });
    }
    if password != confirm_password {
        return Err(AppError::ValidationError {
            field: "New Password",
            msg: r#"does not match Confirm Password"#,
        });
    }
    Ok(())
}

fn compute_sha256(pwd: &str) -> Result<Vec<u8>, AppError> {
    let mut mac = HmacSha256::new_from_slice(JWT_KEY).unwrap();
    mac.update(pwd.as_bytes());
    let result = mac.finalize();
    Ok(result.into_bytes().to_vec())
}

pub(crate) fn db_login_user(
    app_state: &AppData,
    req: &LoginRequest,
) -> Result<InfoResponse, AppError> {
    let mut db_conn = app_state.db_pool.get().unwrap();

    match t_users
        .filter(Schema::dsl::user.eq(&req.user))
        .first::<UserDB>(&mut db_conn)
    {
        Ok(user) => {
            user.check_password(&req.password)?;
            let mut users = app_state.user_list.write().unwrap();
            let nuser = users.add_db_user(user);
            nuser.reset_state();
            let token = nuser.get_token();
            log::info!(
                "db_login_user {} db_id:{:?} token:{token}",
                nuser.db.user,
                nuser.db.id
            );
            Ok(InfoResponse {
                response: Some(ApiResponse::UsersLogin(LoginResponse { token })),
                user_state: nuser.state.clone(),
            })
        }
        Err(_) => {
            log::error!("db_login_user {} not found", req.user);
            Err(AppError::InvalidUser)
        }
    }
}

impl Users {
    pub fn new() -> Self {
        Self { users: vec![] }
    }

    pub fn add_db_user(&mut self, user: UserDB) -> &mut User {
        self.users.push(User::new(user));
        self.users.last_mut().unwrap()
    }

    pub fn find_user_by_state(&mut self, user_state: &UserState) -> Result<&mut User, AppError> {
        if let Some(user) = self.users.iter_mut().find(|x| x.db.id == user_state.id) {
            Ok(user)
        } else {
            Err(AppError::CacheError)
        }
    }

    pub fn find_user(&self, user: &str) -> Result<&User, AppError> {
        if let Some(user) = self.users.iter().find(|&x| x.db.user == user) {
            Ok(user)
        } else {
            Err(AppError::CacheError)
        }
    }

    pub fn login(&self, l: &LoginRequest) -> Result<InfoResponse, AppError> {
        let user = self.find_user(&l.user)?;
        user.db.check_password(&l.password)?;
        let token = user.get_token();
        log::info!(
            "login_user {} db_id:{:?} token:{token}",
            user.db.user,
            user.db.id
        );
        Ok(InfoResponse {
            response: Some(ApiResponse::UsersLogin(LoginResponse { token })),
            user_state: user.state.clone(),
        })
    }

    pub fn check_token(
        &self,
        token: &str,
        token_expire: chrono::Duration,
    ) -> Result<UserState, AppError> {
        let key = HmacSha256::new_from_slice(JWT_KEY).unwrap();
        let claims_res: Result<BTreeMap<String, String>, _> = token.verify_with_key(&key);
        if let Ok(claims) = claims_res {
            if let Some(username) = claims.get("user") {
                let user = self.find_user(username)?;
                if let Some(password) = claims.get("password") {
                    let mut s = String::with_capacity(2 * user.db.password.len());
                    for byte in &user.db.password {
                        write!(s, "{:02X}", byte).unwrap();
                    }
                    if password == &s {
                        if let Some(token_time) = claims.get("token_time") {
                            if chrono::DateTime::parse_from_rfc3339(token_time).unwrap()
                                + token_expire
                                > chrono::Utc::now()
                            {
                                if let Some(user_state) = user.state.clone() {
                                    return Ok(user_state);
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(AppError::InvalidToken)
    }

    pub fn save(&mut self, db_conn: &mut DBConnection) {
        log::info!("Users Save to DB ...");
        for user in self.users.iter_mut() {
            let _ = user.db_save(db_conn);
        }
        log::info!("Users are saved");
    }
}

impl Default for Users {
    fn default() -> Self {
        Self::new()
    }
}


#[derive(Debug, Insertable)]
#[diesel(table_name = Schema)]
pub struct UserNew<'a> {
    pub user: &'a str,
    pub name: &'a str,
    pub password: Vec<u8>,
    pub json_state: &'a str,
}

#[derive(Debug, Queryable, Identifiable, AsChangeset)]
#[diesel(table_name = Schema)]
pub struct UserDB {
    // db fields, keep in same order from schema.rs
    pub id: Option<i32>,
    pub user: String,
    pub name: String,
    pub password: Vec<u8>,
    pub json_state: String,
}

impl UserDB {
    pub fn check_password(&self, pwd: &str) -> Result<(), AppError> {
        let s = compute_sha256(pwd)?;
        if s == self.password {
            Ok(())
        } else {
            Err(AppError::InvalidUser)
        }
    }
    pub fn decode_state(&self) -> Option<UserState> {
        serde_json::from_str(&self.json_state).ok()
    }
}

#[derive(Debug)]
pub struct User {
    pub db: UserDB,
    pub state: Option<UserState>,
    pub requires_save: bool,
}

impl User {
    pub fn get_token(&self) -> String {
        let mut claims = BTreeMap::new();

        claims.insert("user", &self.db.user);
        let mut s = String::with_capacity(2 * self.db.password.len());
        for byte in &self.db.password {
            write!(s, "{:02X}", byte).unwrap();
        }
        claims.insert("password", &s);
        let now_str = chrono::Utc::now().to_rfc3339();
        claims.insert("token_time", &now_str);

        let key = HmacSha256::new_from_slice(JWT_KEY).unwrap();
        claims.sign_with_key(&key).unwrap()
    }

    fn new(user: UserDB) -> Self {
        Self {
            db: user,
            state: None,
            requires_save: false,
        }
    }

    pub fn set_state(&mut self, state: UserState) {
        self.state = Some(state);
        self.requires_save = true;
    }

    pub fn reset_state(&mut self) {
        self.state = self.db.decode_state();
    }

    pub fn db_save(&mut self, db_conn: &mut DBConnection) -> Result<(), AppError> {
        if self.requires_save {
            self.db.json_state = serde_json::to_string(&self.state).unwrap();
            let upd_query = diesel::update(t_users)
                .filter(Schema::dsl::id.eq(self.db.id))
                .set(&self.db);
            // #[cfg(debug_assertions)]
            // println!("db_login_user upd_query: {:?}",diesel::debug_query::<diesel::sqlite::Sqlite,_>(&upd_query));
            upd_query.execute(db_conn)?;
            self.requires_save = false;
            log::info!("user save state {}", self.db.user);
        }
        Ok(())
    }
}

#[derive(Deserialize, JsonForms)]
pub struct LoginRequest {
    #[jsonforms(schema = r#""minLength": 4, "title":"Login User""#)]
    pub user: String,
    #[jsonforms(schema = r#""passwordFmt":1, "format":"password", "title":"Login Password""#)]
    pub password: String,
}

impl JsonFormsButtons for LoginRequest {
    fn add_buttons(form: &mut JsonFormsResponse) {
        form.add_button(Button {
            name: "Sign In",
            btype: ButtonType::Submit,
            bpos: ButtonPos::Center,
            form: None,
        });
        form.add_button(Button {
            name: "Sign Up",
            btype: ButtonType::NextForm,
            bpos: ButtonPos::Right,
            form: Some(JsonFormsRequest {
                name: stringify_nosp!(users::NewRequest),
            }),
        });
    }
}

#[derive(Serialize, Debug)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Deserialize, JsonForms)]
pub struct NewRequest {
    #[jsonforms(schema = r#""minLength": 4, "title":"Login User""#)]
    pub user: String,
    #[jsonforms(schema = r#""minLength": 4"#)]
    pub name: String,
    #[jsonforms(schema = r#""passwordFmt":2, "format":"password", "title":"New Password""#)]
    pub password: String,
    #[jsonforms(schema = r#""passwordFmt":1, "format":"password", "title":"Confirm Password""#)]
    pub confirm_password: String,
}

impl JsonFormsButtons for NewRequest {
    fn add_buttons(form: &mut JsonFormsResponse) {
        form.add_button(Button {
            name: "Sign Up",
            btype: ButtonType::Submit,
            bpos: ButtonPos::Center,
            form: None,
        });
        form.add_button(Button {
            name: "Sign In",
            btype: ButtonType::NextForm,
            bpos: ButtonPos::Right,
            form: Some(JsonFormsRequest {
                name: stringify_nosp!(users::LoginRequest),
            }),
        });
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UserState {
    pub id: Option<i32>,
    pub user_lib: String,
    pub json_form: JsonFormsRequest,
    #[cfg(test)]
    pub dummy: Option<Vec<usize>>,
}

impl UserState {
    fn new(id: &Option<i32>) -> Self {
        Self {
            id: *id,
            user_lib: "user_app".to_owned(),
            json_form: JsonFormsRequest {
                name: "app::MainRequest".to_owned(),
            },
            #[cfg(test)]
            dummy: None,
        }
    }
    
    pub fn save(self, app_state: &AppData) -> Result<(),AppError> {
        let mut users = app_state.user_list.write().unwrap();
        let user = users.find_user_by_state(&self)?;
        user.set_state(self);
        Ok(())
    }
}


#[cfg(test)]
mod test {
    use actix_web::web;

    use super::*;
    use crate::app_state::Data as AppData;
    use std::{thread, time::Duration};
    const TEST_USER: &str = "test@self.com";
    const TEST_PWD: &str = "deJDH`$%dei67ofHJwwwII)(&^%#";
    const TEST_USERNAME: &str = "My Test User";
    const NTHREADS: usize = 10;
    const NTHREAD_ITERS: usize = 1_000;

    fn get_user_state(app_data: &AppData, token: &str) -> Result<UserState, AppError>
    {
        let users = app_data.user_list.read().unwrap();
        users.check_token(token, app_data.token_expires)
    }

    fn check_and_print_user(app_data: &AppData, token: &str) -> Result<(), AppError>
    {
        let user_state = get_user_state(app_data, token)?;
        println!(" UserState dummy: {:?}", &user_state.dummy.unwrap()[..NTHREADS]);
        Ok(())
    }

    fn process_token(app_data: &AppData, token: &str, i: usize) -> Result<(), AppError> {
        // get user state clone
        let mut user_state = get_user_state(app_data,token)?;

        //processing user state, based on request and db queries
        user_state.json_form.name = "check_user".to_owned();
        let mut dummy_vec = user_state.dummy.unwrap_or([0; 1000].to_vec());
        dummy_vec[i] += 1;
        user_state.dummy = Some(dummy_vec);
        thread::sleep(Duration::from_millis(5));
        //end user processing

        user_state.save(app_data)
    }
    #[test]
    fn user_login_and_process() {
        let app_data = web::Data::new(AppData::new());
        let login = db_login_user(
            &app_data,
            &LoginRequest {
                user: TEST_USER.to_owned(),
                password: TEST_PWD.to_owned(),
            },
        );
        assert!(login.is_ok(), "login error: {login:?}");
        if let ApiResponse::UsersLogin(r) = login.unwrap().response.unwrap() {
            let mut threads = vec![];
            for i in 0..NTHREADS {
                let my_token = r.token.clone();
                let my_data = app_data.clone();
                threads.push(thread::spawn(move || {
                    for _ in 0..NTHREAD_ITERS {
                        let check = process_token(&my_data, &my_token, i);
                        assert!(check.is_ok(), "get error {check:?}")
                    }
                }));
            }
            for thread in threads {
                let _ = thread.join();
            }
            let check_user = check_and_print_user(&app_data,&r.token);
            assert!(check_user.is_ok(), "login error: {check_user:?}");
        }
    }

    #[test]
    fn user_new() {
        let app_data = AppData::new();
        let login = db_new_user(
            &app_data,
            NewRequest {
                user: TEST_USER.to_owned(),
                name: TEST_USERNAME.to_owned(),
                password: TEST_PWD.to_owned(),
                confirm_password: TEST_PWD.to_owned(),
            },
        );
        match login {
            Err(AppError::ValidationError { field, msg }) => {
                assert!(
                    field == "user" && msg == "already exists",
                    "new user ValidationError {field} : {msg}"
                );
            }
            Err(_) => {
                assert!(login.is_ok(), "new user error: {login:?}");
            }
            Ok(_) => (),
        }
    }

    #[test]
    fn user_login() {
        let app_data = web::Data::new(AppData::new());
        let login = db_login_user(
            &app_data,
            &LoginRequest {
                user: TEST_USER.to_owned(),
                password: TEST_PWD.to_owned(),
            },
        );
        assert!(login.is_ok(), "login error: {login:?}");
    }


    
}
