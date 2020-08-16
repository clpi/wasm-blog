use std::{
    fmt::Debug,
};
use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple
};

pub trait Token {  }
pub trait Date: Token {  }
pub trait Person: Token { }

pub fn dump<T: Debug>(result: IResult<&str, T>) {
    match result {
        Ok((rest, val)) => {
            println!("Done: {:?}, {:?}", rest, val)
        },
        Err(err) => {
            println!("Error: {:?}", err)
        },
    }
}

pub type VisitResult<T> = Result<T, Box<std::error::Error>>;

pub struct VarRef<T> { name: String, ref_type: T }

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
pub enum ParseType {
    
}

pub struct Node {
    pub children: Vec<Self>,
    pub entry: ParseType,
}

impl Node {
    pub fn new(node: ParseType) -> Self {
        Self { children: Vec::new(), entry: node }
    }
}

pub trait Visitor {
    
    //fn visit_ref<T>(&mut self, _t: &mut VarRef<T>) -> VisitResult<()>;
}

pub trait CanVisit {
    fn visit(&mut self, visitor: &mut dyn Visitor) -> VisitResult<()>;
}

pub mod macros {

    #[macro_export]
    macro_rules! create_type {
        () => {};
        ($($name:ident, $type:tt),+) => { ( 
            pub struct $name {

            }
        )+ }
    }

    #[macro_export]
    macro_rules! impl_token {
        ($($token:ty, $logic:block),+) => { (
            impl Token for $token { }
        )+ };
        ($($token:ty,  $type:ident),+) => { (
            impl Token for $token {  }

            impl $ident for $token {  }
        )+ };
    }

}
