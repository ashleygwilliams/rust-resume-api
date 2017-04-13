use cargonauts;
use tokio_service::NewService;
use cargonauts::format::Debug;

use resources::person::Person;

routes! {
    resource Person {
        method Get in Debug;
        method Index in Debug;
    }
}
