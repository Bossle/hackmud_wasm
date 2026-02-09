extern crate alloc;
use core::iter;
use core::slice::from_raw_parts_mut;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use crate::myalloc::alloc;
use crate::polyfill::SyncUnsafeCell;

pub mod global {
use super::*;
pub const FUNCTION_PROTOTYPE: Function = Function { id: 999 };
pub const OBJECT: Function = Function { id: 999 };
pub const ARRAY: Function = Function { id: 999 };
pub const SYMBOL: Function = Function { id: 999 };
}

static NUMBER_ID_MAP: SyncUnsafeCell<BTreeMap<u64, usize>> = SyncUnsafeCell::new(BTreeMap::new());
static STRING_ID_MAP: SyncUnsafeCell<BTreeMap<String, usize>> = SyncUnsafeCell::new(BTreeMap::new());
static CALLBACKS: SyncUnsafeCell<Vec<Box<dyn FnMut(FunctionContext)->Value>>> = SyncUnsafeCell::new(Vec::new());
const TYPEOF_ID: usize = 0;
const CALLBACK_ID: usize = 1;
const NEW_TARGET_ID: usize = 2;
const ARGUMENTS_ID: usize = 3;
const BUILT_ID: usize = 4;
const NULL_ID: usize = 999;
const UNDEFINED_ID: usize = 999;
const TRUE_ID: usize = 999;
const FALSE_ID: usize = 999;

const OBJECT_CREATE_ID: usize = 999;
const DEFINE_PROPERTY_ID: usize = 999;
const OBJECT_IS_ID: usize = 999;
const IS_EXTENSIBLE_ID: usize = 999;
const IS_NULL_ID: usize = 999;
const GET_OWN_PROPERTY_DESCRIPTOR_ID: usize = 999;
const GET_OWN_PROPERTY_NAMES_ID: usize = 999;
const GET_OWN_PROPERTY_SYMBOLS_ID: usize = 999;
const PREVENT_EXTENSIONS_ID: usize = 999;
const SET_PROTOTYPE_OF_ID:  usize = 999;

const PROTO_GET_CALL_ID: usize = 999;
const BIND_APPLY_ID: usize = 999;
const IS_ARRAY_ID: usize = 999;
const SYMBOL_KEY_FOR_ID: usize = 999;

const EMPTY_ARRAY_ID: usize = 999;
const DEFAULT_DATA_DESCRIPTOR_ID: usize = 999;  // {value: ?, writable: true, enumerable: true, configurable: true}
const DEFAULT_GETTER_DESCRIPTOR_ID: usize = 999;  // {getter: ?, enumerable: true, configurable: true}
const DEFAULT_SETTER_DESCRIPTOR_ID: usize = 999;  // {setter: ?, enumerable: true, configurable: true}
const EMPTY_VALUE_DESCRIPTOR_ID: usize = 999;  // {value: ?}
const EMPTY_GETTER_DESCRIPTOR_ID: usize = 999;  // {getter: ?}
const EMPTY_SETTER_DESCRIPTOR_ID: usize = 999;  // {setter: ?}
const WRITABLE_DESCRIPTOR_ID: usize = 999;  // {writable: true}
const UNWRITABLE_DESCRIPTOR_ID: usize = 999;  // {writable: false}
const ENUMERABLE_DESCRIPTOR_ID: usize = 999;  // {enumerable: true}
const UNENUMERABLE_DESCRIPTOR_ID: usize = 999;  // {enumerable: false}
const UNCONFIGURABLE_DESCRIPTOR_ID: usize = 999;  // {configurable: false}

pub struct Context {
    new_target: Value,
    arguments: Value,
}

