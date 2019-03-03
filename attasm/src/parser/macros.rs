macro_rules! check_complete {
    ($pair:ident, $input:ident) => {
        if $pair.as_span().end() < $input.len() {
            let variant = $crate::pest::error::ErrorVariant::CustomError {
                message: String::from("unexpected remaining input")
            };
            let span = $crate::pest::Span::new(
                $input, $pair.as_span().end(), $input.len()
            );
            let error = $crate::pest::error::Error::new_from_span(
                variant, span.unwrap()
            );
            return Err(error);
        }
    }
}
