#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

extern crate reqwest;
extern crate rocket_contrib;
extern crate uuid;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use rocket::config::Config;
use rocket::State;
use rocket_contrib::json::Json;
use std::collections::HashMap;
use std::fmt;
use std::str;
use std::sync::Arc;
use std::sync::RwLock;
use uuid::Uuid;

const PARTIES: u32 = 2;
const THRESHOLD: u32 = 1;
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub struct TupleKey {
    pub first: String,
    pub second: String,
    pub third: String,
    pub fourth: String,
}
impl TupleKey {
    fn new(first: String, second: String, third: String, fourth: String) -> TupleKey {
        return TupleKey {
            first,
            second,
            third,
            fourth,
        };
    }
}
fn pr<T: std::fmt::Debug + ?Sized>(x: &String) {
    println!("{:?}", &*x);
}
impl fmt::Display for TupleKey {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.first, self.second, self.third, self.fourth
        )
    }
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct PartySignup {
    pub number: u32,
    pub uuid: String,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Index {
    pub key: TupleKey,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Entry {
    pub key: TupleKey,
    pub value: String,
}
#[post("/get", format = "json", data = "<request>")]
fn get(
    db_mtx: State<RwLock<HashMap<TupleKey, String>>>,
    request: Json<Index>,
) -> Json<Result<Entry, ()>> {
    let index: Index = request.0;
    let hm = db_mtx.read().unwrap();
    match hm.get(&index.key) {
        Some(v) => {
            let entry = Entry {
                key: index.key,
                value: format!("{}", v.clone()),
            };
            Json(Ok(entry))
        }
        None => Json(Err(())),
    }
}

#[post("/set", format = "json", data = "<request>")]
fn set(
    db_mtx: State<RwLock<HashMap<TupleKey, String>>>,
    request: Json<Entry>,
) -> Json<Result<(), ()>> {
    let entry: Entry = request.0;
    let mut hm = db_mtx.write().unwrap();
    hm.insert(entry.key.clone(), entry.value.clone());
    Json(Ok(()))
}

#[post("/signupkeygen", format = "json")]
fn signup_keygen(
    db_mtx: State<RwLock<HashMap<TupleKey, String>>>,
) -> Json<Result<PartySignup, ()>> {
    let key = TupleKey {
        first: "signup".to_string(),
        second: "keygen".to_string(),
        third: "".to_string(),
        fourth: "".to_string(),
    };
    let mut party_signup: PartySignup;
    {
        let hm = db_mtx.read().unwrap();
        let value = hm.get(&key).unwrap();
        let party_i_minus1_signup: PartySignup = serde_json::from_str(&value).unwrap();
        if party_i_minus1_signup.number < PARTIES {
            let party_num = party_i_minus1_signup.number + 1;
            party_signup = PartySignup {
                number: party_num.clone(),
                uuid: party_i_minus1_signup.uuid,
            };
        } else {
            let uuid = Uuid::new_v4().to_string();
            let party1 = 1;
            party_signup = PartySignup {
                number: party1,
                uuid,
            };
        }
    }
    let mut hm = db_mtx.write().unwrap();
    hm.insert(key, serde_json::to_string(&party_signup).unwrap());
    return Json(Ok(party_signup));
}

#[post("/signupsign", format = "json")]
fn signup_sign(db_mtx: State<RwLock<HashMap<TupleKey, String>>>) -> Json<Result<PartySignup, ()>> {
    let key = TupleKey {
        first: "signup".to_string(),
        second: "sign".to_string(),
        third: "".to_string(),
        fourth: "".to_string(),
    };
    let mut party_signup: PartySignup;
    {
        let hm = db_mtx.read().unwrap();
        let value = hm.get(&key).unwrap();
        let party_i_minus1_signup: PartySignup = serde_json::from_str(&value).unwrap();
        if party_i_minus1_signup.number < THRESHOLD + 1 {
            let party_num = party_i_minus1_signup.number + 1;
            party_signup = PartySignup {
                number: party_num.clone(),
                uuid: party_i_minus1_signup.uuid,
            };
        } else {
            let uuid = Uuid::new_v4().to_string();
            let party1 = 1;
            party_signup = PartySignup {
                number: party1,
                uuid,
            };
        }
    }
    let mut hm = db_mtx.write().unwrap();
    hm.insert(key, serde_json::to_string(&party_signup).unwrap());
    return Json(Ok(party_signup));
}

//refcell, arc

fn main() {
    // let mut my_config = Config::development();
    // my_config.set_port(18001);
    let db: HashMap<TupleKey, String> = HashMap::new();
    let db_mtx = RwLock::new(db);
    //rocket::custom(my_config).mount("/", routes![get, set]).manage(db_mtx).launch();

    /////////////////////////////////////////////////////////////////
    //////////////////////////init signups://////////////////////////
    /////////////////////////////////////////////////////////////////

    let keygen_key = TupleKey {
        first: "signup".to_string(),
        second: "keygen".to_string(),
        third: "".to_string(),
        fourth: "".to_string(),
    };
    let sign_key = TupleKey {
        first: "signup".to_string(),
        second: "sign".to_string(),
        third: "".to_string(),
        fourth: "".to_string(),
    };
    let uuid_keygen = Uuid::new_v4().to_string();
    let uuid_sign = Uuid::new_v4().to_string();

    let party1 = 0;
    let party_signup_keygen = PartySignup {
        number: party1.clone(),
        uuid: uuid_keygen,
    };
    let party_signup_sign = PartySignup {
        number: party1.clone(),
        uuid: uuid_sign,
    };
    {
        let mut hm = db_mtx.write().unwrap();
        hm.insert(
            keygen_key,
            serde_json::to_string(&party_signup_keygen).unwrap(),
        );
        hm.insert(sign_key, serde_json::to_string(&party_signup_sign).unwrap());
    }
    /////////////////////////////////////////////////////////////////
    rocket::ignite()
        .mount("/", routes![get, set, signup_keygen, signup_sign])
        .manage(db_mtx)
        .launch();
}
