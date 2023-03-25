# F1-Docs-Types

This is a rust type library for F1 Document types.
This is currently WIP and will eventually replace both the
[FIA-Docs-Discord Bot](https://github.com/MarkusTheOrt/FIA-Docs-Discord-Bot)
and the [FIA Mongo Docs](https://github.com/MarkusTheOrt/FIA-Mongo-Docs)
repositories.

## Types

This Library defines some basic types for which will be shared for both
scraper and bot.

 - `Event`: Describes a F1 Race Weekend (Name and Year)
 - `Document`: Describes a FIA Document for a given event.
 - `DiscordGuild`: Describes a Discord Server with some required data.
 - `Thread`: Describes a Discord Thread for a guild and event.

This Library uses Serde, Chrono, and Mongodb as dependencies.
