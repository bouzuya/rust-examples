use non_exhaustive1::CreateUserOutput1;
use non_exhaustive1::CreateUserOutput2;

struct CreateUserResponseBody {
    id: String,
    name: String,
}

impl From<CreateUserOutput1> for CreateUserResponseBody {
    fn from(CreateUserOutput1 { id, name }: CreateUserOutput1) -> Self {
        Self { id, name }
    }
}

impl From<CreateUserOutput2> for CreateUserResponseBody {
    // .. なしではコンパイルエラーになる (フィールドが増える可能性があるため)
    // fn from(CreateUserOutput2 { id, name }: CreateUserOutput2) -> Self {
    fn from(CreateUserOutput2 { id, name, .. }: CreateUserOutput2) -> Self {
        Self { id, name }
    }
}

impl From<CreateUserOutput0> for CreateUserResponseBody {
    fn from(CreateUserOutput0 { id, name }: CreateUserOutput0) -> Self {
        Self { id, name }
    }
}

#[non_exhaustive]
pub struct CreateUserOutput0 {
    pub id: String,
    pub name: String,
}

fn main() {
    let output = CreateUserOutput1 {
        id: "123".to_owned(),
        name: "Alice".to_owned(),
    };
    let response_body = CreateUserResponseBody::from(output);

    let CreateUserResponseBody { id, name } = response_body;
    assert_eq!(id, "123");
    assert_eq!(name, "Alice");

    // フィールドを指定して construct できない (フィールドが増える可能性があるため)
    // let output = CreateUserOutput2 {
    //     id: "456".to_owned(),
    //     name: "Bob".to_owned(),
    // };
    let output = CreateUserOutput2::new("456".to_owned(), "Bob".to_owned());
    let response_body = CreateUserResponseBody::from(output);

    let CreateUserResponseBody { id, name } = response_body;
    assert_eq!(id, "456");
    assert_eq!(name, "Bob");
}
