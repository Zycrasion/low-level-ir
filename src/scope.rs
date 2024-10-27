use crate::*;

/// Stores a Stack of scopes and also an extra global scope
pub struct ScopeManager
{
    scopes : Vec<Scope>,
    global_scope : Scope
}

impl Default for ScopeManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ScopeManager
{
    pub fn enter_scope(&mut self)
    {
        self.scopes.push(Scope::new());
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

    pub fn declare_function_global<S>(&mut self, name : S, _type : &OperandType, params : &[OperandType])
        where S : AsRef<str>
    {
        let name = name.as_ref().to_string();
        self.global_scope.functions.declare_function(&name, _type, params);
    }

    pub fn get_function<S>(&self, name : S) -> Option<(OperandType, Vec<OperandType>)>
    where S : AsRef<str>
    {
        self.global_scope.functions.get_function_type(&name.as_ref().to_string())
    }

    pub fn get_variable_manager(&mut self) -> &mut VariableManager
    {   
        if self.scopes.is_empty()
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

impl Default for Scope {
    fn default() -> Self {
        Self::new()
    }
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