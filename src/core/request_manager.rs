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

        }
        BCRequestType::InitDatabase => {
            super::address_manager::init_database();
        },
        BCRequestType::CreateTable => {
            let param = request.params.first().expect("Empty params.");
            super::address_manager::create_table(param.value.clone());
        }
    }
    Ok(())
}

pub fn send_request() {

}