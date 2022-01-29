use super::inter::*;
use std::result::Result;
use std::ops::Deref;

pub enum BCRequestType {
    CreateTable,
    InitDatabase,
    CreateObject
}

pub fn accept(request: ModuleRequest<BCRequestType>) -> Result<(), ()> {
    match request.value {
        BCRequestType::CreateObject => {

        },
        BCRequestType::InitDatabase => unsafe {
            super::address_manager::init_database();
        },
        BCRequestType::CreateTable => unsafe {
            let param = request.params.first().expect("Empty params.");
            let param2 = request.params.get(1).expect("Params must be two.");
            super::create_table(param.value.clone(), param2.value.clone());
        }
    }
    Ok(())
}

pub fn send_request() {

}