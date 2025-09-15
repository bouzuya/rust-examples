mod permission;
mod role;
mod user;

use std::{
    cell::LazyCell,
    collections::{BTreeMap, BTreeSet},
};

pub use crate::rbac::permission::Permission;
pub use crate::rbac::role::Role;
pub use crate::rbac::user::User;

pub const ROLE_TO_PERMISSIONS: LazyCell<BTreeMap<Role, BTreeSet<Permission>>> =
    LazyCell::new(|| {
        BTreeMap::<Role, BTreeSet<Permission>>::from_iter([
            (
                Role::Admin,
                BTreeSet::<Permission>::from_iter([
                    Permission::Parent1OutputChild1,
                    Permission::Parent2OutputChild2,
                    Permission::QueryRootParent1,
                    Permission::QueryRootParent2,
                ]),
            ),
            (
                Role::User,
                BTreeSet::<Permission>::from_iter([Permission::QueryRootParent1]),
            ),
        ])
    });

pub const USER_TO_ROLES: LazyCell<BTreeMap<User, BTreeSet<Role>>> = LazyCell::new(|| {
    BTreeMap::<User, BTreeSet<Role>>::from_iter([
        (
            <User as std::str::FromStr>::from_str("admin123")
                .expect("admin123 to be valid as UserId"),
            BTreeSet::from_iter([Role::Admin]),
        ),
        (
            <User as std::str::FromStr>::from_str("user123")
                .expect("user123 to be valid as UserId"),
            BTreeSet::from_iter([Role::User]),
        ),
    ])
});

// pub enum Error {
//     RoleNotFound,
//     PermissionNotFound,
// }

// pub fn user_id_to_permissions(user_id: &User) -> Result<BTreeSet<Permission>, Error> {
//     let roles = match USER_TO_ROLES.get(user_id) {
//         None => return Err(Error::RoleNotFound),
//         Some(roles) => roles.clone(),
//     };

//     let mut permissions = BTreeSet::<Permission>::new();
//     for role in roles {
//         match ROLE_TO_PERMISSIONS.get(&role) {
//             None => return Err(Error::PermissionNotFound),
//             Some(ps) => {
//                 for p in ps {
//                     permissions.insert(*p);
//                 }
//             }
//         }
//     }

//     Ok(permissions)
// }
