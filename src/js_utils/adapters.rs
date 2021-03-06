use crate::js_utils::adapters::proxies::{JsProxy, JsProxyHandle};
use crate::js_utils::facades::{JsNull, JsUndefined, JsValueFacade, JsValueType};
use crate::js_utils::{JsError, Script};

pub mod proxies;

pub trait JsRuntimeAdapter {
    type JsValueAdapterType: JsValueAdapter + Clone;
    type JsPromiseAdapterType: JsPromiseAdapter + Clone;
    type JsRealmAdapterType: JsRealmAdapter;

    fn js_create_realm(&self, id: &str) -> Result<&Self::JsRealmAdapterType, JsError>;
    fn js_get_realm(&self, id: &str) -> Option<&Self::JsRealmAdapterType>;
    fn js_get_main_realm(&self) -> &Self::JsRealmAdapterType;
}

pub trait JsRealmAdapter {
    type JsRuntimeAdapterType: JsRuntimeAdapter;

    fn to_js_value_facade(
        &self,
        js_value: &dyn JsValueAdapter<JsRuntimeAdapterType = Self::JsRuntimeAdapterType>,
    ) -> Box<dyn JsValueFacade> {
        match js_value.js_get_type() {
            JsValueType::I32 => Box::new(js_value.js_to_i32()),
            JsValueType::F64 => Box::new(js_value.js_to_f64()),
            JsValueType::String => Box::new(js_value.js_to_string()),
            JsValueType::Boolean => Box::new(js_value.js_to_bool()),
            JsValueType::Object => {
                todo!();
            }
            JsValueType::Function => {
                todo!();
            }
            JsValueType::BigInt => {
                todo!();
            }
            JsValueType::Promise => {
                todo!();
            }
            JsValueType::Date => {
                todo!();
            }
            JsValueType::Null => Box::new(JsNull {}),
            JsValueType::Undefined => Box::new(JsUndefined {}),

            JsValueType::Array => {
                todo!();
            }
        }
    }