pub fn setup() -> Context {
    unsafe {
        assert_eq!(global::FUNCTION_PROTOTYPE.id, getprop(TYPEOF_ID, id_from_str("__proto__")));
        let object_proto_id = getprop(global::FUNCTION_PROTOTYPE.id, id_from_str("__proto__"));
        assert_eq!(global::OBJECT.id, getprop(object_proto_id, id_from_str("constructor")));

        assert_eq!(OBJECT_CREATE_ID, getprop(global::OBJECT.id, id_from_str("create")));
        assert_eq!(DEFINE_PROPERTY_ID, getprop(global::OBJECT.id, id_from_str("defineProperty")));
        assert_eq!(OBJECT_IS_ID, getprop(global::OBJECT.id, id_from_str("is")));
        assert_eq!(IS_EXTENSIBLE_ID, getprop(global::OBJECT.id, id_from_str("isExtensible")));
        assert_eq!(IS_NULL_ID, getprop(global::OBJECT.id, id_from_str("isNull")));
        assert_eq!(GET_OWN_PROPERTY_DESCRIPTOR_ID, getprop(global::OBJECT.id, id_from_str("getOwnPropertyDescriptor")));
        assert_eq!(GET_OWN_PROPERTY_NAMES_ID, getprop(global::OBJECT.id, id_from_str("getOwnPropertyNames")));
        assert_eq!(GET_OWN_PROPERTY_SYMBOLS_ID, getprop(global::OBJECT.id, id_from_str("getOwnPropertySymbols")));
        assert_eq!(PREVENT_EXTENSIONS_ID, getprop(global::OBJECT.id, id_from_str("preventExtensions")));
        assert_eq!(SET_PROTOTYPE_OF_ID, getprop(global::OBJECT.id, id_from_str("setPrototypeOf")));

        let proto_desc = call2(GET_OWN_PROPERTY_DESCRIPTOR_ID, object_proto_id, id_from_str("__proto__"));
        let proto_getter = getprop(proto_desc, id_from_str("get"));
        assert_eq!(PROTO_GET_CALL_ID, getprop(proto_getter, id_from_str("call")));
        let func_symbols = call1(GET_OWN_PROPERTY_SYMBOLS_ID, global::FUNCTION_PROTOTYPE.id);
        let a_symbol = getprop(func_symbols, id_from_number(0.0));
        let bind = getprop(TYPEOF_ID, id_from_str("bind"));
        assert_eq!(BIND_APPLY_ID, getprop(bind, id_from_str("apply")));

        assert_eq!(global::ARRAY.id, getprop(func_symbols, id_from_str("constructor")));
        assert_eq!(global::SYMBOL.id, getprop(a_symbol, id_from_str("constructor")));
        assert_eq!(NULL_ID, getprop(object_proto_id, id_from_str("__proto__")));
        assert_eq!(UNDEFINED_ID, getprop(TYPEOF_ID, id_from_str("undefined")));
        assert_eq!(FALSE_ID, getprop(proto_desc, id_from_str("enumerable")));
        assert_eq!(TRUE_ID, call1(IS_EXTENSIBLE_ID, proto_getter));

        assert_eq!(IS_ARRAY_ID, getprop(global::ARRAY.id, id_from_str("isArray")));
        assert_eq!(SYMBOL_KEY_FOR_ID, getprop(global::SYMBOL.id, id_from_str("keyFor")));
        assert_eq!(EMPTY_ARRAY_ID, call0(global::ARRAY.id));

        assert_eq!(DEFAULT_DATA_DESCRIPTOR_ID, call1(OBJECT_CREATE_ID, NULL_ID));
        setprop(DEFAULT_DATA_DESCRIPTOR_ID, id_from_str("writable"), TRUE_ID);
        setprop(DEFAULT_DATA_DESCRIPTOR_ID, id_from_str("enumerable"), TRUE_ID);
        setprop(DEFAULT_DATA_DESCRIPTOR_ID, id_from_str("configurable"), TRUE_ID);
        assert_eq!(DEFAULT_GETTER_DESCRIPTOR_ID, call1(OBJECT_CREATE_ID, NULL_ID));
        setprop(DEFAULT_GETTER_DESCRIPTOR_ID, id_from_str("enumerable"), TRUE_ID);
        setprop(DEFAULT_GETTER_DESCRIPTOR_ID, id_from_str("configurable"), TRUE_ID);
        assert_eq!(DEFAULT_SETTER_DESCRIPTOR_ID, call1(OBJECT_CREATE_ID, NULL_ID));
        setprop(DEFAULT_SETTER_DESCRIPTOR_ID, id_from_str("enumerable"), TRUE_ID);
        setprop(DEFAULT_SETTER_DESCRIPTOR_ID, id_from_str("configurable"), TRUE_ID);
        assert_eq!(EMPTY_VALUE_DESCRIPTOR_ID, call1(OBJECT_CREATE_ID, NULL_ID));
        assert_eq!(EMPTY_GETTER_DESCRIPTOR_ID, call1(OBJECT_CREATE_ID, NULL_ID));
        assert_eq!(EMPTY_SETTER_DESCRIPTOR_ID, call1(OBJECT_CREATE_ID, NULL_ID));
        assert_eq!(WRITABLE_DESCRIPTOR_ID, call1(OBJECT_CREATE_ID, NULL_ID));
        setprop(WRITABLE_DESCRIPTOR_ID, id_from_str("writable"), TRUE_ID);
        assert_eq!(UNWRITABLE_DESCRIPTOR_ID, call1(OBJECT_CREATE_ID, NULL_ID));
        setprop(UNWRITABLE_DESCRIPTOR_ID, id_from_str("writable"), FALSE_ID);
        assert_eq!(ENUMERABLE_DESCRIPTOR_ID, call1(OBJECT_CREATE_ID, NULL_ID));
        setprop(ENUMERABLE_DESCRIPTOR_ID, id_from_str("enumerable"), TRUE_ID);
        assert_eq!(UNENUMERABLE_DESCRIPTOR_ID, call1(OBJECT_CREATE_ID, NULL_ID));
        setprop(UNENUMERABLE_DESCRIPTOR_ID, id_from_str("enumerable"), FALSE_ID);
        assert_eq!(UNCONFIGURABLE_DESCRIPTOR_ID, call1(OBJECT_CREATE_ID, NULL_ID));
        setprop(UNCONFIGURABLE_DESCRIPTOR_ID, id_from_str("configurable"), FALSE_ID);

        Context {
            new_target: value_from_id(NEW_TARGET_ID),
            arguments: value_from_id(ARGUMENTS_ID),
        }
    }
}

