use crate::BddWorld;
use cucumber::then;
use flyconomy::model::commands::{SellLandingRightsError, SellPlaneError};

#[then("I should get a NotExist error")]
async fn i_should_get_a_not_exist_error(w: &mut BddWorld) {
    match &w.last_result {
        Err(err) => {
            if let Some(specific_err) = err.downcast_ref::<SellPlaneError>() {
                if !matches!(*specific_err, SellPlaneError::NotExist) {
                    panic!("Expected a NotExist error but got a different error");
                }
            } else if let Some(specific_err) = err.downcast_ref::<SellLandingRightsError>() {
                if !matches!(*specific_err, SellLandingRightsError::NotExist) {
                    panic!("Expected a NotExist error, but got a different type");
                }
            } else {
                panic!("Expected a NotExist error but got a different error type");
            }
        }
        _ => panic!("Expected an error but got a successful result"),
    }
}
