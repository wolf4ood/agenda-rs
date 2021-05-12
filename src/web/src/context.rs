use std::sync::Arc;

use agenda_domain::todos::TodoRepo;

#[derive(Clone)]
pub struct ApplicationContext {
    todos: Arc<dyn TodoRepo>,
}

impl ApplicationContext {
    pub fn new(todos: impl TodoRepo + 'static) -> ApplicationContext {
        ApplicationContext {
            todos: Arc::new(todos),
        }
    }
    pub fn todos(&self) -> &dyn TodoRepo {
        &*self.todos
    }
}
