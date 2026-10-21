use rustdev::auth_util::models::Credentials;
use rustdev::authenticate;

fn main(){
    let cred = Credentials{
        username: String::from("abhinayjangde"),
        password: String::from("wahitohai"),
    };
    authenticate(cred);
}