#[derive(Clone, Debug)]
pub enum AnyObject {
    Plain(PlainObject),
    Function(Function),
    Array(Array),
}

#[derive(Clone, Debug)]
pub struct Symbol {
    id: usize,
    name: Cow<'static, str>,
    global: bool
}

#[derive(Clone, Debug)]
pub enum Value {
    Null,
    Undefined,
    Boolean(bool),
    Number(f64),  // todo: investigate Infinity, NaN
    String(Cow<'static, str>),
    Symbol(Symbol),
    Object(AnyObject)
}

pub fn eq(a: &Value, b: &Value) -> bool {
    unsafe {
        get_bool(call2(OBJECT_IS_ID, id_from_value(a), id_from_value(b)))
    }
}

#[derive(Clone, Debug)]
pub struct FunctionContext {
    pub args: Array,
    pub is_new: bool,
}

#[derive(Clone, Debug)]
pub enum ObjectKey {
    String(Cow<'static, str>),
    Symbol(Symbol)
}

#[derive(Clone, Debug)]
pub struct DataDescriptor {
    id: usize,
    pub value: Value,
    pub writable: bool,
    pub enumerable: bool,
    pub configurable: bool
}

#[derive(Clone, Debug)]
pub struct AccessorDescriptor {
    id: usize,
    pub get: Option<Function>,
    pub set: Option<Function>,
    pub enumerable: bool,
    pub configurable: bool
}

#[derive(Clone, Debug)]
pub enum PropertyDescriptor {
    Data(DataDescriptor),
    Accessor(AccessorDescriptor),
}

trait Object {
    unsafe fn id(&self) -> usize;

    fn get_prototype(&self) -> Value {
        unsafe {
            value_from_id(call1(PROTO_GET_CALL_ID, self.id()))
        }
    }

    fn set_prototype(&self, prototype: &AnyObject) {
        unsafe {
            call2(SET_PROTOTYPE_OF_ID, self.id(), prototype.id());
        }
    }

    fn unset_prototype(&self) {
        unsafe {
            call2(SET_PROTOTYPE_OF_ID, self.id(), NULL_ID);
        }
    }

    fn get_property_names(&self) -> Array {
        unsafe {
            Array { id: call1(GET_OWN_PROPERTY_NAMES_ID, self.id()) }
        }
    }

    fn get_property_symbols(&self) -> Array {
        unsafe {
            Array { id: call1(GET_OWN_PROPERTY_SYMBOLS_ID, self.id()) }
        }
    }

    fn has_key(&self, key: &ObjectKey) -> bool {
        unsafe {
            get_bool(call2(GET_OWN_PROPERTY_DESCRIPTOR_ID, self.id(), id_from_object_key(key)))
        }
    }

