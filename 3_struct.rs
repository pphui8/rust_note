#[derive(Debug)]
struct User {
    active: bool,
    // not a &str because we want instances of 
    // this struct to own all of its data
    username: String,
    email: String,
    sign_in_count: u64
}

// struct without any fields
struct AlwaysEqual;

// method
impl User {
	// &self equals &self: Self
    fn get_username(&self) -> String {
        String::from(&self.username)
    }

	// method without self paramter
	fn new_user(username: String, email: String) -> User {
		User {
			active: true,
			username,
			email,
			sign_in_count: 1,
		}
	}    
}

fn main() {    
    let pphui8 = build_user(String::from("1216595344@qq.com"), String::from("pphui8"));
    println!("{:#?}", pphui8);
    let pphui89 = build_user_autoinit(pphui8, String::from("pphui89"));
    // & to keep the ownership
    dbg!(&pphui89);

	// call method whitch has no self paramter
	let pphui889 = User::new_user(String::from("pphui889"), String::from("1@qq.com"));
	println!("{}", pphui889.get_username());
}

fn build_user(email: String, username: String) -> User {
    // use email rather than email: email
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_autoinit(model: User, username: String) -> User {
    let tmp =  User {
        username,
        // String in model was moved from here so it`s no longer exsist
        // boolean and integer exsist
        ..model
    };
    // println!("{:#?}", model); //wrong
    println!("{}", model.active); //accurate
    return tmp;
}