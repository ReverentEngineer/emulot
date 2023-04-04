use crate::Error;

/// A trait for interpreting into command args
pub(crate) trait AsArgs {

    /// Format into args
    fn as_args(&self) -> Result<Vec<String>, Error>;

}

impl<T> AsArgs for Option<T> where T: AsArgs {

    fn as_args(&self) -> Result<Vec<String>, Error> {
        match self {
            Some(args) => args.as_args(),
            None => Ok(Vec::new())
        }
    }

}

impl<T> AsArgs for Vec<T> where T: AsArgs {

    fn as_args(&self) -> Result<Vec<String>, Error> {
        Ok(self.into_iter().map(|args| args.as_args()).collect::<Result<Vec<_>, Error>>()?.into_iter().flatten().collect())
    }

}
