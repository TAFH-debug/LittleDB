pub struct ModuleRequest<E> {
    pub value: E,
    pub params: Vec<Parameter>
}

pub struct Parameter {
    pub name: String,
    pub value: String
}

impl Parameter {
    pub fn new(name: &str, value: &str) -> Parameter {
        Parameter {
            name: name.to_string(),
            value: value.to_string()
        }
    }
}

impl <E> ModuleRequest<E> {
    pub fn new(value: E, params: Vec<Parameter>) -> ModuleRequest<E> {
        ModuleRequest {
            value,
            params
        }
    }
}