use crate::*;

/// Stores a Stack of scopes and also an extra global scope
pub struct ScopeManager
{
    scopes : Vec<Scope>,
    global_scope : Scope
}

impl ScopeManager
{
    pub fn enter_scope(&mut self)
    {
        self.scopes.push(Scope::new());
        println!("{:#?}", self.scopes);
    }

    pub fn leave_scope(&mut self)
    {
        self.scopes.pop();
    }

    pub fn new() -> Self
    {
        Self
        {
            scopes : vec![],
            global_scope : Scope::new(),
        }
    }

    pub fn declare_function_global<S>(&mut self, name : S, _type : &OperandType) -> Result<(), ()>
        where S : AsRef<str>
    {
        let name = name.as_ref().to_string();
        self.global_scope.functions.declare_function(&name, _type)
    }

    pub fn get_variable_manager(&mut self) -> &mut VariableManager
    {   
        if self.scopes.len() == 0
        {
            eprintln!("No Valid Scopes! {:#?}", self.scopes);
            panic!();
        }

        &mut self.scopes.last_mut().expect("Unexpected error").variables
    }
}

/// Stores Functions and Variables
#[derive(Debug)]
pub struct Scope
{
    pub variables: VariableManager,
    pub functions: FunctionManager,
}

impl Scope
{
    pub fn new() -> Self
    {
        Scope { 
            variables: VariableManager::new(), 
            functions: FunctionManager::new()
        }
    }
}