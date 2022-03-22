mod result;

use std::borrow::Cow;
pub use result::*;

use crate::element::Element;
use crate::{string_element, object_element};

pub struct NoArgFunction {
    pub name: String,
    namespace: Cow<'static, str>,
    pub allow_as_symbol: bool,
}

impl NoArgFunction {
    pub fn new_with_config(name: String, namespace: String, allow_as_symbol: bool) -> NoArgFunction {
        NoArgFunction {
            name, namespace: Cow::Owned(namespace), allow_as_symbol,
        }
    }

    pub fn new_with_allow_as_symbol(name: String, allow_as_symbol: bool) -> NoArgFunction {
        NoArgFunction {
            name, namespace: Cow::Borrowed("minecraft"), allow_as_symbol,
        }
    }

    pub fn new(name: String) -> NoArgFunction {
        NoArgFunction::new_with_allow_as_symbol(name, false)
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn namespace(&self) -> &str { &self.namespace }
    pub fn allow_as_symbol(&self) -> bool { self.allow_as_symbol }
}

pub struct OneArgFunction {
    pub name: String,
    namespace: Cow<'static, str>,
    argument_key: Cow<'static, str>,
    pub allow_method_syntax: bool,
}

impl OneArgFunction {
    pub fn new_with_config(name: String, namespace: String, argument_key: String, allow_method_syntax: bool) -> OneArgFunction {
        OneArgFunction {
            name,
            namespace: Cow::Owned(namespace),
            argument_key: Cow::Owned(argument_key),
            allow_method_syntax,
        }
    }

    pub fn new(name: String, allow_method_syntax: bool) -> OneArgFunction {
        OneArgFunction {
            name,
            namespace: Cow::Borrowed("minecraft"),
            argument_key: Cow::Borrowed("argument"),
            allow_method_syntax,
        }
    }

    pub fn new_with_method_syntax(name: String) -> OneArgFunction {
        OneArgFunction::new(name, true)
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn namespace(&self) -> &str { &self.namespace }
    pub fn argument_key(&self) -> &str { &self.argument_key }
    pub fn allow_method_syntax(&self) -> bool { self.allow_method_syntax }
}

pub struct TwoArgsFunction {
    pub name: String,
    namespace: Cow<'static, str>,
    argument1_key: Cow<'static, str>,
    argument2_key: Cow<'static, str>,
    pub allow_method_syntax: bool,
}

impl TwoArgsFunction {
    pub fn new_with_config(name: String, namespace: String, argument1_key: String, argument2_key: String, allow_method_syntax: bool) -> TwoArgsFunction {
        TwoArgsFunction {
            name,
            namespace: Cow::Owned(namespace),
            argument1_key: Cow::Owned(argument1_key),
            argument2_key: Cow::Owned(argument2_key),
            allow_method_syntax,
        }
    }

    pub fn new(name: String, allow_method_syntax: bool) -> TwoArgsFunction {
        TwoArgsFunction {
            name,
            namespace: Cow::Borrowed("minecraft"),
            argument1_key: Cow::Borrowed("argument1"),
            argument2_key: Cow::Borrowed("argument2"),
            allow_method_syntax,
        }
    }

    pub fn new_without_method_syntax(name: String) -> TwoArgsFunction {
        TwoArgsFunction::new(name, false) // Functions with two arguments can't be used as methods by default
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn namespace(&self) -> &str { &self.namespace }
    pub fn argument1_key(&self) -> &str { &self.argument1_key }
    pub fn argument2_key(&self) -> &str { &self.argument2_key }
    pub fn allow_method_syntax(&self) -> bool { self.allow_method_syntax }
}

pub struct CustomThreeArgsFunction {
    pub name: String,
    pub allow_method_syntax: bool,
    function: Box<dyn Fn(Element, Element, Element) -> ProcessResult>,
}

impl CustomThreeArgsFunction {
    pub fn new_with_allow_method_syntax(name: String, allow_method_syntax: bool, function: Box<dyn Fn(Element, Element, Element) -> ProcessResult>) -> CustomThreeArgsFunction {
        CustomThreeArgsFunction {
            name, allow_method_syntax, function
        }
    }

    pub fn new(name: String, function: Box<dyn Fn(Element, Element, Element) -> ProcessResult>) -> CustomThreeArgsFunction {
        CustomThreeArgsFunction::new_with_allow_method_syntax(name, false, function)
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn allow_method_syntax(&self) -> bool { self.allow_method_syntax }
    pub fn function(&self) -> &Box<dyn Fn(Element, Element, Element) -> ProcessResult> { &self.function }
}

pub struct BinaryOperator {
    pub name: String,
    namespace: Cow<'static, str>,
    internal_name: String,
    argument1_key: Cow<'static, str>,
    argument2_key: Cow<'static, str>,
}

impl BinaryOperator {
    pub fn new_with_config(name: String, namespace: String, internal_name: String, argument1_key: String, argument2_key: String) -> BinaryOperator {
        BinaryOperator {
            name,
            namespace: Cow::Owned(namespace),
            internal_name,
            argument1_key: Cow::Owned(argument1_key),
            argument2_key: Cow::Owned(argument2_key),
        }
    }