    fn get_value(&self, key: &ObjectKey) -> Value {
        unsafe {
            value_from_id(getprop(self.id(), id_from_object_key(key)))
        }
    }

    fn get_descriptor(&self, key: &ObjectKey) -> Option<PropertyDescriptor> {
        unsafe {
            let id = call2(GET_OWN_PROPERTY_DESCRIPTOR_ID, self.id(), id_from_object_key(key));
            if get_bool(id) {
                Some(descriptor_from_id(id))
            } else {
                None
            }
        }
    }

    /**
     * Note that setters may throw TypeErrors if attempted in non-configurable properties or non-extensible objects,
     * those are not caught by these functions, you should check for their validity yourself.
     * Also, calling set_writability / set_enumerability / set_non_configurable on non-existing keys has weird behavior,
     * which should not be relied upon
     */

    fn set_value(&self, key: &ObjectKey, value: &Value) {
        unsafe {
            let desc_id = if self.has_key(key) { EMPTY_VALUE_DESCRIPTOR_ID } else { DEFAULT_DATA_DESCRIPTOR_ID };
            setprop(desc_id, id_from_str("value"), id_from_value(value));
            call3(DEFINE_PROPERTY_ID, self.id(), id_from_object_key(key), desc_id);
        }
    }

    fn set_writability(&self, key: &ObjectKey, writable: bool) {
        unsafe {
            call3(DEFINE_PROPERTY_ID, self.id(), id_from_object_key(key),
                if writable { WRITABLE_DESCRIPTOR_ID } else { UNWRITABLE_DESCRIPTOR_ID });
        }
    }

    fn set_getter(&self, key: &ObjectKey, getter: Option<&Function>) {
        unsafe {
            let desc_id = if self.has_key(key) { EMPTY_GETTER_DESCRIPTOR_ID } else { DEFAULT_GETTER_DESCRIPTOR_ID };
            setprop(desc_id, id_from_str("get"), match getter {
                Some(g) => g.id,
                None => UNDEFINED_ID
            });
            call3(DEFINE_PROPERTY_ID, self.id(), id_from_object_key(key), desc_id);
        }
    }

    fn set_setter(&self, key: &ObjectKey, setter: Option<&Function>) {
        unsafe {
            let desc_id = if self.has_key(key) { EMPTY_SETTER_DESCRIPTOR_ID } else { DEFAULT_SETTER_DESCRIPTOR_ID };
            setprop(desc_id, id_from_str("set"), match setter {
                Some(s) => s.id,
                None => UNDEFINED_ID
            });
            call3(DEFINE_PROPERTY_ID, self.id(), id_from_object_key(key), desc_id);
        }
    }

    fn set_enumerability(&self, key: &ObjectKey, enumerable: bool) {
        unsafe {
            call3(DEFINE_PROPERTY_ID, self.id(), id_from_object_key(key),
                if enumerable { ENUMERABLE_DESCRIPTOR_ID } else { UNENUMERABLE_DESCRIPTOR_ID });
        }
    }

    fn set_not_configurable(&self, key: &ObjectKey) {
        unsafe {
            call3(DEFINE_PROPERTY_ID, self.id(), id_from_object_key(key), UNCONFIGURABLE_DESCRIPTOR_ID);
        }
    }

    fn delete(&self, key: &ObjectKey) -> bool {
        unsafe {
            delprop(self.id(), id_from_object_key(key))
        }
    }

    fn is_extensible(&self) -> bool {
        unsafe {
            get_bool(call1(IS_EXTENSIBLE_ID, self.id()))
        }
    }

    fn prevent_extensions(&self) {
        unsafe {
            call1(PREVENT_EXTENSIONS_ID, self.id());
        }
    }
}

#[derive(Clone, Debug)]
pub struct PlainObject {
    id: usize
}

impl Object for PlainObject {
    unsafe fn id(&self) -> usize {
        self.id
    }
}

impl PlainObject {
    pub fn new() -> PlainObject {
        PlainObject { id: unsafe{call1(OBJECT_CREATE_ID, NULL_ID)} }
    }
}

#[derive(Clone, Debug)]
pub struct Function {
    id: usize
}

impl Object for Function {
    unsafe fn id(&self) -> usize {
        self.id
    }
}

impl Function {
    pub fn new<F: FnMut(FunctionContext)->Value + 'static>(f: F) -> Function {
        unsafe {
            (*CALLBACKS.get()).push(Box::new(f));
            Function{ id: CALLBACK_ID }.bind(&Value::Undefined, &[&Value::Number(((*CALLBACKS.get()).len()-1) as f64)])
        }
    }

