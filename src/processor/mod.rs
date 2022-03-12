mod result;

pub use result::*;

use crate::element::Element;

pub struct ElementProcessor<'a> {
    preprocessors: Vec<Box<dyn Fn(Element) -> ProcessResult + 'a>>,
    postprocessors: Vec<Box<dyn Fn(Element) -> ProcessResult + 'a>>,
}

impl<'a> ElementProcessor<'a> {
    pub fn new() -> ElementProcessor<'a> {
        ElementProcessor {
            preprocessors: Vec::new(), postprocessors: Vec::new(),
        }
    }

    pub fn add_preprocessor<F: 'a>(&mut self, preprocessor: F) where F: Fn(Element) -> ProcessResult {
        self.preprocessors.push(Box::new(preprocessor));
    }

    pub fn add_postprocessor<F: 'a>(&mut self, postprocessor: F) where F: Fn(Element) -> ProcessResult {
        self.postprocessors.push(Box::new(postprocessor));
    }

    pub fn process(&self, element: Element) -> ProcessResult {
        let mut pre_result = ElementProcessor::apply_processors(element, &self.preprocessors);

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
                },
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
                },
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
                },
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
                        arguments: arguments_result
                    }))
                },

                _ => ProcessResult::from_element(element),
            };

            result.flat_map(|element| ElementProcessor::apply_processors(element, &self.postprocessors))
        } else {
            pre_result
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
