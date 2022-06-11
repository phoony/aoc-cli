use thiserror::Error;

pub struct Replacement {
    from: String,
    to: String,
}

impl Replacement {
    pub fn new(from: &str, to: &str) -> Self {
        Self {
            from: from.to_string(),
            to: to.to_string(),
        }
    }
}

macro_rules! rep {
    ($from:literal, $to:literal) => {
        Replacement::new($from, $to)
    };
}

#[derive(Error, Debug)]
pub enum TemplateError {
    #[error("'{0}' is not a valid replacement")]
    PatternNotRecognized(String),
}

pub trait TemplateProcessing {
    fn process_template(self, replacements: &[Replacement]) -> Result<String, TemplateError>;
}

impl TemplateProcessing for &str {
    fn process_template(self, replacements: &[Replacement]) -> Result<String, TemplateError> {
        let template = Template::from(self);
        template.process(replacements)
    }
}

struct Template<'a> {
    rest: &'a str,
}

impl<'a> From<&'a str> for Template<'a> {
    fn from(s: &'a str) -> Self {
        Self { rest: s }
    }
}

impl Template<'_> {
    fn process(mut self, replacements: &[Replacement]) -> Result<String, TemplateError> {
        let mut result = String::with_capacity(self.rest.len());

        // while the rest of the template is not empty
        while !self.rest.is_empty() {
            // process arbitrary (non template) text
            result.push_str(self.process_non_template());

            // see if we found a pattern
            if let Some(pattern) = self.try_read_template() {
                // trim to allow {{ pattern }} or similar formatting
                let pattern = pattern.trim();

                // check if the pattern is in our replacement list
                if let Some(rep) = replacements.iter().find(|rep| rep.from == pattern) {
                    result.push_str(&rep.to)
                } else {
                    return Err(TemplateError::PatternNotRecognized(pattern.to_string()));
                }
            }
        }

        Ok(result)
    }

    fn process_non_template(&mut self) -> &str {
        // consume everything until a template is found
        if let Some(index) = self.rest.find("{{") {
            let consumed = &self.rest[..index];
            self.rest = &self.rest[index..];
            return consumed;
        }

        // if there was no template found, consume the rest
        let consumed = self.rest;
        self.rest = "";

        consumed
    }

    fn try_read_template(&mut self) -> Option<&str> {
        // if we found a template start
        if self.rest.starts_with("{{") {
            // try to find the end
            let r_index = self.rest.find("}}")?;
            let temp = self.rest;

            // advance pointer state
            self.rest = &self.rest[r_index + 2..];

            // return the contained template string
            return Some(&temp[2..r_index]);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("", &[], "" ; "empty strings")]
    #[test_case("{{foo}}", &[rep!("foo", "bar")], "bar" ; "simple replacement")]
    #[test_case("{{ foo }}", &[rep!("foo", "bar")], "bar" ; "outer spaces")]
    #[test_case("{{ f o o }}", &[rep!("f o o", "bar")], "bar" ; "spaces inside of pattern")]
    #[test_case("abc {{foo}}", &[rep!("foo", "bar")], "abc bar" ; "text pattern")]
    #[test_case("{{foo}} abc", &[rep!("foo", "bar")], "bar abc" ; "pattern text")]
    #[test_case("abc {{foo}} abc", &[rep!("foo", "bar")], "abc bar abc" ; "text pattern text")]
    #[test_case("{abc} {{foo}} def {{ bar }}", &[rep!("foo", "bar"), rep!("bar", "foo")], "{abc} bar def foo" ; "complex")]
    fn templating(input: &str, replacements: &[Replacement], expected: &str) {
        let t = Template { rest: input };
        let result = t.process(replacements).unwrap();

        assert_eq!(result, expected)
    }
}
