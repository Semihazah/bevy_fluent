use bevy::{ecs::system::SystemParam, prelude::*};
use fluent_langneg::{negotiate_languages, NegotiationStrategy};
use std::{iter::once, ops::Deref, path::Path, slice::from_ref};
use unic_langid::{langid, LanguageIdentifier};

/// Rotor
pub trait Rotor {
    fn rotate_left(&mut self, n: usize) -> &mut Self;

    fn rotate_right(&mut self, n: usize) -> &mut Self;
}

/// Locale bundle
#[derive(SystemParam)]
pub struct LocaleBundle<'a> {
    pub locales: ResMut<'a, Locales>,
    pub default: Option<Res<'a, DefaultLocale>>,
}

impl<'a> LocaleBundle<'a> {
    pub fn pack<F>(&'a self, f: F) -> impl 'a + Iterator<Item = &'static Path>
    where
        F: 'a + Fn(Option<&LanguageIdentifier>) -> &'static Path,
    {
        self.locales
            .fallback_chain(self.default.as_deref())
            .into_iter()
            .map(Some)
            .chain(once(None))
            .map(f)
    }
}

impl Rotor for LocaleBundle<'_> {
    fn rotate_left(&mut self, n: usize) -> &mut Self {
        self.locales.rotate_left(n);
        self
    }

    fn rotate_right(&mut self, n: usize) -> &mut Self {
        self.locales.rotate_right(n);
        self
    }
}

/// Default locale
#[derive(Clone, Debug, Default)]
pub struct DefaultLocale(pub LanguageIdentifier);

impl Deref for DefaultLocale {
    type Target = LanguageIdentifier;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Locales
#[derive(Clone, Debug, Default)]
pub struct Locales(Vec<LanguageIdentifier>);

impl Locales {
    pub fn new(locales: Vec<LanguageIdentifier>) -> Self {
        assert!(!locales.is_empty());
        Self(locales)
    }

    pub fn locale(&self) -> &LanguageIdentifier {
        &self[0]
    }

    pub fn fallback_chain<'a>(
        &'a self,
        default: Option<&'a DefaultLocale>,
    ) -> Vec<&'a LanguageIdentifier> {
        let available = &*self;
        let default = default.map(Deref::deref);
        let requested = from_ref(&self[0]);
        negotiate_languages(
            requested,
            available,
            default,
            NegotiationStrategy::Filtering,
        )
    }
}

impl Rotor for Locales {
    fn rotate_left(&mut self, n: usize) -> &mut Self {
        let len = self.0.len();
        self.0.rotate_left(n % len);
        self
    }

    fn rotate_right(&mut self, n: usize) -> &mut Self {
        let len = self.0.len();
        self.0.rotate_right(n % len);
        self
    }
}

impl Deref for Locales {
    type Target = [LanguageIdentifier];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub mod converters {
    use super::*;

    pub fn menu(locale: Option<&LanguageIdentifier>) -> &'static Path {
        match locale {
            Some(&de::DE) => Path::new("locales/de-DE/menu.ron"),
            Some(&en::US) => Path::new("locales/en-US/menu.ron"),
            Some(&ru::BY) => Path::new("locales/ru/ru-BY/menu.ron"),
            Some(&ru::RU) => Path::new("locales/ru/ru-RU/menu.ron"),
            None => Path::new("locales/interlocale/menu.ron"),
            _ => unimplemented!(),
        }
    }
}

pub mod de {
    use super::*;

    pub const DE: LanguageIdentifier = langid!("de-DE");
}

pub mod en {
    use super::*;

    pub const US: LanguageIdentifier = langid!("en-US");
}

pub mod ru {
    use super::*;

    pub const BY: LanguageIdentifier = langid!("ru-BY");
    pub const RU: LanguageIdentifier = langid!("ru-RU");
}
