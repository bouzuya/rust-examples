pub struct Error1;

impl crate::MyErrorTrait for Error1 {
    fn into_public_error(self) -> crate::PublicError {
        crate::PublicError("error1".to_owned())
    }
}
