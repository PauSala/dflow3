use dotenv_codegen::dotenv;
use jsonwebtoken::Algorithm;
use rocket_jwt::jwt;

static SECRET_KEY: &'static str = dotenv!("SECRET_KEY");
#[jwt(SECRET_KEY)]
pub struct UserClaim {
    pub(crate) id: String,
}

#[jwt("secret", exp = 100)]
pub struct UserClaimExp {
    id: String,
}

#[jwt("secret", leeway = 10)]
pub struct UserClaimLeeway {
    id: String,
}

// get token from request query, key is `token`
#[jwt("secret", query = "token")]
pub struct UserClaimQuery {
    id: String,
}
