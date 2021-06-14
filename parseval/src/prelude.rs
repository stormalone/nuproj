#[macro_export]
macro_rules! return_err {
    ($expr:expr) => {
        match $expr {
            Err(_) => return,
            Ok(expr) => expr,
        };
    };
}

pub(crate) use bigdecimal::BigDecimal;
pub(crate) use nu_engine::CommandArgs;
pub(crate) use nu_engine::Example;
pub(crate) use nu_source::{SpannedItem, Tag};
pub(crate) use nu_stream::{ActionStream, InputStream};

#[allow(clippy::wrong_self_convention)]
pub trait FromInputStream {
    fn from_input_stream(self) -> ActionStream;
}

impl<T> FromInputStream for T
where
    T: Iterator<Item = nu_protocol::Value> + Send + Sync + 'static,
{
    fn from_input_stream(self) -> ActionStream {
        ActionStream {
            values: Box::new(self.map(nu_protocol::ReturnSuccess::value)),
        }
    }
}
