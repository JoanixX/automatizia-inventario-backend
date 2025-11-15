use anyhow::Result;

pub fn hash_password(password: &str) -> Result<String> {
    let hash = bcrypt::hash(password, bcrypt::DEFAULT_COST)?;
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let is_valid = bcrypt::verify(password, hash)?;
    Ok(is_valid)
}
