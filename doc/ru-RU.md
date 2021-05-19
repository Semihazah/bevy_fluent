# Документация

[🔼](../README.md) | [English 🇺🇸](en-US.md) | **Русский** 🇷🇺

## Использование

## Настройки

Настройки задаются с помощью ресурса `Settings`. Он предоставляет следующие
опции:

- `default_locale` - локаль которая будет использоваться в вашем приложении по
  умолчанию,
- `fallback_locale_chain` - цепочка резервных локалей, которую вы хотите
  использовать в своем приложении,
- `locales_folder` - *корневая директория локалей*.

## Определения

***Корневая директория локалей*** - это корневая директория для всех локалей. По
умолчанию - это `assets/locales/`.

***Корень локали*** - это директория или файл конкретной локали. *Корень
локали*, являющийся директорией, по-другому называется ***директорией локали***.

[***Aссет***][asset] Fluent или просто *ассет* - это любой файл, соответствующий
шаблону `*.ftl`. *Ассет* является атомарной единицей хранения информации на
диске для Fluent.

[***Снепшот***][snapshot] - это это коллекция ассоциированных с локалями
*ассетов* для текущих настроек плагина.

[***Локализация***][localization] - это скомпилированный *снепшот*. По сути -
это коллекция [***бандлов***][fluent-bundle] Fluent.

Каждый *ассет* представляет собой набор [***сообщений***][message]. *Cообщение*
является атомарной единицей перевода во Fluent.

Каждое *сообщение* имеет [***идентификатор***][identifier].

*Сообщения* (как и [***термы***][term], [***варианты***][variant],
[***аттрибуты***][attribute]) хранят свои значения в виде
[***паттернов***][pattern].

Форматированный *паттерн* называется [***контентом***][content].

[***Запрос***][request] представляет собой запрос на получение соответствующего
заданным параметрам *контента*.

## Структура *корневой директории локалей*

Директория или файл на глубине 0 является *корнем локали*, если ее/его имя
соответствует [стандарту][unicode_language_identifier]. Директории или файлы на
глубине 0, не соответствующие указанному стандарту принадлежат интерлокали.
Директория или файл на глубине больше 0 является *корнем локали*, если
родительская директория является *директорией локали*, а локаль родительской
директории является надмножеством ее/его локали. *Ассеты*, расположенные в
иерархии соответствующей *директории локали* относятся к этой локали.

Пример:

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
