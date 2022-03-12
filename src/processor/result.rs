use std::fmt::{Display, Formatter};

use crate::element::Element;

#[derive(Clone, Debug)]
pub enum ProcessWarning {
}

impl Display for ProcessWarning {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO(ProcessWarning)")
    }
}

#[derive(Clone, Debug)]
pub enum ProcessError {
}

impl Display for ProcessError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO(ProcessError)")
    }
}

#[derive(Clone, Debug)]
pub struct ProcessResult {
    pub element: Option<Element>,
    pub warnings: Vec<ProcessWarning>,
    pub errors: Vec<ProcessError>,
}

impl ProcessResult {
    pub fn new(element: Element, warnings: Vec<ProcessWarning>, errors: Vec<ProcessError>) -> ProcessResult {
        ProcessResult {
            element: Some(element), warnings, errors
        }
    }

    pub fn from_element(element: Element) -> ProcessResult {
        ProcessResult::new(element, Vec::new(), Vec::new())
    }

    pub fn empty() -> ProcessResult {
        ProcessResult {
            element: None,
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    // Getters; in case the fields will be made private in the future
    pub fn element(&self) -> Option<&Element> { self.element.as_ref() }
    pub fn take_element(&mut self) -> Option<Element> { self.element.take() }

    pub fn warnings(&self) -> &Vec<ProcessWarning> { &self.warnings }
    pub fn errors(&self) -> &Vec<ProcessError> { &self.errors }

    pub fn with_warning(&mut self, warning: ProcessWarning) -> &mut Self {
        self.warnings.push(warning);
        self
    }

    pub fn with_warnings(&mut self, warnings: &mut Vec<ProcessWarning>) -> &mut Self { // Moves all warnings into self (leaves warnings empty)
        self.warnings.append(warnings);
        self
    }

    pub fn with_error(&mut self, error: ProcessError) -> &mut Self {
        self.errors.push(error);
        self
    }

    pub fn with_errors(&mut self, errors: &mut Vec<ProcessError>) -> &mut Self {
        self.errors.append(errors);
        self
    }

    pub fn with_element(mut self, element: Option<Element>) -> Self {
        self.element = element;
        self
    }

    pub fn map<F: FnOnce(Element) -> Element>(mut self, f: F) -> Self {
        if let Some(element) = self.element.take() {
            self.element = Some(f(element));
            self
        } else {
            self
        }
    }

    pub fn append_warnings_and_errors(&mut self, other: &mut Self) -> &mut Self {
        self.with_warnings(&mut other.warnings);
        self.with_errors(&mut other.errors);
        self
    }

    pub fn flat_map<F: FnOnce(Element) -> ProcessResult>(mut self, f: F) -> Self {
        if let Some(element) = self.element.take() {
            let mut result = f(element);

            self.append_warnings_and_errors(&mut result);
            self.element = result.element;
            self
        } else {
            self
        }
    }
}

impl Display for ProcessResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TODO(ProcessResult)")
    }
}
