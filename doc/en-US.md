# Documentation

[🔼](../README.md) | **English** 🇺🇸 | [Русский 🇷🇺](ru-RU.md)

## Use

## Settings

Settings are set using the `Settings` resource. It provides the following
options:

- `default_locale` - the locale to use as the default in your application,
- `fallback_locale_chain` - the fallback locale chain you want to use in your
  application,
- `locales_folder` - the *root locales folder*.

## Definitions

***Root locales directory*** - is the root directory for all locales. By default
it is `assets/locales/`.

***Locale root*** is a directory or file for a specific locale. *Locale root*
which is a directory, is also called ***Locale directory***.

Fluent [***asset***][asset] or simply *asset* - is any file that matches the
pattern `*.ftl`. *Asset* is the atomic unit of disk storage for Fluent.

[***Snapshot***][snapshot] is a collection of locale-associated *assets* for the
current plugin settings.

[***Localization***][localization] is a compiled *snapshot*. Effectively it is a
collection of Fluent [***bundles***][fluent-bundle].

Each *asset* is a set of [***messages***][message]. *Message* is the basic
atomic translation unit for Fluent.

Each *message* has an [***identifier***][identifier].

*Messages* (and [***terms***][term], [***variants***][variant],
[***attributes***][attribute]) store their values as [***patterns***][pattern].

Formated *pattern* are called [***content***][content].

[***Request***][request] provides access to *content* according to the given
components.

[***Request***][request] is a request to receive *content* specified by the
parameters.

## The *root locales directory* structure

Directory or file at depth 0 is a *locale root* if its name matches the
[standard][unicode_language_identifier]. Directories or files at depth 0 that do
not meeting the specified standard belong to interlocals. A directory or file
deeper than 0 is a *locale root* if the parent directory is a *locale directory*
and the parent directory's locale is a superset of its locale. *Assets* located
in the hierarchy of the corresponding *locale directory* belong to that locale.

Example:

```md
locales
    - de
        ...
    - en-US
        ...
    - ru
        - ru-RU
            ...
        - ru-BY
            ...
```

[asset]: https://docs.rs/bevy_fluent/*/bevy_fluent/struct.FluentAsset.html
[attribute]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Attribute.html
[content]: https://docs.rs/bevy_fluent/*/bevy_fluent/utils/trait.BundleExt.html#tymethod.content
[explicit]: https://docs.rs/crate/bevy_fluent/*/features#implicit
[fluent-bundle]: https://docs.rs/fluent/*/fluent/bundle/struct.FluentBundle.html
[identifier]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Identifier.html
[implicit]: https://docs.rs/crate/bevy_fluent/*/features#implicit
[localization]: https://docs.rs/bevy_fluent/*/bevy_fluent/struct.Localization.html
[message]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Message.html
[pattern]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Pattern.html
[request]: https://docs.rs/bevy_fluent/*/bevy_fluent/struct.Request.html
[snapshot]: https://docs.rs/bevy_fluent/*/bevy_fluent/struct.Snapshot.html
[term]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Term.html
[variant]: https://docs.rs/fluent-syntax/*/fluent_syntax/ast/struct.Variant.html

[unicode-language-identifier]: http://unicode.org/reports/tr35/#Unicode_language_identifier
