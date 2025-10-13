pub struct Error2;

impl crate::MyErrorTrait for Error2 {
    fn into_public_error(self) -> crate::PublicError {
        crate::PublicError("error2".to_owned())
    }
}