    pub fn new(name: String, internal_name: String) -> BinaryOperator {
        BinaryOperator {
            name,
            namespace: Cow::Borrowed("minecraft"),
            internal_name,
            argument1_key: Cow::Borrowed("argument1"),
            argument2_key: Cow::Borrowed("argument2"),
        }
    }

    pub fn name(&self) -> &str { &self.name }
    pub fn namespace(&self) -> &str { &self.namespace }
    pub fn internal_name(&self) -> &str { &self.internal_name }
    pub fn argument1_key(&self) -> &str { &self.argument1_key }
    pub fn argument2_key(&self) -> &str { &self.argument2_key }
}

pub struct ElementProcessor<'a> {
    preprocessors: Vec<Box<dyn Fn(Element) -> ProcessResult + 'a>>,
    postprocessors: Vec<Box<dyn Fn(Element) -> ProcessResult + 'a>>,

    no_arg_functions: Vec<NoArgFunction>,
    one_arg_functions: Vec<OneArgFunction>,
    two_args_functions: Vec<TwoArgsFunction>,
    custom_three_args_functions: Vec<CustomThreeArgsFunction>,
    binary_operators: Vec<BinaryOperator>, // Will ignore allow_method_syntax
}

impl<'a> ElementProcessor<'a> {
    pub fn new() -> ElementProcessor<'a> {
        ElementProcessor {
            preprocessors: Vec::new(),
            postprocessors: Vec::new(),

            no_arg_functions: Vec::new(), one_arg_functions: Vec::new(), two_args_functions: Vec::new(),
            custom_three_args_functions: Vec::new(),
            binary_operators: Vec::new(),
        }
    }

    pub fn add_preprocessor<F: 'a>(&mut self, preprocessor: F) where F: Fn(Element) -> ProcessResult {
        self.preprocessors.push(Box::new(preprocessor));
    }

    pub fn add_postprocessor<F: 'a>(&mut self, postprocessor: F) where F: Fn(Element) -> ProcessResult {
        self.postprocessors.push(Box::new(postprocessor));
    }

    pub fn add_no_arg_function(&mut self, function: NoArgFunction) {
        self.no_arg_functions.push(function);
    }

    pub fn add_one_arg_function(&mut self, function: OneArgFunction) {
        self.one_arg_functions.push(function);
    }

    pub fn add_two_args_function(&mut self, function: TwoArgsFunction) {
        self.two_args_functions.push(function);
    }

    pub fn add_custom_three_args_function(&mut self, function: CustomThreeArgsFunction) {
        self.custom_three_args_functions.push(function);
    }

    pub fn add_binary_operator(&mut self, function: BinaryOperator) {
        self.binary_operators.push(function);
    }

    pub fn process(&self, element: Element) -> ProcessResult {
        // let mut pre_result = ElementProcessor::apply_processors(element, &self.preprocessors);
        let mut pre_result = self.pre_process(element);

        if let Some(element) = pre_result.take_element() {
            let result = match element {
                Element::ArrayElement(values) => {
                    let mut result = ProcessResult::empty();
                    let mut result_values = Vec::new();

                    for mut sub_result in values.into_iter().map(|element| self.process(element)) {
                        result.append_warnings_and_errors(&mut sub_result);

                        if let None = sub_result.element() {
                            return result;
                        }

                        result_values.push(sub_result.take_element().unwrap());
                    }

                    result.with_element(Some(Element::ArrayElement(result_values)))
                }
                Element::ObjectElement(fields) => {
                    let mut result = ProcessResult::empty();
                    let mut result_fields = Vec::new();

                    for (key, value) in fields {
                        let mut key_result = self.process(key);
                        let mut value_result = self.process(value);

                        result.append_warnings_and_errors(&mut key_result);
                        result.append_warnings_and_errors(&mut value_result);

                        if key_result.element().is_none() || value_result.element().is_none() {
                            return result;
                        }

                        result_fields.push((key_result.take_element().unwrap(), value_result.take_element().unwrap()));
                    }

                    result.with_element(Some(Element::ObjectElement(result_fields)))
                }
                Element::BinaryElement { left, operator, right } => {
                    let mut result = ProcessResult::empty();

                    let mut left_result = self.process(*left);
                    let mut right_result = self.process(*right);

                    result.append_warnings_and_errors(&mut left_result);
                    result.append_warnings_and_errors(&mut right_result);

                    if left_result.element().is_none() || right_result.element().is_none() {
                        return result;
                    }

                    result.with_element(Some(Element::BinaryElement {
                        left: Box::new(left_result.take_element().unwrap()),
                        operator,
                        right: Box::new(right_result.take_element().unwrap()),
                    }))
                }
                Element::FunctionCallElement { receiver, name, arguments } => {
                    let mut result = ProcessResult::empty();

                    let mut receiver_result = if let Some(receiver) = receiver {
                        let mut receiver_result = self.process(*receiver);
                        result.append_warnings_and_errors(&mut receiver_result);
                        receiver_result
                    } else {
                        ProcessResult::empty()
                    };

                    let arguments_result = if let Some(arguments) = arguments {
                        let mut result_arguments = Vec::new();

                        for mut argument_result in arguments.into_iter().map(|element| self.process(element)) {
                            result.append_warnings_and_errors(&mut argument_result);

                            if let None = argument_result.element() {
                                return result;
                            }

                            result_arguments.push(argument_result.take_element().unwrap());
                        }

                        Some(result_arguments)
                    } else {
                        None
                    };

                    result.with_element(Some(Element::FunctionCallElement {
                        receiver: receiver_result.take_element().map(Box::new),
                        name,
                        arguments: arguments_result,
                    }))
                }

                _ => ProcessResult::from_element(element),
            };

            result.flat_map(|element| ElementProcessor::apply_processors(element, &self.postprocessors))
        } else {
            pre_result
        }
    }