    fn from_js_value_facade(
        &self,
        value_facade: &dyn JsValueFacade,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    > {
        match value_facade.js_get_type() {
            JsValueType::I32 => self.js_i32_create(value_facade.js_as_i32()),
            JsValueType::F64 => self.js_f64_create(value_facade.js_as_f64()),
            JsValueType::String => self.js_string_create(value_facade.js_as_str()),
            JsValueType::Boolean => self.js_boolean_create(value_facade.js_as_bool()),
            JsValueType::Object => {
                todo!()
            }
            JsValueType::Function => {
                todo!()
            }
            JsValueType::BigInt => {
                todo!()
            }
            JsValueType::Promise => {
                todo!()
            }
            JsValueType::Date => {
                todo!()
            }
            JsValueType::Null => self.js_null_create(),
            JsValueType::Undefined => self.js_undefined_create(),
            JsValueType::Array => {
                todo!()
            }
        }
    }

    fn js_eval(
        &self,
        script: Script,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;

    fn js_proxy_install(&self, proxy: JsProxy<Self::JsRuntimeAdapterType>);
    fn js_proxy_instantiate(
        &self,
        namespace: &[&str],
        class_name: &str,
        arguments: &[<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType],
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_proxy_invoke_event(
        &self,
        proxy_handle: &JsProxyHandle,
        event_id: &str,
        event_obj: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
    );

    #[allow(clippy::type_complexity)]
    fn js_install_function(
        &self,
        namespace: &[&str],
        name: &str,
        js_function: fn(
            &Self,
            &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
            &[<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType],
        ) -> Result<<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType, JsError>,
        arg_count: u32,
    ) -> Result<(), JsError>;
    fn js_install_closure<
        F: Fn(
            &Self,
            &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
            &[<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType],
        ) -> Result<<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType, JsError> + 'static,
    >(
        &self,
        namespace: &[&str],
        name: &str,
        js_function: F,
        arg_count: u32,
    ) -> Result<(), JsError>;
    fn js_eval_module(
        &self,
        script: Script,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_get_namespace(
        &self,
        namespace: &[&str],
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    // function methods
    fn js_function_invoke_by_name(
        &self,
        namespace: &[&str],
        method_name: &str,
        args: &[<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType],
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_function_invoke_member_by_name(
        &self,
        this_obj: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        method_name: &str,
        args: &[<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType],
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_function_invoke(
        &self,
        this_obj: Option<&<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType>,
        function_obj: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        args: &[<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType],
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_function_create<
        F: Fn(
            &Self,
            &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
            &[<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType],
        ) -> Result<<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType, JsError> + 'static,
    >(
        &self,
        name: &str,
        js_function: F,
        arg_count: u32,
    ) -> Result<<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType, JsError>;
    //object functions
    fn js_object_delete_property(
        &self,
        object: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        property_name: &str,
    ) -> Result<(), JsError>;
    fn js_object_set_property(
        &self,
        object: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        property_name: &str,
        property: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
    ) -> Result<(), JsError>;

    fn js_object_get_property(
        &self,
        object: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        property_name: &str,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_object_create(
        &self,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_object_construct(
        &self,
        constructor: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        args: &[<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType],
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_object_get_properties(
        &self,
        object: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
    ) -> Result<Vec<String>, JsError>;
    fn js_object_traverse<F, R>(
        &self,
        object: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        visitor: F
    ) -> Result<Vec<R>, JsError> where F: Fn(&str, &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType) -> Result<R, JsError>;
    // array functions
    fn js_array_get_element(
        &self,
        array: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        index: u32,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_array_set_element(
        &self,
        array: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        index: u32,
        element: <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
    ) -> Result<(), JsError>;
    fn js_array_get_length(
        &self,
        array: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
    ) -> Result<u32, JsError>;
    fn js_array_create(
        &self,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_array_traverse<F, R>(
        &self,
        array: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        visitor: F
    ) -> Result<Vec<R>, JsError> where F: Fn(u32, &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType) -> Result<R, JsError>;
    // primitives

    fn js_null_create(
        &self,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_undefined_create(
        &self,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_i32_create(
        &self,
        val: i32,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_string_create(
        &self,
        val: &str,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_boolean_create(
        &self,
        val: bool,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;
    fn js_f64_create(
        &self,
        val: f64,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        JsError,
    >;

    // promises
    fn js_promise_create(
        &self,
    ) -> Result<
        <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsPromiseAdapterType,
        JsError,
    >;

    // cache
    fn js_cache_add(
        &self,
        object: <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
    ) -> i32;
    fn js_cache_dispose(&self, id: i32);
    fn js_cache_with<C, R>(
        &self,
        id: i32,
        consumer: C
    ) -> R where C: FnOnce(&<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType) -> R;
    fn js_cache_consume(
        &self,
        id: i32,
    ) -> <<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType;

    // instanceof
    fn js_instance_of(
        &self,
        object: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
        constructor: &<<Self as JsRealmAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
    ) -> bool;
}

pub trait JsPromiseAdapter {
    type JsRuntimeAdapterType: JsRuntimeAdapter;
    fn js_promise_resolve(
        &self,
        context: &<<Self as JsPromiseAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsRealmAdapterType,
        resolution: &<<Self as JsPromiseAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
    ) -> Result<(), JsError>;
    fn js_promise_reject(
        &self,
        context: &<<Self as JsPromiseAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsRealmAdapterType,
        rejection: &<<Self as JsPromiseAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType,
    ) -> Result<(), JsError>;
    fn js_promise_add_reactions<F>(
        &self,
        context: &<<Self as JsPromiseAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsRealmAdapterType,
        then: Option<F>,
        catch: Option<F>,
        finally: Option<F>,
    ) -> Result<(), JsError> where F: Fn(&<<Self as JsPromiseAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType) -> Result<(), JsError> + 'static;
    fn js_promise_get_value(
        &self,
    ) -> <<Self as JsPromiseAdapter>::JsRuntimeAdapterType as JsRuntimeAdapter>::JsValueAdapterType;
}

pub trait JsValueAdapter {
    type JsRuntimeAdapterType: JsRuntimeAdapter;

    /// js_get_type returns the rust type of the value (more extensive list than javascript typeof)
    fn js_get_type(&self) -> JsValueType;

    fn js_is_null_or_undefined(&self) -> bool {
        self.js_get_type() == JsValueType::Null || self.js_get_type() == JsValueType::Undefined
    }

    /// js_type_of returns the Javascript typeof String
    fn js_type_of(&self) -> &'static str;
    fn js_to_bool(&self) -> bool;
    fn js_to_i32(&self) -> i32;
    fn js_to_f64(&self) -> f64;
    fn js_to_string(&self) -> String;
}
