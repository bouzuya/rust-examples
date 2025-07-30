fn main() {
    // pub fn downcast<T: Error + 'static>(self: Box<Self>) -> Result<Box<T>, Box<dyn Error>> {
    //     if self.is::<T>() {
    //         unsafe {
    //             let raw: *mut dyn Error = Box::into_raw(self);
    //             Ok(Box::from_raw(raw as *mut T))
    //         }
    //     } else {
    //         Err(self)
    //     }
    // }

    // pub fn is<T: Error + 'static>(&self) -> bool {
    //     // Get `TypeId` of the type this function is instantiated with.
    //     let t = TypeId::of::<T>();
    //     // Get `TypeId` of the type in the trait object (`self`).
    //     let concrete = self.type_id(private::Internal);
    //     // Compare both `TypeId`s on equality.
    //     t == concrete
    // }

    // fn type_id(&self, _: private::Internal) -> TypeId
    // where
    //     Self: 'static,
    // {
    //     TypeId::of::<Self>()
    // }

    #[derive(Debug, thiserror::Error)]
    enum MyError {
        #[error("invalid input")]
        InvalidInput,
        #[error("not found")]
        NotFound,
    }

    let b2: Box<dyn std::error::Error> = Box::new(MyError::InvalidInput);
    match b2.downcast::<MyError>() {
        // e: Box<MyError>
        Ok(e) => println!(
            "{}",
            match *e {
                MyError::InvalidInput => "downcasted to MyError::InvalidInput",
                MyError::NotFound => "downcasted to MyError::NotFound",
            }
        ),
        // _e: Box<dyn std::error::Error>
        Err(_e) => unreachable!(),
    }

    let b2: Box<dyn std::error::Error> = Box::new(MyError::NotFound);
    match b2.downcast::<std::io::Error>() {
        // _e: Box<std::io::Error>
        Ok(_e) => unreachable!(),
        // e: Box<dyn std::error::Error>
        Err(e) => println!("Failed to downcast: {}", e),
    }
}