    fn pre_process(&self, element: Element) -> ProcessResult {
        match element {
            Element::NameElement(name) => {
                for function in &self.no_arg_functions {
                    if function.allow_as_symbol() && function.name() == name {
                        return ProcessResult::from_element(object_element!(
                                string_element!("type") => Element::StringElement(format!("{}:{}", function.namespace(), function.name()))
                        ));
                    }
                }

                ProcessResult::from_element(Element::NameElement(name))
            },
            Element::BinaryElement { left, operator, right } => {
                for function in &self.binary_operators {
                    if function.name() == operator.text() {
                        return ProcessResult::from_element(object_element!(
                            string_element!("type") => Element::StringElement(format!("{}:{}", function.namespace(), function.internal_name())),
                            Element::StringElement(function.argument1_key().to_owned()) => *left,
                            Element::StringElement(function.argument2_key().to_owned()) => *right
                        ));
                    }
                }

                ProcessResult::from_element(Element::BinaryElement { left, operator, right })
            }
            Element::FunctionCallElement { receiver: Some(receiver), name, arguments: Some(mut args) } => {
                if args.len() == 0 {
                    for function in &self.one_arg_functions {
                        if function.allow_method_syntax() && function.name() == name {
                            return ProcessResult::from_element(object_element!(
                                string_element!("type") => Element::StringElement(format!("{}:{}", function.namespace(), function.name())),
                                Element::StringElement(function.argument_key().to_owned()) => *receiver
                            ));
                        }
                    }
                } else if args.len() == 1 {
                    for function in &self.two_args_functions {
                        if function.allow_method_syntax() && function.name() == name {
                            return ProcessResult::from_element(object_element!(
                                string_element!("type") => Element::StringElement(format!("{}:{}", function.namespace(), function.name())),
                                Element::StringElement(function.argument1_key().to_owned()) => *receiver,
                                Element::StringElement(function.argument2_key().to_owned()) => args.swap_remove(0)
                            ));
                        }
                    }
                } else if args.len() == 2 {
                    for function in &self.custom_three_args_functions {
                        if function.allow_method_syntax() && function.name() == name {
                            return function.function()(*receiver, args.swap_remove(0), args.swap_remove(0));
                        }
                    }
                }

                ProcessResult::from_element(Element::FunctionCallElement { receiver: Some(receiver), name, arguments: Some(args) })
            },
            Element::FunctionCallElement { receiver: None, name, arguments: Some(mut args) } => {
                if args.len() == 0 {
                    for function in &self.no_arg_functions {
                        if function.name() == name {
                            return ProcessResult::from_element(object_element!(
                                string_element!("type") => Element::StringElement(format!("{}:{}", function.namespace(), function.name()))
                            ));
                        }
                    }
                } else if args.len() == 1 {
                    for function in &self.one_arg_functions {
                        if function.name() == name {
                            return ProcessResult::from_element(object_element!(
                                string_element!("type") => Element::StringElement(format!("{}:{}", function.namespace(), function.name())),
                                Element::StringElement(function.argument_key().to_owned()) => args.swap_remove(0)
                            ));
                        }
                    }
                } else if args.len() == 2 {
                    for function in &self.two_args_functions {
                        if function.name() == name {
                            return ProcessResult::from_element(object_element!(
                                string_element!("type") => Element::StringElement(format!("{}:{}", function.namespace(), function.name())),
                                Element::StringElement(function.argument1_key().to_owned()) => args.swap_remove(0),
                                Element::StringElement(function.argument2_key().to_owned()) => args.swap_remove(0)
                            ));
                        }
                    }
                } else if args.len() == 3 {
                    for function in &self.custom_three_args_functions {
                        if function.name() == name {
                            return function.function()(args.swap_remove(0), args.swap_remove(0), args.swap_remove(0));
                        }
                    }
                }

                ProcessResult::from_element(Element::FunctionCallElement { receiver: None, name, arguments: Some(args) })
            }
            element => ProcessResult::from_element(element),
        }
    }

    fn apply_processors<'b>(element: Element, processors: &Vec<Box<dyn Fn(Element) -> ProcessResult + 'b>>) -> ProcessResult {
        let mut result = ProcessResult::from_element(element);

        for processor in processors {
            result = result.flat_map(processor);

            if let None = result.element() {
                return result;
            }
        }

        result
    }
}
