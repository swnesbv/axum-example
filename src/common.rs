use std::sync::Arc;
use tera::Tera;
use axum::{
    extract::{FromRef, FromRequestParts},
    http::{request::Parts},
    http::{header::{HeaderMap}},
};
use bb8_postgres::PostgresConnectionManager;
use tokio_postgres::NoTls;

pub type Templates = Arc<Tera>;


pub type PgPool    = bb8::Pool<PostgresConnectionManager<NoTls>>;
pub type RedisPool = bb8::Pool<redis::Client>;


#[derive(Debug)]
pub struct RedisConn(pub bb8::PooledConnection<'static, redis::Client>);
impl<S> FromRequestParts<S> for RedisConn
where
    RedisPool: FromRef<S>,
    S: Send + Sync + Clone + 'static,
{
    type Rejection = String;
    async fn from_request_parts(
        _parts: &mut Parts, state: &S
    ) -> Result<Self, Self::Rejection> {
        let pool = RedisPool::from_ref(state);
        let conn = pool.get_owned().await.unwrap();
        Ok(Self(conn))
    }
}

#[derive(Clone, Debug)]
pub struct DoubleConn {
    pub pool: PgPool,
    pub conn: RedisPool
}

use serde_json::{Value, json};
use std::iter::Enumerate;
use std::str::Chars;
use std::slice::Iter;
use serde_json::map::Iter as MapIter;

pub enum KvIter<'a> {
    StrIter ( Enumerate<Chars<'a>>),
    ArrIter (Enumerate<Iter<'a, Value>>),
    MapIter (MapIter<'a>),
}
impl<'a> Iterator for KvIter<'a> {
    type Item = (Value, Value);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::StrIter(chars) => {
                let (idx, character) = chars.next()?;
                Some((json!(idx), json!(character)))
            }
            Self::ArrIter(values) => {
                let (idx, value) = values.next()?;
                Some((json!(idx), value.clone()))
            }
            Self::MapIter(items) => {
                let (key, value) = items.next()?;
                Some((json!(key), value.clone()))
            }
        }
    }
}
pub trait IntoKvIter {
    fn items(&self) -> Option<KvIter<'_>>;
}
impl IntoKvIter for Value {
    fn items(&self) -> Option<KvIter<'_>> {
        Some(match self {
            Value::String(string) => KvIter::StrIter(string.chars().enumerate()),
            Value::Array(values) => KvIter::ArrIter(values.iter().enumerate()),
            Value::Object(map) => KvIter::MapIter(map.iter()),
            _ => return None,
        })
    }
}


pub fn to_bool(c: &str) -> bool {
    match c {
        "true"  => true,
        "t"     => true,
        "false" => false,
        "f"     => false,
        "yes"   => true,
        "y"     => true,
        "no"    => false,
        "on"    => true,
        "n"     => false,
        "1"     => true,
        "0"     => false,
        _       => panic!(" err..!"),
    }
}
pub fn to_vec_bool(v: Vec<&str>) -> Vec<bool> {
    let mut x: Vec<bool> = vec![];
    v.iter().for_each(|i| {
        match *i {
            "true"  => x.push(true),
            "false" => x.push(false),
            "t"     => x.push(true),
            "f"     => x.push(false),
            "yes"   => x.push(true),
            "y"     => x.push(true),
            "no"    => x.push(false),
            "n"     => x.push(false),
            "1"     => x.push(true),
            "0"     => x.push(false),
            _       => panic!("err.. {:?}", i),
        };
    });
    x
}

pub async fn to_token(
    headers: HeaderMap,
    name: String,
) -> Result<Option<String>, Option<String>> {

    let mut token = String::from("");

    let a = match headers.get("Cookie") {
        Some(expr) => expr,
        None => return Err(None),
    };
    let s: String = match a.to_str() {
        Ok(expr) => expr.to_string(),
        Err(err) => return Err(Some(err.to_string())),
    };
    let ss = s.replace("; ", ";");
    let all: Vec<&str> = ss.split(";").collect();
    all.iter().for_each(|i| {
        if i.split("=").next() == Some(&name) {
            token.push_str(i.split("=").last().unwrap());
        }
    });
    Ok(Some(token))
}



/*fn main() {
    let a = vec!["1", "0", "1", "0"];
    let b = vec![1, 2, 3, 4];
    let c = to_bool(a);
    println!("c = {:?}", c);

    let mut f: Vec<i32> = vec![];
    for (d,e) in c.iter().zip(b.iter()) {
        if *d == true {
            f.push(*e);
        }
    }
    println!("f.. {:?}", f);
}*/