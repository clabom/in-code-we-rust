use std::num::ParseFloatError;
use std::str::FromStr;
use std::convert::Infallible;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ParseBookError {
    Comment,
    InvalidFormat,
}

impl From<ParseFloatError> for ParseBookError {
    fn from(_: ParseFloatError) -> Self {
        Self::InvalidFormat
    }
}

impl From<Infallible> for ParseBookError {
    fn from(_: Infallible) -> Self {
        unreachable!()
    }
}

#[derive(Debug, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub price: f32,
    pub description: Option<String>,
}

impl FromStr for Book {
    type Err = ParseBookError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(';').map(str::trim).collect();

        if parts.len() < 3 || parts.len() > 4 {
            return  Err(ParseBookError::InvalidFormat);
        }

        let title = parts[0].parse::<String>()?;
        let author = parts[1].parse::<String>()?;
        if author.len() == 0 {
            return  Err(ParseBookError::InvalidFormat);
        }

        if title.starts_with("//") {
            return  Err(ParseBookError::Comment);
        }

        let price = parts[2].parse::<f32>()?;

        let description: Option<String> = match parts.get(3) {
            Some(desc) => Some(desc.parse::<String>()?),
            None => None,
        };
        Ok(Book { title: title, author:author, price: price, description: description })
    }
}

impl fmt::Display for Book {    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Title: {}, Author: {}, Price:{}",
               self.title, self.author, self.price)?;
        match &self.description {
            Some(desc) => write!(f, ", Description: {}", desc)?,
            None => write!(f, "")?,
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn book() {
        let line = "Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall; 35.96";
        assert_eq!(
            line.parse::<Book>(),
            Ok(Book {
                title: "Bussysteme in der Fahrzeugtechnik".to_string(),
                author: "Werner Zimmermann, Ralf Schmidgall".to_string(),
                price: 35.96,
                description: None
            })
        )
    }

    #[test]
    fn book_with_description() {
        let line = "Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall; 35.96; Erstes deutschsprachiges Buch zur Informationsverarbeitung im Auto, jetzt mit AUTOSAR 3.x";
        assert_eq!(
            line.parse::<Book>(),
            Ok(Book {
                title: "Bussysteme in der Fahrzeugtechnik".to_string(),
                author: "Werner Zimmermann, Ralf Schmidgall".to_string(),
                price: 35.96,
                description: Some("Erstes deutschsprachiges Buch zur Informationsverarbeitung im Auto, jetzt mit AUTOSAR 3.x".to_string())
            })
        )
    }

    #[test]
    fn comment() {
        let line =
            "// Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall; 35.96";
        assert_eq!(line.parse::<Book>(), Err(ParseBookError::Comment))
    }

    #[test]
    fn missing_author() {
        let line = "Bussysteme in der Fahrzeugtechnik; ; 35.96";
        assert_eq!(line.parse::<Book>(), Err(ParseBookError::InvalidFormat))
    }

    #[test]
    fn too_short() {
        let line = "Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall";
        assert_eq!(line.parse::<Book>(), Err(ParseBookError::InvalidFormat))
    }

    #[test]
    fn too_long() {
        let line = "Bussysteme in der Fahrzeugtechnik; Werner Zimmermann, Ralf Schmidgall; 35.96; Erstes deutschsprachiges Buch zur Informationsverarbeitung im Auto, jetzt mit AUTOSAR 3.x; Springer";
        assert_eq!(line.parse::<Book>(), Err(ParseBookError::InvalidFormat))
    }
}
