fn main() {
    let user_has_paid_for_subs = false;
    let user_is_admin = true;
    let user_has_premium_exp = user_has_paid_for_subs || user_is_admin;
    println!("Can this user see my site? {}", user_has_premium_exp);
}