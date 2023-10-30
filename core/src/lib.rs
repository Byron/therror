use std::{error::Error, fmt};

#[derive(Debug)]
pub struct TheError {
    error_identifier: Box<[&'static str]>,
    error: Box<dyn Error + Send + Sync>,
}

impl TheError {
    pub fn new(error_identifier: Box<[&'static str]>, error: Box<dyn Error + Send + Sync>) -> Self {
        Self {
            error_identifier,
            error,
        }
    }

    /// Wrap the existing error with a new layer name.
    ///
    /// E.g., if `self.get_error_identifier()` returns `["A", "B"]` and this
    /// function is called with `layer_name` equals to `C`, then the new error
    /// would have `self.get_error_identifier()` equals to `["C", "A", "B"]`.
    pub fn wrap(self, layer_name: &'static str) -> Self {
        let mut error_identifier = Vec::with_capacity(self.error_identifier.len() + 1);
        error_identifier.push(layer_name);
        error_identifier.extend(&*self.error_identifier);
        Self {
            error_identifier: error_identifier.into_boxed_slice(),
            error: self.error,
        }
    }

    /// Return the identifier for the error in format `Variant1::Variant2`
    pub fn get_error_identifier(&self) -> &[&'static str] {
        &self.error_identifier
    }

    /// Downcast to `T`.
    pub fn downcast<T>(self) -> Result<T, Self>
    where
        T: Error + Send + Sync + 'static,
    {
        let Self {
            error_identifier,
            error,
        } = self;
        error
            .downcast::<T>()
            .map(|boxed| *boxed)
            .map_err(|error| Self {
                error_identifier,
                error,
            })
    }

    /// Downcast to `&mut T`.
    pub fn downcast_mut<T>(&mut self) -> Option<&mut T>
    where
        T: Error + Send + Sync + 'static,
    {
        self.error.downcast_mut::<T>()
    }

    /// Downcast to `&T`.
    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Error + Send + Sync + 'static,
    {
        self.error.downcast_ref::<T>()
    }

    /// Returns `true` if it contains `T`.
    pub fn is<T>(&self) -> bool
    where
        T: Error + Send + Sync + 'static,
    {
        self.error.is::<T>()
    }
}

impl fmt::Display for TheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let last_error_identifier_index = self.error_identifier.len() - 1;
        for (i, ident) in self.error_identifier.iter().enumerate() {
            f.write_str(ident)?;
            if i < last_error_identifier_index {
                f.write_str( "::")?;
            }
        }

        write!(f, "(`{}`)", self.error)
    }
}

impl Error for TheError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&*self.error)
    }
}
