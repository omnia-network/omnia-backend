use candid::Principal;

pub fn get_principal_from_string(principal_id: String) -> Principal {
  Principal::from_text(principal_id).expect("Principal id not valid")
}