    pub fn call(&self, args: &[&Value], new: bool) -> Result<Value, PlainObject> {
        unsafe {
            let ret = match args.len() {
                0 => trya(self.id, EMPTY_ARRAY_ID, new),
                _ => trya(self.id, args.iter().copied().collect::<Array>().id, new)
            };
            if get_bool(ret) {
                Ok(value_from_id(ret+1))
            } else {
                Err(PlainObject { id: ret+1 })
            }
        }
    }

    pub fn apply(&self, args: Array, new: bool) -> Result<Value, PlainObject> {
        unsafe {
            let ret = trya(self.id, args.id, new);
            if get_bool(ret) {
                Ok(value_from_id(ret+1))
            } else {
                Err(PlainObject { id: ret+1 })
            }
        }
    }

    pub fn bind(&self, this: &Value, args: &[&Value]) -> Function {
        unsafe {
            let arr = Array::from_iter(
                iter::once(id_from_value(this))
                .chain(ids_from_values(args.iter().copied()))
            );
            Function { id: call2(BIND_APPLY_ID, self.id, arr.id) }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Array {
    id: usize
}

impl Object for Array {
    unsafe fn id(&self) -> usize {
        self.id
    }
}

impl Array {
    pub fn new() -> Array {
        unsafe {
            Array { id: call0(global::ARRAY.id) }
        }
    }
}

impl Array {
    fn from_iter<T: IntoIterator<Item = usize>>(iter: T) -> Self {
        unsafe {
            let mut v = iter.into_iter().map(|x: usize| x as u32).collect::<Vec<u32>>();
            Array { id: putarray(v.as_mut_ptr(), v.len()) }
        }
    }
}

impl<'a> FromIterator<&'a Value> for Array {
    fn from_iter<T: IntoIterator<Item = &'a Value>>(iter: T) -> Self {
        Array::from_iter(ids_from_values(iter))
    }
}

impl Object for AnyObject {
    unsafe fn id(&self) -> usize {
        match self{
            AnyObject::Plain(o) => o.id,
            AnyObject::Function(o) => o.id,
            AnyObject::Array(o) => o.id,
        }
    }
}

pub fn throw(v: &Value) -> ! {
    unsafe {
        error(id_from_value(v))
    }
}

fn value_from_id(id: usize) -> Value {
    unsafe {
        let ts = get_string(call1(TYPEOF_ID, id));
        match ts.as_str() {
            "undefined" => Value::Undefined,
            "object" => {
                if get_bool(call1(IS_NULL_ID, id)) {
                    Value::Null
                } else if get_bool(call1(IS_ARRAY_ID, id)) {
                    Value::Object(AnyObject::Array(Array{id}))
                } else {
                    Value::Object(AnyObject::Plain(PlainObject{id}))
                }
            }
            "boolean" => Value::Boolean(get_bool(id)),
            "number" => Value::Number(getf64(id)),
            "string" => Value::String(get_string(id).into()),
            "symbol" => {
                let name = get_string(getprop(id, id_from_str("description")));
                let key_id = call1(SYMBOL_KEY_FOR_ID, id);
                let key_type = get_string(call1(TYPEOF_ID, key_id));
                Value::Symbol(Symbol {
                    id,
                    name: name.into(),
                    global: match key_type.as_str() {
                        "undefined" => false,
                        "string" => true,
                        _ => panic!("unexpected keyFor {} = {}", id, key_type)
                    }
                })
            }
            "function" => Value::Object(AnyObject::Function(Function{id})),
            _ => panic!("unexpected typeof {} = {}", id, ts)
        }
    }
}

fn descriptor_from_id(id: usize) -> PropertyDescriptor {
    unsafe {
        let wv = value_from_id(getprop(id, id_from_str("writable")));
        match wv {
            Value::Undefined => {
                PropertyDescriptor::Accessor(AccessorDescriptor {
                    id,
                    get: match value_from_id(getprop(id, id_from_str("get"))) {
                        Value::Undefined => None,
                        Value::Object(AnyObject::Function(f)) => Some(f),
                        _ => panic!("unexpected 'get' value in {}", id)
                    },
                    set: match value_from_id(getprop(id, id_from_str("set"))) {
                        Value::Undefined => None,
                        Value::Object(AnyObject::Function(f)) => Some(f),
                        _ => panic!("unexpected 'set' value in {}", id)
                    },
                    enumerable: get_bool(getprop(id, id_from_str("enumerable"))),
                    configurable: get_bool(getprop(id, id_from_str("configurable")))
                })
            }
            Value::Boolean(w) => {
                PropertyDescriptor::Data(DataDescriptor {
                    id,
                    value: value_from_id(getprop(id, id_from_str("value"))),
                    writable: w,
                    enumerable: get_bool(getprop(id, id_from_str("enumerable"))),
                    configurable: get_bool(getprop(id, id_from_str("configurable")))
                })
            }
            _ => panic!("unexpected 'writable' value {:?} in {}", wv, id)
        }
    }
}

fn id_from_value(v: &Value) -> usize {
    match v {
        Value::Null => NULL_ID,
        Value::Undefined => UNDEFINED_ID,
        Value::Boolean(b) => id_from_bool(*b),
        Value::Number(n) => id_from_number(*n),
        Value::String(s) => id_from_string(s),
        Value::Symbol(symbol) => symbol.id,
        Value::Object(object) => unsafe{object.id()},
    }
}

fn ids_from_values<'a, T: IntoIterator<Item = &'a Value>>(iter: T) -> impl IntoIterator<Item = usize> {
    iter.into_iter().map(id_from_value)
}

fn id_from_object_key(k: &ObjectKey) -> usize {
    match k {
        ObjectKey::String(s) => id_from_string(s),
        ObjectKey::Symbol(symbol) => symbol.id,
    }
}

fn id_from_bool(b: bool) -> usize {
    match b {
        true => TRUE_ID,
        false => FALSE_ID
    }
}

fn id_from_number(n: f64) -> usize {
    unsafe {*(&mut (*NUMBER_ID_MAP.get())).entry(n.to_bits()).or_insert_with(|| putf64(n))}
}

fn id_from_string(s: &Cow<'static, str>) -> usize {
    unsafe {*(&mut (*STRING_ID_MAP.get())).entry(s.clone().into_owned()).or_insert_with(|| put_string(s))}
}

fn id_from_str(s: &'static str) -> usize {
    id_from_string(&Cow::Borrowed(s))
}

#[no_mangle]
pub unsafe extern "C" fn callback(i: usize, args: usize, is_new: bool) -> usize {
    id_from_value(&((&mut (*CALLBACKS.get()))[i](FunctionContext {
        args: Array { id: args },
        is_new: is_new,
    }).clone()))
}

#[link(wasm_import_module = "i")]
extern "C" {
    fn getf64(id: usize) -> f64;
    fn putf64(val: f64) -> usize;
    fn putstr(loc: *const u16, len: usize) -> usize;
    fn getstr(id: usize, loc: *mut u16);
    fn getlen(id: usize) -> usize;
    fn getprop(id: usize, arg: usize) -> usize;
    fn setprop(id: usize, arg: usize, i: usize);
    fn delprop(id: usize, arg: usize) -> bool;
    fn call0(id: usize) -> usize;
    fn call1(id: usize, arg1: usize) -> usize;
    fn call2(id: usize, arg1: usize, arg2: usize) -> usize;
    fn call3(id: usize, arg1: usize, arg2: usize, arg3: usize) -> usize;
    fn error(id: usize) -> !;
    fn trya(id: usize, args: usize, n: bool) -> usize;
    fn putarray(loc: *mut u32, len: usize) -> usize;
}

fn get_string(id: usize) -> String {
    unsafe {
        let len = getlen(id);
        let p = alloc(2*len, 2) as *mut u16;
        getstr(id, p);
        let b: &[u16] = from_raw_parts_mut(p, len);
        String::from_utf16_lossy(b)
    }
}

fn put_string(s: &Cow<'static, str>) -> usize {
    unsafe {
        let b: Vec<u16> = s.encode_utf16().collect();
        putstr(b.as_ptr(), b.len())
    }
}

fn get_bool(id: usize) -> bool {
    unsafe {
        getf64(id) != 0.0
    }
}

#[panic_handler]
pub unsafe fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    throw(&Value::String(_panic.to_string().into()))
}
