use openssl::{
    md_ctx::MdCtx,
    md::Md
};
use crate::Error;

pub struct MessageDigest {
    context: MdCtx  
}

impl MessageDigest {

    /// New instance of a message digest using the specified algorithm
    ///
    /// The algorithm refers to an OpenSSL algorithm name
    pub fn new<S: AsRef<str>>(algorithm: S) -> Result<Self, Error> {
        let md = Md::fetch(None, algorithm.as_ref(), None)?;
        let mut context = MdCtx::new()?;
        context.digest_init(&md)?;
        Ok(Self {
            context
        })
    }

    /// Add more input to the digest
    pub fn update(&mut self, input: &[u8]) -> Result<(), Error> {
        self.context.digest_update(input)?;
        Ok(())
    }

    /// Finalize the digest, returning the result
    pub fn r#final(mut self) -> Result<Vec<u8>, Error> {
        let mut output = Vec::new();
        output.resize(self.context.size(), 0);
        self.context.digest_final(&mut output)?;
        Ok(output)
    }
   
    /// Caculate hash of input using provided algorithm
    pub fn calculate<S: AsRef<str>>(algorithm: S, input: &[u8]) -> Result<Vec<u8>, Error> {
        let mut md = Self::new(algorithm)?;
        md.update(input)?;
        md.r#final()
    }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sha256() {
        let digest = MessageDigest::calculate("SHA256", &[]).unwrap();
        assert_eq!(digest, [
            0xe3, 0xb0, 0xc4, 0x42, 0x98, 0xfc, 0x1c, 0x14, 0x9a, 0xfb,
            0xf4, 0xc8, 0x99, 0x6f, 0xb9, 0x24, 0x27, 0xae, 0x41, 0xe4,
            0x64, 0x9b, 0x93, 0x4c, 0xa4, 0x95, 0x99, 0x1b, 0x78, 0x52,
            0xb8, 0x55 
        ])
    }

}
