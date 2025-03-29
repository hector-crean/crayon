 Shared state:

 1. wasm bindgen globals
 #[wasm_bindgen]
 pub static mut GLOBAL_STATE: i32 = 0;

 2. thread local@! with RefCell
 thread_local! {
     static GLOBAL_STATE: RefCell<Vec<String>> = RefCell::new(Vec::new());
 }

 3. Closure-based state:
 #[wasm_bindgen]
 pub fn create_state() -> Box<dyn Fn() -> i32> {
     let mut state = 0;
     Box::new(move || {
         state += 1;
         state
     })
 }
 4. Dom storage
 use web_sys::window;
 
 fn set_local_storage(key: &str, value: &str) {
     if let Some(window) = window() {
         if let Ok(Some(storage)) = window.local_storage() {
             let _ = storage.set_item(key, value);
         }
     }
 }
 
5. Custom object with methods:
 #[wasm_bindgen]
 pub struct State {
     value: i32,
 }
 
 #[wasm_bindgen]
 impl State {
     #[wasm_bindgen(constructor)]
     pub fn new() -> State {
         State { value: 0 }
     }
 
     pub fn increment(&mut self) {
         self.value += 1;
     }
 
     pub fn get_value(&self) -> i32 {
         self.value
     }
 }